import { execFile } from "node:child_process";
import { access, stat } from "node:fs/promises";
import * as path from "node:path";

export interface PdfPosition { page: number; x: number; y: number }
export interface SourcePosition { file: string; line: number; column: number }

export class SyncTeXError extends Error {
  constructor(message: string, readonly details?: string) { super(message); }
}

export async function hasSyncTeXMap(pdf: string): Promise<boolean> {
  const stem = pdf.replace(/\.pdf$/i, "");
  for (const suffix of [".synctex.gz", ".synctex"]) {
    try {
      const info = await stat(stem + suffix);
      if (info.isFile() && info.size > 0) { return true; }
    } catch { /* Try the other supported map format. */ }
  }
  return false;
}

export function syncTeXFailure(error: Error & { code?: string | number | null }, stderr: string): SyncTeXError {
  if (error.code === "ENOENT") {
    return new SyncTeXError("SyncTeX executable not found. Set tekai.synctex.executable to your synctex binary.", error.message);
  }
  const detail = stderr.trim() || error.message;
  const firstLine = detail.split(/\r?\n|\busage:/i)[0].trim().slice(0, 300);
  return new SyncTeXError(`SyncTeX failed: ${firstLine || "no location was returned"}. Rebuild the PDF with SyncTeX enabled; see Tekai output for details.`, detail);
}

function values(output: string): Map<string, string> {
  const result = new Map<string, string>();
  for (const line of output.split(/\r?\n/)) {
    const match = /^([A-Za-z]+):(.*)$/.exec(line);
    if (match && !result.has(match[1])) { result.set(match[1], match[2].trim()); }
  }
  return result;
}

export function parseForwardSync(output: string): PdfPosition {
  const fields = values(output);
  const page = Number(fields.get("Page"));
  const x = Number(fields.get("x"));
  const y = Number(fields.get("y"));
  if (!Number.isInteger(page) || page < 1 || !Number.isFinite(x) || !Number.isFinite(y)) {
    throw new Error("SyncTeX found no PDF location. Rebuild with SyncTeX enabled.");
  }
  return { page, x, y };
}

export function parseInverseSync(output: string, cwd: string): SourcePosition {
  const fields = values(output);
  const file = fields.get("Input");
  const line = Number(fields.get("Line"));
  const column = Number(fields.get("Column"));
  if (!file || !Number.isInteger(line) || line < 1) { throw new Error("SyncTeX found no source location at this point."); }
  return { file: path.resolve(cwd, file), line, column: Number.isInteger(column) && column > 0 ? column : 1 };
}

export async function runSyncTeX(executable: string, args: string[], cwd: string): Promise<string> {
  const outputIndex = args.indexOf("-o");
  const output = outputIndex >= 0 ? args[outputIndex + 1] : undefined;
  const pdf = args[0] === "view" ? output : args[0] === "edit" ? output?.match(/^[^:]+:[^:]+:[^:]+:(.+)$/)?.[1] : undefined;
  if (pdf && !await hasSyncTeXMap(path.resolve(cwd, pdf))) {
    throw new SyncTeXError(`No SyncTeX map for ${path.basename(pdf)}. Open its root TeX file and run "Tekai: Build PDF", then use the PDF opened by that build.`, `Missing SyncTeX map beside ${path.resolve(cwd, pdf)}`);
  }
  // GUI applications on macOS often do not inherit the TeX distribution's PATH.
  if (executable === "synctex" && process.platform === "darwin") {
    try { await access("/Library/TeX/texbin/synctex"); executable = "/Library/TeX/texbin/synctex"; } catch { /* Use PATH. */ }
  }
  return new Promise((resolve, reject) => execFile(executable, args, { cwd, timeout: 15000, maxBuffer: 1024 * 1024 }, (error, stdout, stderr) => {
    if (error) { reject(syncTeXFailure(error, stderr)); }
    else { resolve(stdout); }
  }));
}
