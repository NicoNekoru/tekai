import assert from "node:assert/strict";
import * as path from "node:path";
import { test } from "node:test";
import { allowWarningsOnce, describeBlockedCheck, formatLintOutput, lintBlocksCheck } from "../checkResult";
import { LintReport } from "../protocol";

const warnings: LintReport = {
  diagnostics: Array.from({ length: 6 }, (_, index) => ({ path: "main.tex", line: 33 + index, column: 1,
    severity: "warning", rule: "indent/size", message: "expected 0 leading tabs, found 1", help: "Indent nested environments with one tab per level." })),
  error_count: 0, warning_count: 6,
};

test("warning-only checks explain the lint gate and the first source location", () => {
  const message = describeBlockedCheck(warnings, path.resolve("paper"));
  assert.match(message, /6 warnings/);
  assert.match(message, /Compilation did not run/);
  assert.match(message, /Warnings block checks by default/);
  assert.match(message, /main\.tex:33:1 \[indent\/size\] expected 0 leading tabs, found 1/);
  assert(lintBlocksCheck(warnings, []));
  assert(!lintBlocksCheck(warnings, ["--allow-warnings"]), "allowed warnings must not mask a compiler failure");
});

test("output lists all diagnostics with absolute clickable locations, rules and help", () => {
  const cwd = path.resolve("paper with spaces");
  const output = formatLintOutput(warnings, cwd);
  assert.match(output, /0 errors and 6 warnings/);
  for (let line = 33; line <= 38; line++) { assert(output.includes(`${path.join(cwd, "main.tex")}:${line}:1 warning [indent/size]`)); }
  assert.equal(output.match(/Indent nested environments/g)?.length, 6);
});

test("lint errors remain blocking and appear before warnings in the summary", () => {
  const report: LintReport = { diagnostics: [...warnings.diagnostics, { path: "section.tex", line: 2, column: 3, severity: "error", rule: "env/unclosed", message: "unclosed environment" }], error_count: 1, warning_count: 6 };
  assert(lintBlocksCheck(report, ["--allow-warnings"]));
  assert.match(describeBlockedCheck(report, "/paper"), /1 error and 6 warnings/);
  assert.match(describeBlockedCheck(report, "/paper"), /section\.tex:2:3/);
  assert.doesNotMatch(describeBlockedCheck(report, "/paper"), /Warnings block checks by default/);
});

test("allow warnings once replaces conflicting flags without mutating settings", () => {
  const args = ["check", "main.tex", "--fail-on-warnings", "--force", "--allow-warnings"];
  assert.deepEqual(allowWarningsOnce(args), ["check", "main.tex", "--force", "--allow-warnings"]);
  assert.equal(args[2], "--fail-on-warnings");
  assert(!lintBlocksCheck({ diagnostics: [], error_count: 0, warning_count: 0 }, []));
});
