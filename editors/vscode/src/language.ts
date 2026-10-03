import * as path from "node:path";
import * as vscode from "vscode";
import { indexProject } from "./project";
import { completionContext, ProjectIndex, stripComments } from "./tex";
import { resolveMainDocument } from "./root";

export const texSelector: vscode.DocumentSelector = [{ scheme: "file", language: "latex" }, { scheme: "file", language: "tex" }];

// Workshop supplies the richer editor support when present. Do not register two
// competing sets of completions, definitions, hover cards, snippets, or outlines.
export function registerLanguageFeatures(extensionUri: vscode.Uri): vscode.Disposable {
  let features: vscode.Disposable | undefined;
  const update = () => {
    const mode = vscode.workspace.getConfiguration("tekai").get<string>("languageFeatures", "auto");
    const enabled = mode === "native" || (mode === "auto" && !vscode.extensions.getExtension("james-yu.latex-workshop"));
    if (enabled && !features) { features = createLanguageFeatures(extensionUri); }
    else if (!enabled && features) { features.dispose(); features = undefined; }
  };
  update();
  return vscode.Disposable.from(
    vscode.workspace.onDidChangeConfiguration((event) => { if (event.affectsConfiguration("tekai.languageFeatures")) { update(); } }),
    vscode.extensions.onDidChange(update),
    new vscode.Disposable(() => features?.dispose()),
  );
}

function createLanguageFeatures(extensionUri: vscode.Uri): vscode.Disposable {
  let generation = 0;
  const cache = new Map<string, { generation: number; result: Promise<ProjectIndex> }>();
  const invalidate = () => { generation++; cache.clear(); };
  const watcher = vscode.workspace.createFileSystemWatcher("**/*.{tex,ltx,bib,cls,sty}");
  const subscriptions = [watcher, watcher.onDidChange(invalidate), watcher.onDidCreate(invalidate), watcher.onDidDelete(invalidate),
    vscode.workspace.onDidChangeTextDocument((e) => { if (/\.(tex|ltx|bib|cls|sty)$/i.test(e.document.fileName)) { invalidate(); } }),
    vscode.workspace.onDidChangeConfiguration(invalidate)];
  const read = async (file: string): Promise<string> => {
    const open = vscode.workspace.textDocuments.find((document) => document.uri.fsPath === file);
    return open ? open.getText() : Buffer.from(await vscode.workspace.fs.readFile(vscode.Uri.file(file))).toString("utf8");
  };
  async function project(document: vscode.TextDocument): Promise<ProjectIndex> {
    const main = await resolveMainDocument(document, false) ?? document.uri;
    let entry = cache.get(main.fsPath);
    if (!entry || entry.generation !== generation) {
      entry = { generation, result: indexProject(main.fsPath, read) };
      cache.set(main.fsPath, entry);
    }
    try { return await entry.result; } catch { cache.delete(main.fsPath); return { citations: [], labels: [], files: [] }; }
  }
  const provider: vscode.CompletionItemProvider = {
    async provideCompletionItems(document, position, token) {
      if (!vscode.workspace.getConfiguration("tekai", document.uri).get("completion.enabled", true)) { return; }
      const context = completionContext(document.getText(), document.offsetAt(position));
      if (!context) { return; }
      const index = await project(document);
      if (token.isCancellationRequested) { return; }
      const range = new vscode.Range(document.positionAt(context.start), document.positionAt(context.end));
      return (context.kind === "citation" ? index.citations : index.labels).map((entry) => {
        const item = new vscode.CompletionItem(entry.key, vscode.CompletionItemKind.Reference);
        item.range = range;
        item.insertText = entry.key;
        if ("title" in entry) {
          item.label = { label: entry.key, description: [entry.author, entry.year, entry.title].filter(Boolean).join(" · ") };
          item.filterText = `${entry.key} ${entry.author} ${entry.year} ${entry.title}`;
          item.detail = `${entry.type} · ${path.basename(entry.file)}`;
          item.documentation = new vscode.MarkdownString().appendText([entry.title, entry.author, entry.year].filter(Boolean).join("\n\n"));
        } else {
          item.detail = path.basename(entry.file);
          item.documentation = new vscode.MarkdownString().appendCodeblock(entry.context, "latex");
        }
        return item;
      });
    },
  };
  interface Snippet { prefix: string | string[]; body: string | string[]; description?: string }
  let snippets: Promise<Record<string, Snippet>> | undefined;
  const snippetProvider: vscode.CompletionItemProvider = {
    async provideCompletionItems(document, position) {
      if (completionContext(document.getText(), document.offsetAt(position))) { return; }
      snippets ??= Promise.resolve(vscode.workspace.fs.readFile(vscode.Uri.joinPath(extensionUri, "snippets/latex.json")))
        .then((bytes) => JSON.parse(Buffer.from(bytes).toString("utf8")) as Record<string, Snippet>);
      const word = document.getWordRangeAtPosition(position, /\\?[a-zA-Z]+/);
      return Object.entries(await snippets).flatMap(([name, snippet]) =>
        (Array.isArray(snippet.prefix) ? snippet.prefix : [snippet.prefix]).map((prefix) => {
          const item = new vscode.CompletionItem(prefix, vscode.CompletionItemKind.Snippet);
          item.insertText = new vscode.SnippetString(Array.isArray(snippet.body) ? snippet.body.join("\n") : snippet.body);
          item.range = word;
          item.detail = `Tekai: ${snippet.description ?? name}`;
          return item;
        }));
    },
  };
  async function lookup(document: vscode.TextDocument, position: vscode.Position) {
    const source = document.getText();
    const context = completionContext(source, document.offsetAt(position));
    if (!context) { return; }
    const key = source.slice(context.start, context.end).trim();
    const index = await project(document);
    return (context.kind === "citation" ? index.citations : index.labels).find((entry) => entry.key === key);
  }
  subscriptions.push(vscode.languages.registerCompletionItemProvider(texSelector, provider, "{", ","),
    vscode.languages.registerCompletionItemProvider(texSelector, snippetProvider, "\\"),
    vscode.languages.registerDefinitionProvider(texSelector, {
      async provideDefinition(document, position) {
        const entry = await lookup(document, position);
        if (!entry) { return; }
        const target = await vscode.workspace.openTextDocument(vscode.Uri.file(entry.file));
        return new vscode.Location(target.uri, target.positionAt(entry.offset));
      },
    }),
    vscode.languages.registerHoverProvider(texSelector, {
      async provideHover(document, position) {
        const entry = await lookup(document, position);
        if (!entry) { return; }
        return new vscode.Hover(new vscode.MarkdownString().appendText("title" in entry
          ? [entry.key, entry.title, entry.author, entry.year].filter(Boolean).join("\n\n") : entry.context));
      },
    }),
    vscode.languages.registerDocumentSymbolProvider(texSelector, {
      provideDocumentSymbols(document) {
        const symbols: vscode.DocumentSymbol[] = [];
        const re = /\\(part|chapter|section|subsection|subsubsection|paragraph)\*?(?:\[[^\]]*\])?\s*\{([^{}]+)\}/g;
        for (const match of stripComments(document.getText()).matchAll(re)) {
          const range = new vscode.Range(document.positionAt(match.index!), document.positionAt(match.index! + match[0].length));
          symbols.push(new vscode.DocumentSymbol(match[2], match[1], vscode.SymbolKind.Namespace, range, range));
        }
        return symbols;
      },
    }));
  return vscode.Disposable.from(...subscriptions);
}
