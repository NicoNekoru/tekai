export interface BibEntry {
  key: string;
  type: string;
  title: string;
  author: string;
  year: string;
  offset: number;
}

// Preserve offsets so definitions and completion ranges refer to the original text.
export function stripComments(source: string): string {
  return source.replace(/\\[\s\S]|%[^\r\n]*/g, (match) => match.startsWith("%") ? " ".repeat(match.length) : match)
    .replace(/\\begin\{(verbatim\*?|lstlisting|minted)\}[\s\S]*?\\end\{\1\}/g, (match) => match.replace(/[^\r\n]/g, " "));
}

export function parseBibliography(source: string): BibEntry[] {
  const entries: BibEntry[] = [];
  const strings = new Map<string, string>();
  const header = /@([a-z]+)\s*([{(])/gi;
  let match: RegExpExecArray | null;
  while ((match = header.exec(source))) {
    const linePrefix = source.slice(source.lastIndexOf("\n", match.index) + 1, match.index);
    if (/(^|[^\\])%/.test(linePrefix)) { continue; }
    const type = match[1].toLowerCase();
    const close = match[2] === "{" ? "}" : ")";
    let i = header.lastIndex;
    const start = i;
    let depth = 0;
    let quoted = false;
    for (; i < source.length; i++) {
      const c = source[i];
      if (c === "\\") { i++; continue; }
      if (c === '"' && depth === 0) { quoted = !quoted; continue; }
      if (!quoted && depth === 0 && c === close) { break; }
      if (c === "{") { depth++; }
      if (c === "}") { depth--; }
    }
    header.lastIndex = i + 1;
    if (type === "comment" || type === "preamble") { continue; }
    const body = source.slice(start, i);
    const keyMatch = /^\s*([^\s,=]+)\s*,/.exec(body);
    if (type !== "string" && !keyMatch) { continue; }
    const fields = new Map<string, string>();
    const field = /([\w-]+)\s*=\s*/g;
    field.lastIndex = type === "string" ? 0 : keyMatch![0].length;
    let fm: RegExpExecArray | null;
    while ((fm = field.exec(body))) {
      let j = field.lastIndex;
      const pieces: string[] = [];
      do {
        while (/\s/.test(body[j] ?? "") || body[j] === "#") { j++; }
        const opener = body[j];
        if (opener === "{" || opener === '"') {
          const valueStart = ++j;
          let nesting = 0;
          for (; j < body.length; j++) {
            if (body[j] === "\\") { j++; continue; }
            if (nesting === 0 && body[j] === (opener === "{" ? "}" : '"')) { break; }
            if (body[j] === "{") { nesting++; }
            if (body[j] === "}") { nesting--; }
          }
          pieces.push(body.slice(valueStart, j++));
        } else {
          const valueStart = j;
          while (j < body.length && !/[,#\s]/.test(body[j])) { j++; }
          const value = body.slice(valueStart, j);
          pieces.push(strings.get(value.toLowerCase()) ?? value);
        }
        while (/\s/.test(body[j] ?? "")) { j++; }
      } while (body[j] === "#");
      fields.set(fm[1].toLowerCase(), pieces.join(""));
      field.lastIndex = Math.max(j + 1, field.lastIndex);
    }
    if (type === "string") {
      for (const [key, value] of fields) { strings.set(key, value); }
      continue;
    }
    const plain = (value: string | undefined) => (value ?? "").replace(/[{}]/g, "").replace(/\s+/g, " ").trim();
    entries.push({
      key: keyMatch![1], type,
      title: plain(fields.get("title")), author: plain(fields.get("author") ?? fields.get("editor")),
      year: plain(fields.get("year") ?? fields.get("date")),
      offset: start + body.indexOf(keyMatch![1]),
    });
  }
  return entries;
}

export interface TexArgument { command: string; value: string; offset: number }

export function texArguments(source: string): TexArgument[] {
  const result: TexArgument[] = [];
  const clean = stripComments(source);
  const re = /\\([a-zA-Z]+)\*?(?:\s*\[[^\]]*\])*\s*\{([^{}]*)\}/g;
  for (const match of clean.matchAll(re)) {
    result.push({ command: match[1], value: match[2], offset: match.index! + match[0].lastIndexOf("{") + 1 });
  }
  return result;
}

export interface CompletionContext { kind: "citation" | "reference"; start: number; end: number; query: string }

export function completionContext(source: string, offset: number): CompletionContext | undefined {
  const clean = stripComments(source);
  // Match only an unfinished key list, never an earlier command or optional argument.
  const match = /\\([a-zA-Z]*cite[a-zA-Z]*|[cCvV]?(?:eqref|pageref|autoref|nameref|ref|cref)|[cC]refrange)\*?(?:\s*\[[^\]]*\])*\s*\{([^{}]*)$/i.exec(clean.slice(0, offset));
  if (!match) { return undefined; }
  const prefix = match[2].slice(match[2].lastIndexOf(",") + 1);
  const query = prefix.trimStart();
  const start = offset - query.length;
  const rest = /^[^\s,{}]*/.exec(clean.slice(offset))![0];
  return { kind: /cite/i.test(match[1]) ? "citation" : "reference", start, end: offset + rest.length, query };
}

export interface IndexedEntry extends BibEntry { file: string }
export interface LabelEntry { key: string; file: string; offset: number; context: string }
export interface ProjectIndex { citations: IndexedEntry[]; labels: LabelEntry[]; files: string[] }
