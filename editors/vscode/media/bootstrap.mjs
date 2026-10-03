const vscode = acquireVsCodeApi();
function report(error) {
  const message = error?.message ?? String(error);
  document.getElementById("status").textContent = `PDF error: ${message}`;
  vscode.postMessage({ type: "error", message });
}
window.addEventListener("error", (event) => report(event.error ?? event.message));
window.addEventListener("unhandledrejection", (event) => report(event.reason));
import("./viewer.mjs").then(({ start }) => start(vscode)).catch(report);
