import assert from "node:assert/strict";
import * as path from "node:path";
import * as vscode from "vscode";
import { hasSyncTeXMap, parseForwardSync, parseInverseSync, runSyncTeX } from "../synctex";
import { commandCwd } from "../root";

async function eventually(check: () => Promise<boolean>): Promise<void> {
  const deadline = Date.now() + 20000;
  while (!await check()) {
    assert(Date.now() < deadline, "timed out waiting for extension providers/build");
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
}

export async function run(): Promise<void> {
  const root = vscode.workspace.workspaceFolders![0].uri;
  const looseFile = vscode.Uri.file(path.join(path.dirname(root.fsPath), "outside", "main.tex"));
  assert.equal(commandCwd(looseFile), path.dirname(looseFile.fsPath), "loose files must not build into an unrelated workspace");
  const config = vscode.workspace.getConfiguration("tekai");
  await config.update("executable", process.env.TEKAI_EXECUTABLE ?? "tekai", vscode.ConfigurationTarget.Workspace);
  await config.update("synctex.executable", process.env.SYNCTEX_EXECUTABLE ?? "synctex", vscode.ConfigurationTarget.Workspace);
  await config.update("lint.run", "manual", vscode.ConfigurationTarget.Workspace);
  const document = await vscode.workspace.openTextDocument(vscode.Uri.joinPath(root, "chapter.tex"));
  assert.equal(document.languageId, "latex", "VS Code supplies the LaTeX language without Workshop");
  const editor = await vscode.window.showTextDocument(document);
  const extension = vscode.extensions.getExtension("tekai.tekai")!;
  await extension.activate();
  const commands = await vscode.commands.getCommands();
  for (const command of ["tekai.check", "tekai.forwardSync", "tekai.toggleWatch", "tekai.health"]) { assert(commands.includes(command)); }
  const citationPosition = document.positionAt(document.getText().indexOf("knuth") + 2);
  const workshop = vscode.extensions.getExtension("james-yu.latex-workshop");
  if (process.env.WORKSHOP_EXTENSION_PATH) {
    assert(workshop, "companion extension is enabled in the isolated profile");
    await workshop.activate();
    await eventually(async () => {
      const result = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, citationPosition);
      return result.items.some((item) => (typeof item.label === "string" ? item.label : item.label.label) === "knuth1984");
    });
    const result = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, citationPosition);
    const matches = result.items.filter((item) => (typeof item.label === "string" ? item.label : item.label.label) === "knuth1984");
    assert.equal(matches.length, 1, "auto mode must not duplicate Workshop citation suggestions");
    assert.notEqual(matches[0].detail, "book · refs.bib", "Workshop, not Tekai, supplies citations");
    const definitions = await vscode.commands.executeCommand<vscode.Location[]>("vscode.executeDefinitionProvider", document.uri, citationPosition);
    assert.equal(definitions.length, 1, "auto mode must not duplicate definitions");
    const symbols = await vscode.commands.executeCommand<vscode.DocumentSymbol[]>("vscode.executeDocumentSymbolProvider", document.uri);
    assert.equal(symbols.filter((symbol) => symbol.name.includes("References")).length, 1, "auto mode must not duplicate outline entries");
    if (process.env.TEKAI_EXECUTABLE) {
      await vscode.commands.executeCommand("latex-workshop.recipes", "Tekai");
      await eventually(async () => {
        try { return (await vscode.workspace.fs.stat(vscode.Uri.joinPath(root, "build/main.synctex.gz"))).size > 0; }
        catch { return false; }
      });
    }
    console.log("PASS Workshop coexistence: single citation/definition/outline providers and Tekai build recipe");
  } else {
  const completion = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, citationPosition);
  const citation = completion.items.find((item) => (typeof item.label === "string" ? item.label : item.label.label) === "knuth1984");
  assert(citation, "project bibliography completion is registered");
  assert.equal(citation.insertText, "knuth1984");
  assert.match(citation.filterText!, /Donald Knuth.*1984.*TeXbook/);
  const definitions = await vscode.commands.executeCommand<vscode.Location[]>("vscode.executeDefinitionProvider", document.uri, citationPosition);
  assert.equal(definitions[0].uri.fsPath, vscode.Uri.joinPath(root, "refs.bib").fsPath);
  const refPosition = document.positionAt(document.getText().indexOf("sec:intro") + 3);
  const references = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, refPosition);
  assert(references.items.some((item) => item.label === "sec:intro"));
  const bib = await vscode.workspace.openTextDocument(vscode.Uri.joinPath(root, "refs.bib"));
  const edit = new vscode.WorkspaceEdit();
  edit.insert(bib.uri, new vscode.Position(0, 0), "@article{unsaved, title={Unsaved citation}}\n");
  await vscode.workspace.applyEdit(edit);
  const updated = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, citationPosition);
  assert(updated.items.some((item) => typeof item.label !== "string" && item.label.label === "unsaved"), "unsaved bibliography edits invalidate the index");
  const snippetPosition = document.positionAt(document.getText().indexOf("section") + 3);
  const snippets = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, snippetPosition);
  assert(snippets.items.some((item) => item.label === "section" && item.detail?.startsWith("Tekai:")), "native snippets are registered");
  await config.update("languageFeatures", "off", vscode.ConfigurationTarget.Workspace);
  await eventually(async () => {
    const disabled = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, citationPosition);
    return !disabled.items.some((item) => (typeof item.label === "string" ? item.label : item.label.label) === "knuth1984");
  });
  const disabledDefinitions = await vscode.commands.executeCommand<vscode.Location[]>("vscode.executeDefinitionProvider", document.uri, citationPosition);
  assert.equal(disabledDefinitions.length, 0, "off mode also disposes definitions");
  const disabledSnippets = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, snippetPosition);
  assert(!disabledSnippets.items.some((item) => item.detail?.startsWith("Tekai:")), "off mode also disposes snippets");
  await config.update("languageFeatures", "auto", vscode.ConfigurationTarget.Workspace);
  await eventually(async () => {
    const restored = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", document.uri, citationPosition);
    return restored.items.some((item) => (typeof item.label === "string" ? item.label : item.label.label) === "knuth1984");
  });
  console.log("PASS language registration, citation metadata, definitions, labels, unsaved bibliography updates");
  }

  if (process.env.TEKAI_EXECUTABLE && process.env.SYNCTEX_EXECUTABLE) {
    // Strict checks should block on warnings, then explicit allow-warnings should build.
    await assert.rejects(Promise.resolve(vscode.commands.executeCommand("tekai.check")), /Check blocked by lint with 2 warnings\. Compilation did not run\..*chapter\.tex:5:\d+ \[math\/inline-dollar\]/);
    const diagnostics = vscode.languages.getDiagnostics().flatMap(([, entries]) => entries).filter((item) => item.source === "tekai");
    assert(diagnostics.length > 0, "check publishes its blocking diagnostics");
    await config.update("check.extraArgs", ["--fail-on-warnings"], vscode.ConfigurationTarget.Workspace);
    await vscode.commands.executeCommand("tekai.checkAllowWarnings");
    assert.deepEqual(vscode.workspace.getConfiguration("tekai").get("check.extraArgs"), ["--fail-on-warnings"], "one-time override leaves strict settings unchanged");
    await assert.rejects(Promise.resolve(vscode.commands.executeCommand("tekai.check")), /Check blocked by lint with 2 warnings/, "the next normal check is strict again");
    const warnedSource = document.getText();
    const warnedFailure = new vscode.WorkspaceEdit();
    warnedFailure.insert(document.uri, document.positionAt(warnedSource.length), "\n\\UndefinedTekaiWarningTest\n");
    await vscode.workspace.applyEdit(warnedFailure);
    await assert.rejects(Promise.resolve(vscode.commands.executeCommand("tekai.checkAllowWarnings")), /Undefined control sequence/, "allowed lint warnings must not hide the compiler error");
    const restoreWarning = new vscode.WorkspaceEdit();
    restoreWarning.replace(document.uri, new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length)), warnedSource);
    await vscode.workspace.applyEdit(restoreWarning);
    await config.update("check.extraArgs", ["--allow-warnings"], vscode.ConfigurationTarget.Workspace);
    await vscode.commands.executeCommand("tekai.check");
    const pdf = vscode.Uri.joinPath(root, "build/main.pdf");
    assert((await vscode.workspace.fs.stat(pdf)).size > 0);
    assert((await vscode.workspace.fs.stat(vscode.Uri.joinPath(root, "build/main.synctex.gz"))).size > 0);
    const forward = parseForwardSync(await runSyncTeX(process.env.SYNCTEX_EXECUTABLE,
      ["view", "-i", `5:1:${path.join(root.fsPath, "main.tex")}`, "-o", pdf.fsPath], root.fsPath));
    const inverse = parseInverseSync(await runSyncTeX(process.env.SYNCTEX_EXECUTABLE,
      ["edit", "-o", `${forward.page}:${forward.x}:${forward.y}:${pdf.fsPath}`], root.fsPath), root.fsPath);
    assert.equal(inverse.file, path.join(root.fsPath, "main.tex"));
    assert(inverse.line >= 1);
    await vscode.window.showTextDocument(document, vscode.ViewColumn.One);
    editor.selection = new vscode.Selection(citationPosition, citationPosition);
    await vscode.commands.executeCommand("tekai.forwardSync");
    assert(vscode.window.tabGroups.all.some((group) => group.tabs.some((tab) => tab.input instanceof vscode.TabInputCustom && tab.input.viewType === "tekai.pdfPreview")), "PDF custom editor opened");
    const fix = new vscode.WorkspaceEdit();
    const warningOffset = document.getText().indexOf("$x=1$");
    fix.replace(document.uri, new vscode.Range(document.positionAt(warningOffset), document.positionAt(warningOffset + 5)), "\\(x=1\\)");
    await vscode.workspace.applyEdit(fix);
    await config.update("check.extraArgs", [], vscode.ConfigurationTarget.Workspace);
    await vscode.commands.executeCommand("tekai.check");
    assert.equal(vscode.languages.getDiagnostics().flatMap(([, entries]) => entries).filter((item) => item.source === "tekai").length, 0,
      "a clean explicit check clears old diagnostics");
    const cleanSource = document.getText();
    const broken = new vscode.WorkspaceEdit();
    broken.insert(document.uri, document.positionAt(cleanSource.length), "\n\\UndefinedTekaiTestCommand\n");
    await vscode.workspace.applyEdit(broken);
    await assert.rejects(Promise.resolve(vscode.commands.executeCommand("tekai.check")), /chapter\.tex:\d+: Undefined control sequence/, "compiler failures show the actual source error");
    const compileError = vscode.languages.getDiagnostics(document.uri).find((item) => item.code === "tex/compile");
    assert(compileError, "compiler failures appear in Problems on the included source");
    assert.equal(compileError.range.start.line, document.positionAt(document.getText().indexOf("\\UndefinedTekaiTestCommand")).line);
    const repair = new vscode.WorkspaceEdit();
    repair.replace(document.uri, new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length)), cleanSource);
    await vscode.workspace.applyEdit(repair);
    await vscode.commands.executeCommand("tekai.check");
    assert(!vscode.languages.getDiagnostics().flatMap(([, entries]) => entries).some((item) => item.code === "tex/compile"), "a successful rebuild clears compiler Problems");
    console.log("PASS compiler error location, Problems diagnostics, failed preview reload and repaired build");
    await vscode.commands.executeCommand("tekai.watch");
    await vscode.commands.executeCommand("tekai.stop");
    console.log("PASS warning-only lint message, one-time warning override, strict settings preserved, compiler errors with warnings, exact build, SyncTeX round trip, PDF preview, watch start/stop");

    // A nested paper's relative output path must resolve where its CLI build
    // runs, not silently move to the repository root when opened in VS Code.
    const nested = vscode.Uri.joinPath(root, "nested");
    await vscode.workspace.fs.createDirectory(nested);
    const nestedMain = vscode.Uri.joinPath(nested, "main.tex");
    await vscode.workspace.fs.writeFile(nestedMain, Buffer.from("\\documentclass{article}\n\\begin{document}\nNested paper.\n\\end{document}\n"));
    await vscode.workspace.fs.writeFile(vscode.Uri.joinPath(nested, "tekai.toml"), Buffer.from('[build]\nout_dir = "output/pdf"\nsynctex = false\n'));
    assert.equal(commandCwd(nestedMain), root.fsPath, "workspace remains the compatibility default");
    await config.update("workingDirectory", "document", vscode.ConfigurationTarget.Workspace);
    assert.equal(commandCwd(nestedMain), nested.fsPath);
    const nestedDocument = await vscode.workspace.openTextDocument(nestedMain);
    const nestedEditor = await vscode.window.showTextDocument(nestedDocument, vscode.ViewColumn.One);
    nestedEditor.selection = new vscode.Selection(2, 0, 2, 0);
    await vscode.commands.executeCommand("tekai.build");
    const nestedPdf = vscode.Uri.joinPath(nested, "output/pdf/main.pdf");
    assert((await vscode.workspace.fs.stat(nestedPdf)).size > 0);
    assert(await hasSyncTeXMap(nestedPdf.fsPath), "--synctex overrides the project's old false setting");
    await vscode.commands.executeCommand("tekai.preview");
    await vscode.commands.executeCommand("tekai.check");
    // Explicit config paths stay workspace-relative even in document-CWD mode.
    await config.update("configFile", "nested/tekai.toml", vscode.ConfigurationTarget.Workspace);
    const nestedMap = vscode.Uri.joinPath(nested, "output/pdf/main.synctex.gz");
    await vscode.workspace.fs.delete(nestedMap);
    await assert.rejects(runSyncTeX(process.env.SYNCTEX_EXECUTABLE, ["edit", "-o", `1:40:50:${nestedPdf.fsPath}`], nested.fsPath), /No SyncTeX map/);
    await vscode.commands.executeCommand("tekai.forwardSync");
    assert(await hasSyncTeXMap(nestedPdf.fsPath), "forward sync rebuilds a missing map into the same output directory");
    console.log("PASS nested-paper working directory, configured output path, missing-map error and rebuild recovery");
    await testProjectConfigs(root, config);
  }
}

async function testProjectConfigs(root: vscode.Uri, config: vscode.WorkspaceConfiguration): Promise<void> {
  const directory = vscode.Uri.joinPath(root, "config-papers");
  const currentFile = vscode.Uri.joinPath(directory, "current/sections/chapter.tex");
  const oldFile = vscode.Uri.joinPath(directory, "old/chapter.tex");
  const sharedConfig = vscode.Uri.joinPath(directory, "tekai.toml");
  const oldConfig = vscode.Uri.joinPath(directory, "old/tekai.toml");
  await vscode.workspace.fs.createDirectory(vscode.Uri.joinPath(directory, "current/sections"));
  await vscode.workspace.fs.createDirectory(vscode.Uri.joinPath(directory, "old"));
  const source = Buffer.from("\\begin{itemize}\n\t\\item Inline $x=1$.\n\\end{itemize}\n");
  await vscode.workspace.fs.writeFile(currentFile, source);
  await vscode.workspace.fs.writeFile(oldFile, source);
  await vscode.workspace.fs.writeFile(sharedConfig, Buffer.from('[lint]\nindent_style = "tabs"\nindent_size = 1\n[lint.rules]\n"math/inline-dollar" = "off"\n'));
  await vscode.workspace.fs.writeFile(oldConfig, Buffer.from('[lint]\nindent_style = "spaces"\n[lint.rules]\n"math/inline-dollar" = "error"\n'));
  await config.update("configFile", "", vscode.ConfigurationTarget.Workspace);
  const currentDocument = await vscode.workspace.openTextDocument(currentFile);
  await vscode.window.showTextDocument(currentDocument, vscode.ViewColumn.One);
  await vscode.commands.executeCommand("tekai.lint");
  const diagnostics = (uri: vscode.Uri) => vscode.languages.getDiagnostics(uri).filter((entry) => entry.source === "tekai");
  assert.equal(diagnostics(currentFile).length, 0, "a nested source inherits tabs and disabled rules from its nearest ancestor config");
  const oldDocument = await vscode.workspace.openTextDocument(oldFile);
  await vscode.window.showTextDocument(oldDocument, vscode.ViewColumn.One);
  await vscode.commands.executeCommand("tekai.lint");
  assert(diagnostics(oldFile).some((entry) => entry.code === "indent/tabs"));
  assert(diagnostics(oldFile).some((entry) => entry.code === "math/inline-dollar" && entry.severity === vscode.DiagnosticSeverity.Error));
  await vscode.commands.executeCommand("tekai.lintWorkspace");
  assert.equal(diagnostics(currentFile).length, 0, "workspace lint does not leak another paper's rules");
  assert(diagnostics(oldFile).some((entry) => entry.code === "math/inline-dollar" && entry.severity === vscode.DiagnosticSeverity.Error));
  await config.update("configFile", "config-papers/tekai.toml", vscode.ConfigurationTarget.Workspace);
  await vscode.commands.executeCommand("tekai.lint");
  assert.equal(diagnostics(oldFile).length, 0, "an explicit workspace-relative config overrides nearest-file discovery and clears obsolete Problems");
  await config.update("configFile", oldConfig.fsPath, vscode.ConfigurationTarget.Workspace);
  await vscode.window.showTextDocument(currentDocument, vscode.ViewColumn.One);
  await vscode.commands.executeCommand("tekai.lint");
  assert(diagnostics(currentFile).some((entry) => entry.code === "indent/tabs"), "absolute config overrides also apply");
  await config.update("configFile", "", vscode.ConfigurationTarget.Workspace);
  await vscode.commands.executeCommand("tekai.lint");
  assert.equal(diagnostics(currentFile).length, 0, "removing the override restores the proper paper's config");
  await config.update("lint.run", "onSave", vscode.ConfigurationTarget.Workspace);
  const toml = await vscode.workspace.openTextDocument(sharedConfig);
  const edit = new vscode.WorkspaceEdit();
  edit.replace(toml.uri, new vscode.Range(toml.positionAt(0), toml.positionAt(toml.getText().length)), '[lint]\nindent_style = "tabs"\nindent_size = 1\n[lint.rules]\n"math/inline-dollar" = "error"\n');
  await vscode.workspace.applyEdit(edit);
  await toml.save();
  await eventually(async () => diagnostics(currentFile).some((entry) => entry.code === "math/inline-dollar" && entry.severity === vscode.DiagnosticSeverity.Error));
  await config.update("lint.run", "manual", vscode.ConfigurationTarget.Workspace);
  await vscode.workspace.fs.writeFile(sharedConfig, Buffer.from("[lint\ninvalid TOML\n"));
  await assert.rejects(Promise.resolve(vscode.commands.executeCommand("tekai.lint")), /configuration or source-file errors/, "invalid config must fail, not silently use defaults");
  await vscode.workspace.fs.writeFile(sharedConfig, Buffer.from('[lint.rules]\n"math/inline-dollar" = "off"\n'));
  await vscode.workspace.fs.delete(oldConfig);
  await vscode.window.showTextDocument(oldDocument, vscode.ViewColumn.One);
  await vscode.commands.executeCommand("tekai.lint");
  assert(!diagnostics(oldFile).some((entry) => entry.code === "math/inline-dollar"), "deleting a nested config falls back to the parent, without a stale cache");
  console.log("PASS per-paper TOML inheritance, nested overrides, workspace lint grouping, explicit paths, config-save refresh, invalid config errors, and deleted configs");
}
