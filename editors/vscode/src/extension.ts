import { ChildProcessWithoutNullStreams, spawn } from "node:child_process";
import * as path from "node:path";
import * as vscode from "vscode";
import { absoluteReportedPath, parseBuildReport, parseBuiltPdf, parseLintReport } from "./protocol";
import { PdfPreview } from "./preview";
import { commandCwd, resolveMainDocument } from "./root";

interface CommandResult {
  code: number | null;
  stdout: string;
  stderr: string;
}

function isTexDocument(document: vscode.TextDocument): boolean {
  const extension = path.extname(document.uri.fsPath).toLowerCase();
  return document.uri.scheme === "file" && (extension === ".tex" || extension === ".ltx" || extension === ".cls");
}

class TekaiController implements vscode.Disposable {
  private readonly output = vscode.window.createOutputChannel("Tekai");
  private readonly diagnostics = vscode.languages.createDiagnosticCollection("tekai");
  private readonly preview = new PdfPreview();
  private readonly status = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 10);
  private readonly subscriptions: vscode.Disposable[] = [];
  private readonly lintProcesses = new Map<string, ChildProcessWithoutNullStreams>();
  private buildProcess: ChildProcessWithoutNullStreams | undefined;
  private watchProcess: ChildProcessWithoutNullStreams | undefined;
  private lastPdf: vscode.Uri | undefined;

  constructor(context: vscode.ExtensionContext) {
    this.status.command = "tekai.showOutput";
    this.status.text = "$(check) Tekai";
    this.status.tooltip = "Show Tekai output";
    this.status.show();

    this.registerCommand("tekai.lint", () => this.lintActive());
    this.registerCommand("tekai.lintWorkspace", () => this.lintWorkspace());
    this.registerCommand("tekai.build", () => this.build(false));
    this.registerCommand("tekai.preview", () => this.build(true));
    this.registerCommand("tekai.watch", () => this.startWatch());
    this.registerCommand("tekai.stop", () => this.stopWatch());
    this.registerCommand("tekai.openPdf", () => this.openLastPdf());
    this.registerCommand("tekai.showOutput", () => this.output.show());

    this.subscriptions.push(
      vscode.workspace.onDidSaveTextDocument((document) => void this.onSave(document)),
      vscode.workspace.onDidOpenTextDocument((document) => {
        if (isTexDocument(document) && this.configuration(document.uri).get("lint.run", "onSave") === "onSave") {
          this.background(this.lintDocument(document, false));
        }
      }),
      this.output,
      this.diagnostics,
      this.preview,
      this.status,
    );
    context.subscriptions.push(this);

    const active = vscode.window.activeTextEditor?.document;
    if (active && isTexDocument(active) && this.configuration(active.uri).get("lint.run", "onSave") === "onSave") {
      this.background(this.lintDocument(active, false));
    }
  }

  private registerCommand(name: string, callback: () => unknown): void {
    this.subscriptions.push(
      vscode.commands.registerCommand(name, async () => {
        try {
          await callback();
        } catch (error) {
          this.reportError(error);
        }
      }),
    );
  }

  private configuration(scope?: vscode.Uri): vscode.WorkspaceConfiguration {
    return vscode.workspace.getConfiguration("tekai", scope);
  }

  private executable(scope?: vscode.Uri): string {
    return this.configuration(scope).get("executable", "tekai");
  }

  private configArgs(scope: vscode.Uri, cwd: string): string[] {
    const configured = this.configuration(scope).get<string>("configFile", "").trim();
    if (!configured) {
      return [];
    }
    return ["--config", path.isAbsolute(configured) ? configured : path.resolve(cwd, configured)];
  }

  private async onSave(document: vscode.TextDocument): Promise<void> {
    if (!isTexDocument(document)) {
      return;
    }
    if (this.configuration(document.uri).get("lint.run", "onSave") === "onSave") {
      this.background(this.lintDocument(document, false));
    }
    if (this.configuration(document.uri).get("build.onSave", false)) {
      this.background(this.build(false, document, false));
    }
  }

  private async lintActive(): Promise<void> {
    const document = vscode.window.activeTextEditor?.document;
    if (!document || !isTexDocument(document)) {
      throw new Error("Open a TeX file to lint it");
    }
    await this.lintDocument(document, true);
  }

  private async lintWorkspace(): Promise<void> {
    const document = vscode.window.activeTextEditor?.document;
    const folder = document ? vscode.workspace.getWorkspaceFolder(document.uri) : vscode.workspace.workspaceFolders?.[0];
    if (!folder) {
      throw new Error("Open a workspace before linting it");
    }
    this.diagnostics.clear();
    await this.runLint(folder.uri.fsPath, folder.uri.fsPath, folder.uri, true);
  }

  private async lintDocument(document: vscode.TextDocument, revealErrors: boolean): Promise<void> {
    this.diagnostics.delete(document.uri);
    const folder = vscode.workspace.getWorkspaceFolder(document.uri);
    await this.runLint(document.uri.fsPath, folder?.uri.fsPath ?? path.dirname(document.uri.fsPath), document.uri, revealErrors);
  }

  private async runLint(target: string, cwd: string, scope: vscode.Uri, revealErrors: boolean): Promise<void> {
    const key = path.resolve(target);
    this.lintProcesses.get(key)?.kill();
    const args = ["lint", target, "--report-json", "--allow-warnings", ...this.configArgs(scope, cwd)];
    let started: ChildProcessWithoutNullStreams | undefined;
    const result = await this.runProcess(this.executable(scope), args, cwd, (child) => {
      started = child;
      this.lintProcesses.set(key, child);
    });
    if (!started || this.lintProcesses.get(key) !== started) {
      return;
    }
    this.lintProcesses.delete(key);
    let report;
    try {
      report = parseLintReport(result.stdout);
    } catch (error) {
      if (result.code === null) {
        return;
      }
      this.output.appendLine(result.stderr || result.stdout);
      if (revealErrors) {
        this.output.show(true);
      }
      throw error;
    }

    const grouped = new Map<string, vscode.Diagnostic[]>();
    for (const item of report.diagnostics) {
      const filename = absoluteReportedPath(item.path, cwd);
      const start = new vscode.Position(Math.max(0, item.line - 1), Math.max(0, item.column - 1));
      const diagnostic = new vscode.Diagnostic(
        new vscode.Range(start, start.translate(0, 1)),
        item.help ? `${item.message}\n${item.help}` : item.message,
        item.severity === "error" ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning,
      );
      diagnostic.code = item.rule;
      diagnostic.source = "tekai";
      const entries = grouped.get(filename) ?? [];
      entries.push(diagnostic);
      grouped.set(filename, entries);
    }
    for (const [filename, entries] of grouped) {
      this.diagnostics.set(vscode.Uri.file(filename), entries);
    }
    this.status.text = report.error_count > 0
      ? `$(error) Tekai ${report.error_count}`
      : report.warning_count > 0
        ? `$(warning) Tekai ${report.warning_count}`
        : "$(check) Tekai";
  }

  private async activeDocument(): Promise<vscode.TextDocument> {
    const document = vscode.window.activeTextEditor?.document;
    if (!document || !isTexDocument(document)) {
      throw new Error("Open a TeX file first");
    }
    return document;
  }

  private async build(
    fastPreview: boolean,
    sourceDocument?: vscode.TextDocument,
    interactive = true,
  ): Promise<void> {
    const document = sourceDocument ?? await this.activeDocument();
    const main = await resolveMainDocument(document, interactive);
    if (!main) {
      if (interactive) {
        throw new Error("Could not determine the root TeX file; set tekai.mainFile or add a % !TEX root comment");
      }
      return;
    }
    const cwd = commandCwd(main);
    const config = this.configuration(main);
    const args = ["build", main.fsPath, "--report-json", ...this.configArgs(main, cwd)];
    if (fastPreview) {
      args.push("--once", "--fast");
      args.push(...config.get<string[]>("preview.extraArgs", []));
    } else {
      args.push(...config.get<string[]>("build.extraArgs", []));
    }

    this.buildProcess?.kill();
    this.status.text = fastPreview ? "$(loading~spin) Tekai preview" : "$(loading~spin) Tekai build";
    let started: ChildProcessWithoutNullStreams | undefined;
    const result = await this.runProcess(this.executable(main), args, cwd, (child) => {
      started = child;
      this.buildProcess = child;
    });
    if (!started || this.buildProcess !== started) {
      return;
    }
    this.buildProcess = undefined;
    this.output.append(result.stderr);
    if (result.code !== 0) {
      this.status.text = "$(error) Tekai build";
      this.output.show(true);
      throw new Error(`Tekai ${fastPreview ? "preview" : "build"} failed (exit ${result.code ?? "signal"})`);
    }
    const report = parseBuildReport(result.stdout);
    if (report.pdf_path) {
      this.lastPdf = vscode.Uri.file(absoluteReportedPath(report.pdf_path, cwd));
    }
    this.status.text = report.skipped ? "$(check) Tekai cached" : "$(check) Tekai built";
    this.output.appendLine(
      `${report.skipped ? "Cached" : "Built"} in ${Math.round(report.elapsed_ms)} ms${this.lastPdf ? `: ${this.lastPdf.fsPath}` : ""}`,
    );
    if (fastPreview && this.lastPdf && config.get("preview.openAfterBuild", true)) {
      await this.showPdf(this.lastPdf, main);
    }
  }

  private async startWatch(): Promise<void> {
    const document = await this.activeDocument();
    const main = await resolveMainDocument(document, true);
    if (!main) {
      throw new Error("Could not determine the root TeX file; set tekai.mainFile or add a % !TEX root comment");
    }
    this.stopWatch();
    const cwd = commandCwd(main);
    const config = this.configuration(main);
    const args = ["watch", main.fsPath, "--preview", "--allow-warnings", ...this.configArgs(main, cwd)];
    const idle = config.get<number | null>("preview.finalAfterIdleMs", 1500);
    if (idle !== null) {
      args.push("--final-after-idle-ms", String(idle));
    }
    args.push(...config.get<string[]>("preview.extraArgs", []));

    this.output.appendLine(`$ ${[this.executable(main), ...args].map((arg) => JSON.stringify(arg)).join(" ")}`);
    let stderrBuffer = "";
    let opened = false;
    const child = spawn(this.executable(main), args, { cwd, env: process.env });
    this.watchProcess = child;
    this.status.text = "$(eye) Tekai watching";
    child.stdout.on("data", (chunk: Buffer) => this.output.append(chunk.toString()));
    child.stderr.on("data", (chunk: Buffer) => {
      const text = chunk.toString();
      this.output.append(text);
      stderrBuffer += text;
      const lines = stderrBuffer.split(/\r?\n/);
      stderrBuffer = lines.pop() ?? "";
      for (const line of lines) {
        const reported = parseBuiltPdf(line);
        if (!reported) {
          continue;
        }
        this.lastPdf = vscode.Uri.file(absoluteReportedPath(reported, cwd));
        if (!opened && config.get("preview.openAfterBuild", true)) {
          opened = true;
          this.background(this.showPdf(this.lastPdf, main));
        }
      }
    });
    child.on("error", (error) => this.reportError(error));
    child.on("close", (code) => {
      const wasCurrent = this.watchProcess === child;
      if (wasCurrent) {
        this.watchProcess = undefined;
      }
      if (!wasCurrent) {
        return;
      }
      this.status.text = code === 0 ? "$(check) Tekai" : "$(error) Tekai watch";
      if (code !== 0) {
        this.output.show(true);
        void vscode.window.showErrorMessage(`Tekai live preview stopped (exit ${code ?? "signal"})`);
      }
    });
  }

  private stopWatch(): void {
    if (!this.watchProcess) {
      return;
    }
    this.watchProcess.kill();
    this.watchProcess = undefined;
    this.status.text = "$(check) Tekai";
  }

  private async showPdf(pdf: vscode.Uri, scope: vscode.Uri): Promise<void> {
    const viewer = this.configuration(scope).get<"tab" | "external">("preview.viewer", "tab");
    await this.preview.show(pdf, viewer);
  }

  private async openLastPdf(): Promise<void> {
    if (!this.lastPdf) {
      throw new Error("No Tekai PDF has been built in this session");
    }
    const scope = vscode.window.activeTextEditor?.document.uri ?? this.lastPdf;
    await this.showPdf(this.lastPdf, scope);
  }

  private runProcess(
    executable: string,
    args: string[],
    cwd: string,
    onStart: (child: ChildProcessWithoutNullStreams) => void,
  ): Promise<CommandResult> {
    this.output.appendLine(`$ ${[executable, ...args].map((arg) => JSON.stringify(arg)).join(" ")}`);
    return new Promise((resolve, reject) => {
      const child = spawn(executable, args, { cwd, env: process.env });
      onStart(child);
      let stdout = "";
      let stderr = "";
      child.stdout.on("data", (chunk: Buffer) => { stdout += chunk.toString(); });
      child.stderr.on("data", (chunk: Buffer) => { stderr += chunk.toString(); });
      child.on("error", reject);
      child.on("close", (code) => resolve({ code, stdout, stderr }));
    });
  }

  private reportError(error: unknown): void {
    const message = error instanceof Error ? error.message : String(error);
    this.output.appendLine(`error: ${message}`);
    void vscode.window.showErrorMessage(`Tekai: ${message}`, "Show Output").then((choice) => {
      if (choice === "Show Output") {
        this.output.show(true);
      }
    });
  }

  private background(operation: Promise<unknown>): void {
    void operation.catch((error) => this.reportError(error));
  }

  dispose(): void {
    this.stopWatch();
    this.buildProcess?.kill();
    for (const child of this.lintProcesses.values()) {
      child.kill();
    }
    for (const disposable of this.subscriptions.splice(0)) {
      disposable.dispose();
    }
  }
}

export function activate(context: vscode.ExtensionContext): void {
  new TekaiController(context);
}

export function deactivate(): void {
  // VS Code disposes the extension context subscriptions.
}
