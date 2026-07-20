import * as path from "node:path";
import * as vscode from "vscode";

export class PdfPreview implements vscode.Disposable {
  private panel: vscode.WebviewPanel | undefined;
  private watcher: vscode.FileSystemWatcher | undefined;
  private pdf: vscode.Uri | undefined;

  async show(pdf: vscode.Uri, viewer: "tab" | "external"): Promise<void> {
    this.pdf = pdf;
    if (viewer === "external") {
      await vscode.env.openExternal(pdf);
      return;
    }
    if (!this.panel) {
      this.panel = vscode.window.createWebviewPanel(
        "tekai.pdfPreview",
        `Tekai: ${path.basename(pdf.fsPath)}`,
        vscode.ViewColumn.Beside,
        {
          enableScripts: false,
          localResourceRoots: [vscode.Uri.file(path.dirname(pdf.fsPath))],
          retainContextWhenHidden: true,
        },
      );
      this.panel.onDidDispose(() => {
        this.panel = undefined;
        this.watcher?.dispose();
        this.watcher = undefined;
      });
    } else {
      this.panel.title = `Tekai: ${path.basename(pdf.fsPath)}`;
      this.panel.webview.options = {
        enableScripts: false,
        localResourceRoots: [vscode.Uri.file(path.dirname(pdf.fsPath))],
      };
      this.panel.reveal(vscode.ViewColumn.Beside, true);
    }
    this.installWatcher(pdf);
    this.refresh();
  }

  reveal(): boolean {
    if (!this.panel) {
      return false;
    }
    this.panel.reveal(vscode.ViewColumn.Beside, true);
    return true;
  }

  private installWatcher(pdf: vscode.Uri): void {
    this.watcher?.dispose();
    this.watcher = vscode.workspace.createFileSystemWatcher(
      new vscode.RelativePattern(path.dirname(pdf.fsPath), path.basename(pdf.fsPath)),
    );
    this.watcher.onDidChange(() => this.refresh());
    this.watcher.onDidCreate(() => this.refresh());
  }

  private refresh(): void {
    if (!this.panel || !this.pdf) {
      return;
    }
    const resource = this.panel.webview.asWebviewUri(this.pdf);
    const source = `${resource.toString()}?version=${Date.now()}`;
    this.panel.webview.html = `<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; frame-src ${this.panel.webview.cspSource}; object-src ${this.panel.webview.cspSource}; style-src 'unsafe-inline';">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <style>html, body, iframe { width: 100%; height: 100%; padding: 0; margin: 0; border: 0; overflow: hidden; background: var(--vscode-editor-background); }</style>
</head>
<body><iframe src="${source}" title="Tekai PDF preview"></iframe></body>
</html>`;
  }

  dispose(): void {
    this.watcher?.dispose();
    this.panel?.dispose();
  }
}
