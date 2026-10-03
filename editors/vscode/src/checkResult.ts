import * as path from "node:path";
import { absoluteReportedPath, LintDiagnostic, LintReport } from "./protocol";

const count = (value: number, noun: string) => `${value} ${noun}${value === 1 ? "" : "s"}`;

export function lintSummary(report: LintReport): string {
  return `${count(report.error_count, "error")} and ${count(report.warning_count, "warning")}`;
}

function location(item: LintDiagnostic, cwd: string, relative = false): string {
  const filename = absoluteReportedPath(item.path, cwd);
  return `${relative ? path.relative(cwd, filename) : filename}:${item.line}:${item.column}`;
}

export function formatLintOutput(report: LintReport, cwd: string): string {
  const lines = [`Lint found ${lintSummary(report)}.`];
  for (const item of report.diagnostics) {
    lines.push(`${location(item, cwd)} ${item.severity} [${item.rule}] ${item.message}`);
    if (item.help) { lines.push(`  ${item.help}`); }
  }
  return lines.join("\n");
}

export function lintBlocksCheck(report: LintReport, args: string[]): boolean {
  return report.error_count > 0 || (report.warning_count > 0 && !args.includes("--allow-warnings"));
}

export function describeBlockedCheck(report: LintReport, cwd: string): string {
  const first = report.diagnostics.find((item) => item.severity === "error") ?? report.diagnostics[0];
  const reason = report.error_count ? lintSummary(report) : count(report.warning_count, "warning");
  const detail = first ? ` First issue at ${location(first, cwd, true)} [${first.rule}] ${first.message}.` : "";
  return `Check blocked by lint with ${reason}. Compilation did not run.${report.error_count ? "" : " Warnings block checks by default."}${detail}`;
}

export class CheckBlockedError extends Error {
  constructor(message: string, readonly warningsOnly: boolean, readonly retry?: () => Promise<void>) {
    super(message);
    this.name = "CheckBlockedError";
  }
}

export function allowWarningsOnce(args: string[]): string[] {
  return [...args.filter((arg) => arg !== "--fail-on-warnings" && arg !== "--allow-warnings"), "--allow-warnings"];
}
