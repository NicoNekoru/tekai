import { randomBytes } from "node:crypto";
import * as path from "node:path";
import * as vscode from "vscode";
import { PdfPosition } from "./synctex";

interface PreviewEntry { panel: vscode.WebviewPanel; ready: boolean; rendered: Promise<boolean>; pending?: PdfPosition; refresh: () => void }
const escape = (value: string) => value.replace(/&/g, "&amp;").replace(/"/g, "&quot;").replace(/</g, "&lt;");

export class PdfPreview implements vscode.CustomReadonlyEditorProvider, vscode.Disposable {
  private readonly entries = new Map<string, PreviewEntry>();
  private readonly registration: vscode.Disposable;

  constructor(private readonly extensionUri: vscode.Uri, private readonly inverse: (pdf: vscode.Uri, point: PdfPosition) => Promise<void>,
    private readonly onError: (error: unknown) => void) {
    this.registration = vscode.window.registerCustomEditorProvider("tekai.pdfPreview", this, { webviewOptions: { retainContextWhenHidden: true }, supportsMultipleEditorsPerDocument: false });
  }

  openCustomDocument(uri: vscode.Uri): vscode.CustomDocument { return { uri, dispose() {} }; }

  resolveCustomEditor(document: vscode.CustomDocument, panel: vscode.WebviewPanel): void {
    const pdf = document.uri;
    const assets = vscode.Uri.joinPath(this.extensionUri, "dist", "viewer");
    panel.webview.options = { enableScripts: true, localResourceRoots: [assets, vscode.Uri.file(path.dirname(pdf.fsPath))] };
    let finish: (ready: boolean) => void = () => {};
    const rendered = new Promise<boolean>((resolve) => { finish = resolve; });
    const readinessTimeout = setTimeout(() => finish(false), 20000);
    const entry: PreviewEntry = { panel, ready: false, rendered, refresh: async () => {
      if (!entry.ready) { return; }
      try {
        const stat = await vscode.workspace.fs.stat(pdf);
        if (!stat.size) { throw new Error("empty PDF"); }
        await panel.webview.postMessage({ type: "load", url: `${panel.webview.asWebviewUri(pdf)}?v=${Date.now()}` });
      } catch { await panel.webview.postMessage({ type: "waiting" }); }
    } };
    this.entries.set(pdf.toString(), entry);
    const watcher = vscode.workspace.createFileSystemWatcher(new vscode.RelativePattern(path.dirname(pdf.fsPath), path.basename(pdf.fsPath)));
    let timer: ReturnType<typeof setTimeout> | undefined;
    const refresh = () => { clearTimeout(timer); timer = setTimeout(entry.refresh, 180); };
    const subscriptions = [watcher, watcher.onDidChange(refresh), watcher.onDidCreate(refresh), watcher.onDidDelete(refresh), panel.webview.onDidReceiveMessage((message: unknown) => {
      if (!message || typeof message !== "object") { return; }
      const data = message as Record<string, unknown>;
      if (data.type === "ready") { entry.ready = true; entry.refresh(); }
      if (data.type === "loaded" && entry.pending) { void panel.webview.postMessage({ type: "sync", ...entry.pending }); entry.pending = undefined; }
      if (data.type === "rendered") { clearTimeout(readinessTimeout); finish(true); }
      if (data.type === "error" && typeof data.message === "string") { this.onError(new Error(data.message)); }
      if (data.type === "inverse" && Number.isInteger(data.page) && Number(data.page) > 0 && typeof data.x === "number" && Number.isFinite(data.x) && typeof data.y === "number" && Number.isFinite(data.y)) {
        void this.inverse(pdf, { page: Number(data.page), x: data.x, y: data.y }).catch(this.onError);
      }
    })];
    panel.onDidDispose(() => { clearTimeout(timer); clearTimeout(readinessTimeout); finish(false); subscriptions.forEach((item) => item.dispose()); this.entries.delete(pdf.toString()); });
    const nonce = randomBytes(16).toString("hex");
    const resource = (file: string) => escape(panel.webview.asWebviewUri(vscode.Uri.joinPath(assets, file)).toString());
    const csp = panel.webview.cspSource;
    panel.webview.html = `<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'nonce-${nonce}' ${csp}; worker-src ${csp} blob:; connect-src ${csp}; img-src ${csp} data: blob:; font-src ${csp} data: blob:; style-src ${csp} 'unsafe-inline';">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="${resource("pdfjs/web/pdf_viewer.css")}"><link rel="stylesheet" href="${resource("viewer.css")}">
</head><body><nav aria-label="PDF toolbar">
<button id="previous" title="Previous page">Previous</button><input id="page" type="number" min="1" value="1" aria-label="Page"><span id="count"></span><button id="next" title="Next page">Next</button>
<button id="out" aria-label="Zoom out">−</button><button id="fit">Fit width</button><button id="in" aria-label="Zoom in">+</button>
<input id="search" type="search" placeholder="Find in PDF" aria-label="Find in PDF"><span id="matches"></span><span id="status" role="status">Loading PDF…</span>
</nav><div id="container" tabindex="0"><div id="viewer" class="pdfViewer"></div></div>
<script type="module" nonce="${nonce}" src="${resource("bootstrap.mjs")}"></script></body></html>`;
  }

  async show(pdf: vscode.Uri, viewer: "tab" | "external", position?: PdfPosition): Promise<void> {
    if (viewer === "external" && !position) { await vscode.env.openExternal(pdf); return; }
    let entry = this.entries.get(pdf.toString());
    if (!entry) {
      try { if (!(await vscode.workspace.fs.stat(pdf)).size) { throw new Error("empty PDF"); } }
      catch { throw new Error("PDF is missing or empty. Fix the build error and rebuild before opening the preview."); }
      await vscode.commands.executeCommand("vscode.openWith", pdf, "tekai.pdfPreview", { viewColumn: vscode.ViewColumn.Beside, preserveFocus: true });
      entry = this.entries.get(pdf.toString());
    } else { entry.panel.reveal(vscode.ViewColumn.Beside, true); }
    if (entry && position) {
      entry.pending = position;
      if (entry.ready) { await entry.panel.webview.postMessage({ type: "sync", ...position }); entry.pending = undefined; }
    }
    if (entry && !await entry.rendered) { throw new Error("PDF preview did not render. Close the preview and reopen it; see Tekai output for errors."); }
  }

  dispose(): void { this.registration.dispose(); for (const entry of this.entries.values()) { entry.panel.dispose(); } }
}
