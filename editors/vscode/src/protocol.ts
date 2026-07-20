import * as path from "node:path";

export interface LintDiagnostic {
  path: string;
  line: number;
  column: number;
  severity: "warning" | "error";
  rule: string;
  message: string;
  help?: string | null;
}

export interface LintReport {
  diagnostics: LintDiagnostic[];
  error_count: number;
  warning_count: number;
}

export interface BuildReport {
  pdf_path?: string | null;
  elapsed_ms: number;
  skipped: boolean;
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

export function parseLintReport(stdout: string): LintReport {
  const value: unknown = JSON.parse(stdout);
  if (!isObject(value) || !Array.isArray(value.diagnostics)) {
    throw new Error("Tekai returned an invalid lint report");
  }
  const diagnostics = value.diagnostics.map((entry): LintDiagnostic => {
    if (
      !isObject(entry) ||
      typeof entry.path !== "string" ||
      typeof entry.line !== "number" ||
      typeof entry.column !== "number" ||
      (entry.severity !== "warning" && entry.severity !== "error") ||
      typeof entry.rule !== "string" ||
      typeof entry.message !== "string"
    ) {
      throw new Error("Tekai returned an invalid diagnostic");
    }
    return {
      path: entry.path,
      line: entry.line,
      column: entry.column,
      severity: entry.severity,
      rule: entry.rule,
      message: entry.message,
      help: typeof entry.help === "string" ? entry.help : null,
    };
  });
  return {
    diagnostics,
    error_count: typeof value.error_count === "number" ? value.error_count : 0,
    warning_count: typeof value.warning_count === "number" ? value.warning_count : 0,
  };
}

export function parseBuildReport(stdout: string): BuildReport {
  const value: unknown = JSON.parse(stdout);
  if (!isObject(value) || typeof value.elapsed_ms !== "number") {
    throw new Error("Tekai returned an invalid build report");
  }
  return {
    pdf_path: typeof value.pdf_path === "string" ? value.pdf_path : null,
    elapsed_ms: value.elapsed_ms,
    skipped: value.skipped === true,
  };
}

export function parseMagicRoot(source: string): string | undefined {
  const match = source
    .split(/\r?\n/, 50)
    .join("\n")
    .match(/^\s*%\s*!\s*TEX\s+root\s*=\s*(.+?)\s*$/im);
  if (!match) {
    return undefined;
  }
  return match[1].trim().replace(/^(["'])(.*)\1$/, "$2");
}

export function parseBuiltPdf(line: string): string | undefined {
  const match = line.match(/(?:built|cached)\s+(.+\.pdf)\s+in\s+[\d.]+(?:ns|us|µs|ms|s)/i);
  return match?.[1].trim();
}

export function absoluteReportedPath(reportedPath: string, cwd: string): string {
  return path.isAbsolute(reportedPath) ? path.normalize(reportedPath) : path.resolve(cwd, reportedPath);
}
