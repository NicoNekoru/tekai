import * as path from "node:path";

export interface TexError { file: string; line: number; message: string }

// Direct TeX runs resolve included sources from the main document's directory,
// even when Tekai itself runs from the workspace directory.
export function parseTexErrors(stderr: string, documentDirectory: string): TexError[] {
  const errors: TexError[] = [];
  for (const line of stderr.split(/\r?\n/)) {
    const match = /^(.+\.(?:tex|ltx|sty|cls|aux|bbl)):(\d+):\s*(.+)$/i.exec(line);
    if (!match || Number(match[2]) < 1 || match[3].includes("==> Fatal error")) { continue; }
    errors.push({ file: path.resolve(documentDirectory, match[1]), line: Number(match[2]), message: match[3] });
  }
  return errors;
}
