import * as path from "node:path";
import { parseBibliography, ProjectIndex, texArguments } from "./tex";

// Reading is injected so unsaved editor buffers participate in the same index as disk files.
export async function indexProject(main: string, read: (file: string) => Promise<string>): Promise<ProjectIndex> {
  const result: ProjectIndex = { citations: [], labels: [], files: [] };
  const visited = new Set<string>();
  const bibs = new Set<string>();
  const root = path.dirname(main);
  async function resolve(value: string, from: string, extension: string): Promise<[string, string] | undefined> {
    if (!value || /[\\#\n]/.test(value) || /^https?:/.test(value)) { return undefined; }
    const name = path.extname(value) ? value : value + extension;
    for (const base of new Set([root, path.dirname(from)])) {
      const file = path.resolve(base, name);
      try { return [file, await read(file)]; } catch { /* Optional/missing dependencies are not fatal. */ }
    }
    return undefined;
  }
  async function visit(file: string, source: string): Promise<void> {
    if (visited.has(file) || visited.size >= 1000) { return; }
    visited.add(file);
    result.files.push(file);
    for (const arg of texArguments(source)) {
      if (["input", "include", "subfile"].includes(arg.command)) {
        const child = await resolve(arg.value.trim(), file, ".tex");
        if (child) { await visit(...child); }
      } else if (["bibliography", "addbibresource", "addglobalbib", "addsectionbib"].includes(arg.command)) {
        for (const name of arg.value.split(",")) {
          const bib = await resolve(name.trim(), file, ".bib");
          if (!bib || bibs.has(bib[0])) { continue; }
          bibs.add(bib[0]);
          result.files.push(bib[0]);
          result.citations.push(...parseBibliography(bib[1]).map((entry) => ({ ...entry, file: bib[0] })));
        }
      } else if (arg.command === "label") {
        result.labels.push({ key: arg.value, file, offset: arg.offset, context: source.slice(Math.max(0, arg.offset - 100), arg.offset + 100).trim() });
      } else if (arg.command === "bibitem") {
        result.citations.push({ key: arg.value, type: "bibitem", title: source.slice(arg.offset + arg.value.length + 1).split(/\\bibitem|\\end\{thebibliography\}/)[0].trim(), author: "", year: "", file, offset: arg.offset });
      }
    }
  }
  await visit(path.resolve(main), await read(main));
  result.citations = [...new Map(result.citations.map((entry) => [entry.key, entry])).values()];
  return result;
}
