import { getDocument, PDFWorker } from "./pdfjs/build/pdf.mjs";
import { EventBus, PDFViewer, PDFLinkService, PDFFindController } from "./pdfjs/web/pdf_viewer.mjs";
import { createPdfReloader } from "./pdf-reloader.mjs";

export async function start(vscode) {
const $ = (id) => document.getElementById(id);
// VS Code webview workers cannot import a vscode-resource module. Fetch the
// self-contained worker on the main thread and start it from a blob instead.
const workerResponse = await fetch(new URL("./pdfjs/build/pdf.worker.mjs", import.meta.url));
if (!workerResponse.ok) { throw new Error(`Could not load PDF worker: ${workerResponse.status}`); }
const workerUrl = URL.createObjectURL(new Blob([await workerResponse.text()], { type: "text/javascript" }));
const workerPort = new Worker(workerUrl, { type: "module" });
const worker = new PDFWorker({ port: workerPort });
window.addEventListener("unload", () => { worker.destroy(); workerPort.terminate(); URL.revokeObjectURL(workerUrl); });
const eventBus = new EventBus();
const linkService = new PDFLinkService({ eventBus, externalLinkTarget: 2, externalLinkRel: "noopener noreferrer" });
const findController = new PDFFindController({ eventBus, linkService });
const viewer = new PDFViewer({ container: $("container"), viewer: $("viewer"), eventBus, linkService, findController });
linkService.setViewer(viewer);
let loaded = false;
let pending;
let restore = vscode.getState() ?? { page: 1, scale: "page-width", top: 0, left: 0 };

function sync(point) {
  if (!loaded) { pending = point; return; }
  const page = viewer.getPageView(point.page - 1);
  if (!page?.viewport) { return; }
  // SyncTeX uses unrotated PDF points from the top left.
  const box = page.viewport.viewBox;
  const [x, y] = page.viewport.convertToViewportPoint(box[0] + point.x, box[3] - point.y);
  viewer.currentPageNumber = point.page;
  $("container").scrollTo({ top: page.div.offsetTop + y - $("container").clientHeight / 3, left: Math.max(0, page.div.offsetLeft + x - $("container").clientWidth / 2) });
  const marker = document.createElement("div");
  marker.className = "sync-marker";
  marker.style.left = `${x}px`;
  marker.style.top = `${y}px`;
  page.div.append(marker);
  setTimeout(() => marker.remove(), 1800);
}

eventBus.on("pagesinit", () => {
  loaded = true;
  viewer.currentScaleValue = restore.scale;
  viewer.currentPageNumber = Math.min(restore.page, viewer.pagesCount);
  $("page").max = String(viewer.pagesCount);
  $("count").textContent = `/ ${viewer.pagesCount}`;
  $("status").textContent = "Double-click to jump to source";
  requestAnimationFrame(() => {
    $("container").scrollTop = restore.top;
    $("container").scrollLeft = restore.left;
    if (pending) { sync(pending); pending = undefined; }
    vscode.postMessage({ type: "loaded" });
  });
});
eventBus.on("pagechanging", ({ pageNumber }) => { $("page").value = String(pageNumber); });
eventBus.on("pagerendered", ({ error }) => { if (!error) { vscode.postMessage({ type: "rendered" }); } });
eventBus.on("updatefindmatchescount", ({ matchesCount }) => { $("matches").textContent = `${matchesCount.current}/${matchesCount.total}`; });
$("container").addEventListener("scroll", () => {
  if (loaded) { vscode.setState({ page: viewer.currentPageNumber, scale: viewer.currentScaleValue, top: $("container").scrollTop, left: $("container").scrollLeft }); }
});
$("container").addEventListener("dblclick", (event) => {
  const element = event.target.closest(".page");
  if (!element || !loaded) { return; }
  const page = Number(element.dataset.pageNumber);
  const view = viewer.getPageView(page - 1);
  const rect = element.getBoundingClientRect();
  const [x, y] = view.viewport.convertToPdfPoint(event.clientX - rect.left - element.clientLeft, event.clientY - rect.top - element.clientTop);
  const box = view.viewport.viewBox;
  vscode.postMessage({ type: "inverse", page, x: x - box[0], y: box[3] - y });
});
$("previous").onclick = () => { if (loaded && viewer.currentPageNumber > 1) { viewer.currentPageNumber--; } };
$("next").onclick = () => { if (loaded && viewer.currentPageNumber < viewer.pagesCount) { viewer.currentPageNumber++; } };
$("page").onchange = () => { if (loaded) { viewer.currentPageNumber = Math.max(1, Math.min(viewer.pagesCount, Number($("page").value) || 1)); } };
$("out").onclick = () => { if (loaded) { viewer.currentScale = Math.max(0.25, viewer.currentScale / 1.2); } };
$("in").onclick = () => { if (loaded) { viewer.currentScale = Math.min(5, viewer.currentScale * 1.2); } };
$("fit").onclick = () => { if (loaded) { viewer.currentScaleValue = "page-width"; } };
const search = (again, backwards = false) => eventBus.dispatch("find", { source: window, type: again ? "again" : "", query: $("search").value, caseSensitive: false, entireWord: false, highlightAll: true, findPrevious: backwards });
$("search").oninput = () => search(false);
$("search").onkeydown = (event) => { if (event.key === "Enter") { search(true, event.shiftKey); } };
document.addEventListener("keydown", (event) => { if ((event.metaKey || event.ctrlKey) && event.key === "f") { event.preventDefault(); $("search").focus(); } });

const reload = createPdfReloader({
  load: async (url) => {
    // Read a complete snapshot. Lazy HTTP range requests against an output
    // file being overwritten can corrupt even the previously loaded PDF.
    const response = await fetch(url);
    if (!response.ok) { throw new Error("PDF is unavailable. Build the document and check Tekai output."); }
    const bytes = new Uint8Array(await response.arrayBuffer());
    const tail = new TextDecoder("latin1").decode(bytes.subarray(Math.max(0, bytes.length - 2048)));
    if (!tail.includes("%%EOF")) { throw new Error("PDF is incomplete. Waiting for a successful build."); }
    return getDocument({ data: bytes, worker, cMapUrl: new URL("./pdfjs/cmaps/", import.meta.url).href, cMapPacked: true,
      standardFontDataUrl: new URL("./pdfjs/standard_fonts/", import.meta.url).href,
      wasmUrl: new URL("./pdfjs/wasm/", import.meta.url).href, useWorkerFetch: false, isEvalSupported: false, enableXfa: false });
  },
  commit: (pdf) => {
    restore = vscode.getState() ?? restore;
    loaded = false;
    viewer.setDocument(null);
    linkService.setDocument(null);
    linkService.setDocument(pdf);
    viewer.setDocument(pdf);
    void viewer.pagesPromise?.catch((error) => vscode.postMessage({ type: "error", message: `PDF viewer initialization failed. ${error.message}` }));
  },
  onError: (error, preserved) => {
    $("status").textContent = preserved ? "Showing the previous PDF. Waiting for a successful build." : error.message;
    vscode.postMessage({ type: preserved ? "warning" : "error", message: `PDF preview: ${error.message}` });
  },
});
window.addEventListener("message", ({ data }) => {
  if (data.type === "sync") { sync(data); }
  if (data.type === "load") { void reload(data.url); }
  if (data.type === "waiting") { $("status").textContent = loaded ? "Showing the previous PDF. Waiting for a successful build." : "PDF is unavailable. Build the document and check Tekai output."; }
});
vscode.postMessage({ type: "ready" });
}
