import { ChildProcessWithoutNullStreams, spawn } from "node:child_process";
import * as path from "node:path";
import * as vscode from "vscode";
import { absoluteReportedPath, LintReport, parseBuildReport, parseBuiltPdf, parseLintReport } from "./protocol";
import { PdfPreview } from "./preview";
import { commandCwd, resolveMainDocument } from "./root";
import { registerLanguageFeatures } from "./language";
import { hasSyncTeXMap, parseForwardSync, parseInverseSync, PdfPosition, runSyncTeX, SyncTeXError } from "./synctex";
import { findProjectConfig } from "./config";
import { parseTexErrors } from "./buildErrors";
import { allowWarningsOnce, CheckBlockedError, describeBlockedCheck, formatLintOutput, lintBlocksCheck } from "./checkResult";

interface CommandResult {
  code: number | null;
  stdout: string;
  stderr: string;
}

function isTexDocument(document: vscode.TextDocument): boolean {
  const extension = path.extname(document.uri.fsPath).toLowerCase();
  return document.uri.scheme === "file" && [".tex", ".ltx", ".cls", ".sty"].includes(extension);
}

class TekaiController implements vscode.Disposable {
  private readonly output = vscode.window.createOutputChannel("Tekai");
  private readonly diagnostics = vscode.languages.createDiagnosticCollection("tekai");
  private readonly buildDiagnostics = vscode.languages.createDiagnosticCollection("tekai-build");
  private readonly preview: PdfPreview;
  private readonly status = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 10);
  private readonly subscriptions: vscode.Disposable[] = [];
  private readonly lintProcesses = new Map<string, ChildProcessWithoutNullStreams>();
  private readonly lintGenerations = new Map<string, number>();
  private nextLintGeneration = 0;
  private buildProcess: ChildProcessWithoutNullStreams | undefined;
  private watchProcess: ChildProcessWithoutNullStreams | undefined;
  private lastPdf: vscode.Uri | undefined;
  private readonly pdfs = new Map<string, string>();
  private saving = false;

  constructor(private readonly context: vscode.ExtensionContext) {
    this.preview = new PdfPreview(context.extensionUri, (pdf, point) => this.inverseSync(pdf, point), (error) => this.reportError(error));
    for (const [main, pdf] of context.workspaceState.get<[string, string][]>("tekai.pdfs", [])) { this.pdfs.set(main, pdf); }
    this.status.command = "tekai.showOutput";
    this.status.text = "$(check) Tekai";
    this.status.tooltip = "Show Tekai output";
    this.status.show();

    this.registerCommand("tekai.lint", () => this.lintActive());
    this.registerCommand("tekai.lintWorkspace", () => this.lintWorkspace());
    this.registerCommand("tekai.build", () => this.build(false));
    this.registerCommand("tekai.check", () => this.build(false, undefined, true, true));
    this.registerCommand("tekai.checkAllowWarnings", () => this.build(false, undefined, true, true, true));
    this.registerCommand("tekai.preview", () => this.build(true));
    this.registerCommand("tekai.watch", () => this.startWatch());
    this.registerCommand("tekai.stop", () => this.stopWatch());
    this.registerCommand("tekai.toggleWatch", () => this.watchProcess ? this.stopWatch() : this.startWatch());
    this.registerCommand("tekai.forwardSync", () => this.forwardSync());
    this.registerCommand("tekai.health", () => this.health());
    this.registerCommand("tekai.openPdf", () => this.openLastPdf());
    this.registerCommand("tekai.showOutput", () => this.output.show());

    this.subscriptions.push(
      vscode.workspace.onDidSaveTextDocument((document) => void this.onSave(document)),
      vscode.workspace.onDidOpenTextDocument((document) => {
        if (vscode.workspace.isTrusted && isTexDocument(document) && this.configuration(document.uri).get("lint.onOpen", true) && this.configuration(document.uri).get<string>("lint.run", "onSave") !== "manual") {
          this.background(this.lintDocument(document, false));
        }
      }),
      this.output,
      this.diagnostics,
      this.buildDiagnostics,
      this.preview,
      this.status,
      registerLanguageFeatures(context.extensionUri),
    );
    context.subscriptions.push(this);

    const active = vscode.window.activeTextEditor?.document;
    if (vscode.workspace.isTrusted && active && isTexDocument(active) && this.configuration(active.uri).get("lint.onOpen", true) && this.configuration(active.uri).get<string>("lint.run", "onSave") !== "manual") {
      this.background(this.lintDocument(active, false));
    }
  }

  private registerCommand(name: string, callback: () => unknown): void {
    this.subscriptions.push(
      vscode.commands.registerCommand(name, async () => {
        try {
          if (!vscode.workspace.isTrusted) { throw new Error("Trust this workspace before running Tekai commands."); }
          await callback();
        } catch (error) {
          this.reportError(error);
          throw error;
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

  private async configArgs(scope: vscode.Uri, cwd: string): Promise<string[]> {
    const configured = this.configuration(scope).get<string>("configFile", "").trim();
    const base = vscode.workspace.getWorkspaceFolder(scope)?.uri.fsPath ?? cwd;
    const filename = configured ? path.resolve(base, configured) : await findProjectConfig(path.dirname(scope.fsPath));
    return filename ? ["--config", filename] : [];
  }

  private async onSave(document: vscode.TextDocument): Promise<void> {
    if (!vscode.workspace.isTrusted || this.saving) {
      return;
    }
    if (path.extname(document.fileName).toLowerCase() === ".toml") {
      // Config edits should clear obsolete Problems without requiring a window
      // reload or modifying every open source buffer. Manual lint stays manual.
      const documents = vscode.workspace.textDocuments.filter((item) => isTexDocument(item) && this.configuration(item.uri).get<string>("lint.run", "onSave") !== "manual");
      if (documents.length) { this.background(this.lintFiles(documents.map((item) => item.uri), false)); }
      return;
    }
    if (!isTexDocument(document)) { return; }
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
    this.lintGenerations.clear();
    const files = await vscode.workspace.findFiles(new vscode.RelativePattern(folder, "**/*.{tex,ltx,cls,TEX,LTX,CLS}"), "**/{.git,target,build,.latexmk,.tekai}/**");
    await this.lintFiles(files, true);
  }

  private async lintDocument(document: vscode.TextDocument, revealErrors: boolean): Promise<void> {
    await this.lintFiles([document.uri], revealErrors);
  }

  private async lintFiles(files: vscode.Uri[], revealErrors: boolean): Promise<void> {
    if (!files.length) { this.status.text = "$(check) Tekai"; return; }
    const generation = ++this.nextLintGeneration;
    for (const file of files) { this.lintGenerations.set(file.fsPath, generation); }
    const groups = new Map<string, { files: vscode.Uri[]; cwd: string; configArgs: string[] }>();
    for (const file of files) {
      const cwd = vscode.workspace.getWorkspaceFolder(file)?.uri.fsPath ?? path.dirname(file.fsPath);
      const configArgs = await this.configArgs(file, cwd);
      const key = JSON.stringify([this.executable(file), cwd, configArgs]);
      let group = groups.get(key);
      if (!group) { group = { files: [], cwd, configArgs }; groups.set(key, group); }
      group.files.push(file);
    }
    for (const group of groups.values()) {
      // Bound argv size for large workspaces while keeping one process per
      // config for ordinary projects. Never apply a parent's rules recursively
      // to files that have a nearer config of their own.
      for (let offset = 0; offset < group.files.length; offset += 100) {
        await this.runLint(group.files.slice(offset, offset + 100), group.cwd, group.configArgs, generation, revealErrors);
      }
    }
  }

  private async runLint(files: vscode.Uri[], cwd: string, configArgs: string[], generation: number, revealErrors: boolean): Promise<void> {
    files = files.filter((file) => this.lintGenerations.get(file.fsPath) === generation);
    if (!files.length) { return; }
    const scope = files[0];
    const key = JSON.stringify(files.map((file) => file.fsPath));
    this.lintProcesses.get(key)?.kill();
    const args = ["lint", ...files.map((file) => file.fsPath), "--report-json", ...configArgs];
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
      if (!result.stdout.trim() && result.code !== 0) { throw new Error("Tekai lint failed. See Tekai output for configuration or source-file errors."); }
      throw error;
    }

    const current = new Set(files.filter((file) => this.lintGenerations.get(file.fsPath) === generation).map((file) => file.fsPath));
    if (!current.size) { return; }
    for (const file of current) {
      this.diagnostics.delete(vscode.Uri.file(file));
      this.lintGenerations.delete(file);
    }
    const diagnostics = report.diagnostics.filter((item) => current.has(absoluteReportedPath(item.path, cwd)));
    if (revealErrors) {
      this.output.appendLine(formatLintOutput(report, cwd));
      if (diagnostics.length) { await vscode.commands.executeCommand("workbench.actions.view.problems"); }
    }
    this.applyDiagnostics({ diagnostics, error_count: diagnostics.filter((item) => item.severity === "error").length,
      warning_count: diagnostics.filter((item) => item.severity === "warning").length }, cwd);
  }

  private applyDiagnostics(report: LintReport, cwd: string): void {
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
    check = false,
    permitWarningsOnce = false,
  ): Promise<void> {
    // Notification retries can outlive a workspace trust change.
    if (!vscode.workspace.isTrusted) { throw new Error("Trust this workspace before running Tekai commands."); }
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
    if (interactive) { await this.saveSources(main); }
    let args = [check ? "check" : "build", main.fsPath, "--report-json", ...await this.configArgs(main, cwd)];
    if (config.get("synctex.enabled", true)) { args.push("--synctex"); }
    if (fastPreview) {
      args.push("--once", "--fast");
      args.push(...config.get<string[]>("preview.extraArgs", []));
    } else if (check) {
      args.push(...config.get<string[]>("build.extraArgs", []), ...config.get<string[]>("check.extraArgs", []));
      // An explicit check owns the complete diagnostic set. Ignore any older lint jobs.
      for (const child of this.lintProcesses.values()) { child.kill(); }
      this.lintProcesses.clear();
      this.lintGenerations.clear();
    } else {
      args.push(...config.get<string[]>("build.extraArgs", []));
    }
    if (check && permitWarningsOnce) { args = allowWarningsOnce(args); }

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
    this.buildDiagnostics.clear();
    this.output.append(result.stderr);
    this.output.appendLine(`Process exited with ${result.code === null ? "a signal" : `code ${result.code}`}.`);
    let lintReport: LintReport | undefined;
    if (check && result.stdout.trim()) {
      try { lintReport = parseLintReport(result.stdout); }
      catch {
        this.output.appendLine(result.stdout);
        this.output.show(true);
        this.status.text = "$(error) Tekai report";
        throw new Error("Could not read the CLI check report. Its complete stdout and stderr are in Tekai output.");
      }
      this.output.appendLine(formatLintOutput(lintReport, cwd));
      for (const child of this.lintProcesses.values()) { child.kill(); }
      this.lintProcesses.clear();
      this.lintGenerations.clear();
      this.diagnostics.clear();
      this.applyDiagnostics(lintReport, cwd);
    }
    if (result.code !== 0) {
      if (check && result.code === 1 && lintReport && lintBlocksCheck(lintReport, args)) {
        const warningsOnly = lintReport.error_count === 0;
        this.status.text = `${warningsOnly ? "$(warning)" : "$(error)"} Tekai check blocked`;
        if (interactive) { await vscode.commands.executeCommand("workbench.actions.view.problems"); }
        throw new CheckBlockedError(describeBlockedCheck(lintReport, cwd), warningsOnly,
          warningsOnly ? () => this.build(false, document, true, true, true) : undefined);
      }
      this.status.text = "$(error) Tekai build";
      this.output.show(true);
      if (!check && result.stdout.trim()) { this.output.appendLine(result.stdout); }
      const errors = parseTexErrors(result.stderr, path.dirname(main.fsPath));
      const grouped = new Map<string, vscode.Diagnostic[]>();
      for (const error of errors) {
        const diagnostic = new vscode.Diagnostic(new vscode.Range(error.line - 1, 0, error.line - 1, 1000), error.message, vscode.DiagnosticSeverity.Error);
        diagnostic.source = "tekai";
        diagnostic.code = "tex/compile";
        const entries = grouped.get(error.file) ?? [];
        entries.push(diagnostic);
        grouped.set(error.file, entries);
      }
      for (const [file, entries] of grouped) { this.buildDiagnostics.set(vscode.Uri.file(file), entries); }
      const first = errors[0];
      const detail = first ? `${path.relative(path.dirname(main.fsPath), first.file)}:${first.line}: ${first.message}` : result.stderr.trim().split(/\r?\n/)[0]?.replace(/^Error:\s*/, "");
      throw new Error(`Tekai ${check ? "check" : fastPreview ? "preview" : "build"} failed (exit ${result.code ?? "signal"}). ${detail ? `${detail} ` : ""}See Problems and Tekai output.`);
    }
    let report;
    try { report = parseBuildReport(result.stdout); }
    catch {
      this.output.appendLine(result.stdout);
      this.output.show(true);
      this.status.text = "$(error) Tekai report";
      throw new Error("The CLI exited successfully but returned an invalid build report. Its complete stdout and stderr are in Tekai output.");
    }
    if (report.pdf_path) {
      this.lastPdf = vscode.Uri.file(absoluteReportedPath(report.pdf_path, cwd));
      this.rememberPdf(main, this.lastPdf);
    }
    this.status.text = report.skipped ? "$(check) Tekai cached" : "$(check) Tekai built";
    this.output.appendLine(
      `${report.skipped ? "Cached" : "Built"} in ${Math.round(report.elapsed_ms)} ms${this.lastPdf ? `: ${this.lastPdf.fsPath}` : ""}`,
    );
    if (this.lastPdf && config.get(check ? "check.openOnSuccess" : fastPreview ? "preview.openAfterBuild" : "build.openAfterBuild", true)) {
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
    await this.saveSources(main);
    const cwd = commandCwd(main);
    const config = this.configuration(main);
    const args = ["watch", main.fsPath, "--preview", "--allow-warnings", ...await this.configArgs(main, cwd)];
    if (config.get("synctex.enabled", true)) { args.push("--synctex"); }
    const idle = config.get<number | null>("preview.finalAfterIdleMs", 1500);
    if (idle !== null) {
      args.push("--final-after-idle-ms", String(idle));
    }
    args.push(...config.get<string[]>("preview.extraArgs", []));

    this.output.appendLine(`cwd: ${cwd}`);
    this.output.appendLine(`$ ${[this.executable(main), ...args].map((arg) => JSON.stringify(arg)).join(" ")}`);
    let stderrBuffer = "";
    let opened = false;
    const child = spawn(this.executable(main), args, { cwd, env: process.env });
    this.watchProcess = child;
    this.status.text = "$(eye) Tekai watching";
    child.stdout.on("data", (chunk: Buffer) => this.output.append(chunk.toString()));
    child.stderr.on("data", (chunk: Buffer) => {
      if (this.watchProcess !== child) { return; }
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
        this.rememberPdf(main, this.lastPdf);
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
    const document = vscode.window.activeTextEditor?.document;
    if (document && isTexDocument(document)) {
      const main = await resolveMainDocument(document, true);
      const known = main && this.pdfs.get(main.fsPath);
      if (known) { await this.showPdf(vscode.Uri.file(known), main!); return; }
    }
    if (!this.lastPdf) {
      throw new Error("No Tekai PDF has been built in this session");
    }
    const scope = vscode.window.activeTextEditor?.document.uri ?? this.lastPdf;
    await this.showPdf(this.lastPdf, scope);
  }

  private rememberPdf(main: vscode.Uri, pdf: vscode.Uri): void {
    this.pdfs.set(main.fsPath, pdf.fsPath);
    void this.context.workspaceState.update("tekai.pdfs", [...this.pdfs]);
  }

  private async saveSources(main: vscode.Uri): Promise<void> {
    const directory = vscode.workspace.getWorkspaceFolder(main)?.uri.fsPath ?? path.dirname(main.fsPath);
    this.saving = true;
    try {
      for (const document of vscode.workspace.textDocuments) {
        const relative = path.relative(directory, document.fileName);
        if (relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) { continue; }
        if (document.isDirty && document.uri.scheme === "file" && /\.(tex|ltx|bib|cls|sty)$/i.test(document.fileName)) {
          if (!await document.save()) { throw new Error(`Could not save ${document.fileName}`); }
        }
      }
    } finally { this.saving = false; }
  }

  private async forwardSync(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    const document = await this.activeDocument();
    const position = editor!.selection.active;
    const main = await resolveMainDocument(document, true);
    if (!main) { throw new Error("Could not determine the root TeX file."); }
    if (!this.configuration(main).get("synctex.enabled", true)) { throw new Error("Enable tekai.synctex.enabled and rebuild first."); }
    if (!this.pdfs.has(main.fsPath)) { await this.build(false, document); }
    const pdf = this.pdfs.get(main.fsPath);
    if (!pdf) { throw new Error("Build the document before using SyncTeX."); }
    if (!await hasSyncTeXMap(pdf)) {
      // Older builds or an output-directory clean may leave a remembered PDF without its map.
      await this.build(false, document);
    }
    const currentPdf = this.pdfs.get(main.fsPath)!;
    const output = await runSyncTeX(this.configuration(main).get("synctex.executable", "synctex"),
      ["view", "-i", `${position.line + 1}:${position.character + 1}:${document.uri.fsPath}`, "-o", currentPdf], path.dirname(main.fsPath));
    await this.preview.show(vscode.Uri.file(currentPdf), "tab", parseForwardSync(output));
  }

  private async inverseSync(pdf: vscode.Uri, point: PdfPosition): Promise<void> {
    if (!vscode.workspace.isTrusted) { return; }
    const main = [...this.pdfs].find(([, file]) => file === pdf.fsPath)?.[0];
    const cwd = main ? path.dirname(main) : commandCwd(pdf);
    const scope = main ? vscode.Uri.file(main) : pdf;
    const output = await runSyncTeX(this.configuration(scope).get("synctex.executable", "synctex"),
      ["edit", "-o", `${point.page}:${point.x}:${point.y}:${pdf.fsPath}`], cwd);
    const target = parseInverseSync(output, cwd);
    const document = await vscode.workspace.openTextDocument(vscode.Uri.file(target.file));
    const existing = vscode.window.visibleTextEditors.find((editor) => editor.document.uri.toString() === document.uri.toString());
    const editor = await vscode.window.showTextDocument(document, { viewColumn: existing?.viewColumn ?? vscode.ViewColumn.One, preview: false });
    const position = document.validatePosition(new vscode.Position(target.line - 1, target.column - 1));
    editor.selection = new vscode.Selection(position, position);
    editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
  }

  private async health(): Promise<void> {
    const document = vscode.window.activeTextEditor?.document;
    const cwd = document ? commandCwd(document.uri) : vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? process.cwd();
    const result = await this.runProcess(this.executable(document?.uri), ["--version"], cwd, () => {});
    this.output.appendLine(result.stdout || result.stderr);
    try { this.output.appendLine(await runSyncTeX(this.configuration(document?.uri).get("synctex.executable", "synctex"), ["--version"], cwd)); }
    catch (error) { this.output.appendLine(String(error)); }
    this.output.show();
  }

  private runProcess(
    executable: string,
    args: string[],
    cwd: string,
    onStart: (child: ChildProcessWithoutNullStreams) => void,
  ): Promise<CommandResult> {
    this.output.appendLine(`cwd: ${cwd}`);
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
    if (error instanceof CheckBlockedError) {
      this.output.appendLine(message);
      const actions = ["Show Problems", "Show Output", ...(error.retry ? ["Build with Warnings Once"] : [])];
      const notification = error.warningsOnly
        ? vscode.window.showWarningMessage(`Tekai: ${message}`, ...actions)
        : vscode.window.showErrorMessage(`Tekai: ${message}`, ...actions);
      void notification.then((choice) => {
        if (choice === "Show Problems") { void vscode.commands.executeCommand("workbench.actions.view.problems"); }
        if (choice === "Show Output") { this.output.show(true); }
        if (choice === "Build with Warnings Once" && error.retry) { this.background(error.retry()); }
      });
      return;
    }
    this.output.appendLine(`error: ${message}`);
    if (error instanceof SyncTeXError && error.details) { this.output.appendLine(error.details); }
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
    this.lintGenerations.clear();
    for (const child of this.lintProcesses.values()) {
      child.kill();
    }
    this.lintProcesses.clear();
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
