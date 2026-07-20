import assert from "node:assert/strict";
import test from "node:test";
import { parseBuildReport, parseBuiltPdf, parseLintReport, parseMagicRoot } from "../protocol";

test("parses Tekai lint diagnostics", () => {
  const report = parseLintReport(JSON.stringify({
    diagnostics: [{
      path: "paper.tex",
      line: 3,
      column: 7,
      severity: "warning",
      rule: "math/inline-dollar",
      message: "prefer \\( ... \\)",
      help: null,
    }],
    error_count: 0,
    warning_count: 1,
  }));
  assert.equal(report.diagnostics[0].line, 3);
  assert.equal(report.diagnostics[0].rule, "math/inline-dollar");
});

test("parses build reports", () => {
  const report = parseBuildReport('{"pdf_path":"build/main.pdf","elapsed_ms":12.5,"skipped":false}');
  assert.equal(report.pdf_path, "build/main.pdf");
  assert.equal(report.elapsed_ms, 12.5);
});

test("finds TeX root magic comments", () => {
  assert.equal(parseMagicRoot("% !TEX root = ../main.tex\nChapter"), "../main.tex");
  assert.equal(parseMagicRoot("% !TeX root = 'paper.tex'"), "paper.tex");
});

test("extracts PDFs from watch output", () => {
  assert.equal(parseBuiltPdf("built /tmp/my paper/build/main.pdf in 12.30ms"), "/tmp/my paper/build/main.pdf");
  assert.equal(parseBuiltPdf("cached build/main.pdf in 1.2s (inputs unchanged)"), "build/main.pdf");
});
