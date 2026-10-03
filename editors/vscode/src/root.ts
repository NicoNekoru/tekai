import * as path from "node:path";
import * as vscode from "vscode";
import { parseMagicRoot } from "./protocol";
import { stripComments } from "./tex";

function workspaceFolderFor(uri: vscode.Uri): vscode.WorkspaceFolder | undefined {
  return vscode.workspace.getWorkspaceFolder(uri);
}

function configuredMain(document: vscode.TextDocument): vscode.Uri | undefined {
  const configured = vscode.workspace
    .getConfiguration("tekai", document.uri)
    .get<string>("mainFile", "")
    .trim();
  if (!configured) {
    return undefined;
  }
  const folder = workspaceFolderFor(document.uri);
  const base = folder?.uri.fsPath ?? path.dirname(document.uri.fsPath);
  return vscode.Uri.file(path.isAbsolute(configured) ? configured : path.resolve(base, configured));
}

function magicMain(document: vscode.TextDocument): vscode.Uri | undefined {
  const root = parseMagicRoot(document.getText());
  if (!root) {
    return undefined;
  }
  return vscode.Uri.file(path.resolve(path.dirname(document.uri.fsPath), root));
}

function looksLikeRoot(source: string): boolean {
  const clean = stripComments(source);
  return /\\documentclass(?:\[[^\]]*\])?\s*\{/.test(clean) && /\\begin\s*\{document\}/.test(clean);
}

async function discoverCandidates(document: vscode.TextDocument): Promise<vscode.Uri[]> {
  const folder = workspaceFolderFor(document.uri);
  if (!folder) {
    return [];
  }
  const uris = await vscode.workspace.findFiles(
    new vscode.RelativePattern(folder, "**/*.{tex,ltx}"),
    "**/{.git,target,build,.tekai,.tekai-hmr,.tekai-hmr-warm}/**",
    200,
  );
  const candidates: vscode.Uri[] = [];
  for (const uri of uris) {
    try {
      const open = vscode.workspace.textDocuments.find((item) => item.uri.toString() === uri.toString());
      const source = open?.getText() ?? Buffer.from(await vscode.workspace.fs.readFile(uri)).toString("utf8");
      if (looksLikeRoot(source)) {
        candidates.push(uri);
      }
    } catch {
      // A transiently unreadable candidate should not prevent other roots from being used.
    }
  }
  return candidates;
}

function preferCandidate(candidates: vscode.Uri[], document: vscode.TextDocument): vscode.Uri | undefined {
  if (candidates.length === 1) {
    return candidates[0];
  }
  const sameDirectoryMain = candidates.find(
    (uri) => path.dirname(uri.fsPath) === path.dirname(document.uri.fsPath) && path.basename(uri.fsPath).toLowerCase() === "main.tex",
  );
  if (sameDirectoryMain) {
    return sameDirectoryMain;
  }
  const allMainFiles = candidates.filter((uri) => path.basename(uri.fsPath).toLowerCase() === "main.tex");
  return allMainFiles.length === 1 ? allMainFiles[0] : undefined;
}

export async function resolveMainDocument(
  document: vscode.TextDocument,
  interactive: boolean,
): Promise<vscode.Uri | undefined> {
  const explicit = configuredMain(document) ?? magicMain(document);
  if (explicit) {
    return explicit;
  }
  if (looksLikeRoot(document.getText())) {
    return document.uri;
  }
  const boundary = workspaceFolderFor(document.uri)?.uri.fsPath;
  for (let directory = path.dirname(document.uri.fsPath); ; directory = path.dirname(directory)) {
    const uri = vscode.Uri.file(path.join(directory, "main.tex"));
    try {
      const source = Buffer.from(await vscode.workspace.fs.readFile(uri)).toString("utf8");
      if (looksLikeRoot(source)) { return uri; }
    } catch { /* Keep looking upward. */ }
    if (directory === boundary || path.dirname(directory) === directory) { break; }
  }
  const candidates = await discoverCandidates(document);
  const preferred = preferCandidate(candidates, document);
  if (preferred) {
    return preferred;
  }
  if (!interactive || candidates.length === 0) {
    return undefined;
  }
  const folder = workspaceFolderFor(document.uri);
  const selected = await vscode.window.showQuickPick(
    candidates.map((uri) => ({
      label: folder ? path.relative(folder.uri.fsPath, uri.fsPath) : uri.fsPath,
      description: uri.fsPath,
      uri,
    })),
    { placeHolder: "Select the root TeX document" },
  );
  return selected?.uri;
}

export function commandCwd(main: vscode.Uri): string {
  if (vscode.workspace.getConfiguration("tekai", main).get<string>("workingDirectory", "workspace") === "document") {
    return path.dirname(main.fsPath);
  }
  return workspaceFolderFor(main)?.uri.fsPath ?? path.dirname(main.fsPath);
}
