import assert from "node:assert/strict";
import test from "node:test";
import { mkdtemp, writeFile, rm } from "node:fs/promises";
import * as os from "node:os";
import * as path from "node:path";
import { completionContext, parseBibliography, stripComments } from "../tex";
import { indexProject } from "../project";
import { hasSyncTeXMap, parseForwardSync, parseInverseSync, runSyncTeX, syncTeXFailure } from "../synctex";

test("BibTeX parses nested braces, quoted fields, parentheses, strings and concatenation", () => {
  const bib = `@string{venue = "Journal"}
@comment{not an entry @article{fake, title={No}}}
@article{knuth84, title={The {TeX} Book}, author="Donald {Knuth}", year=1984, journal=venue}
@book(other, title="A {nested} title" # " continued", author={A and B}, date={2026-01})
@preamble{"ignored"}`;
  const entries = parseBibliography(bib);
  assert.deepEqual(entries.map((entry) => entry.key), ["knuth84", "other"]);
  assert.equal(entries[0].title, "The TeX Book");
  assert.equal(entries[0].author, "Donald Knuth");
  assert.equal(entries[1].title, "A nested title continued");
  assert.equal(entries[1].year, "2026-01");
  assert.equal(bib.slice(entries[0].offset, entries[0].offset + 7), "knuth84");
});

test("citation contexts support optional arguments, stars, multiple keys and multiline lists", () => {
  for (const command of ["cite", "citep", "citet", "autocite", "parencite", "textcite", "nocite", "Citeauthor"]) {
    const source = `\\${command}*[see][p. 2]{old,\n  Don`;
    const result = completionContext(source, source.length)!;
    assert.equal(result.kind, "citation");
    assert.equal(result.query, "Don");
    assert.equal(result.start, source.length - 3);
  }
});

test("replacement covers an existing key but not its neighbours", () => {
  const source = "\\cite{first, knuth84, last}";
  const result = completionContext(source, source.indexOf("uth"))!;
  assert.equal(source.slice(result.start, result.end), "knuth84");
});

test("completion excludes comments, closed commands and optional arguments", () => {
  for (const source of ["% \\cite{abc", "\\cite{abc} text", "\\cite[abc", "\\begin{verbatim}\n\\cite{abc\n\\end{verbatim}"]) {
    assert.equal(completionContext(source, source.length), undefined);
  }
  assert.equal(completionContext("\\cref{sec:", 10)?.kind, "reference");
  assert.equal(stripComments("a\\%b% hide\nx").length, "a\\%b% hide\nx".length);
});

test("project index follows included files, BibLaTeX resources and cycles without unrelated bibliographies", async () => {
  const files = new Map([
    ["/paper/main.tex", "\\input{sections/a}\n\\bibliography{refs}\n\\label{sec:root}"],
    ["/paper/sections/a.tex", "\\input{../main}\n\\addbibresource[location=local]{more.bib}\n\\label{sec:child}"],
    ["/paper/refs.bib", "@book{one, title={First}, author={Writer}, year={2020}}"],
    ["/paper/sections/more.bib", "@article{two, title={Second}}"],
    ["/other/unrelated.bib", "@book{no, title={No}}"],
  ]);
  const index = await indexProject("/paper/main.tex", async (file) => {
    const value = files.get(file); if (value === undefined) { throw new Error("missing"); } return value;
  });
  assert.deepEqual(index.citations.map((entry) => entry.key).sort(), ["one", "two"]);
  assert.deepEqual(index.labels.map((entry) => entry.key).sort(), ["sec:child", "sec:root"]);
  assert.equal(index.files.length, 4);
});

test("SyncTeX parsing keeps first result, spaces in paths and clamps missing columns", () => {
  assert.deepEqual(parseForwardSync("SyncTeX result begin\nPage:2\nx:42.2\ny:53\nPage:8\nx:10\ny:12"), { page: 2, x: 42.2, y: 53 });
  assert.deepEqual(parseInverseSync("Input:./a paper.tex\nLine:22\nColumn:-1", "/paper"), { file: "/paper/a paper.tex", line: 22, column: 1 });
  for (const source of ["", "Page:1\nx:NaN\ny:0", "Page:0\nx:0\ny:0"]) { assert.throws(() => parseForwardSync(source)); }
  assert.throws(() => parseInverseSync("Input:a.tex\nLine:-1", "/paper"));
});

test("missing SyncTeX maps fail before launching the CLI with a useful recovery message", async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), "tekai-map-test-"));
  try {
    const pdf = path.join(directory, "a paper.pdf");
    assert.equal(await hasSyncTeXMap(pdf), false);
    for (const args of [["edit", "-o", `1:42:53:${pdf}`], ["view", "-i", "5:1:main.tex", "-o", pdf]]) {
      await assert.rejects(runSyncTeX("/does-not-exist/synctex", args, directory), (error: Error) => {
        assert.match(error.message, /No SyncTeX map for a paper.pdf/);
        assert.match(error.message, /Tekai: Build PDF/);
        assert.doesNotMatch(error.message, /usage:|executable not found/);
        return true;
      });
    }
    await writeFile(path.join(directory, "a paper.synctex.gz"), "");
    assert.equal(await hasSyncTeXMap(pdf), false, "empty maps are not usable");
    await writeFile(path.join(directory, "a paper.synctex"), "SyncTeX Version:1");
    assert.equal(await hasSyncTeXMap(pdf), true, "uncompressed maps are supported");
    await rm(path.join(directory, "a paper.synctex"));
    await writeFile(path.join(directory, "a paper.synctex.gz"), "compressed map");
    assert.equal(await hasSyncTeXMap(pdf), true, "compressed maps are supported");
  } finally { await rm(directory, { recursive: true, force: true }); }
});

test("SyncTeX runtime errors keep CLI help in the output log, not the notification", () => {
  const stderr = "SyncTeX ERROR: No SyncTeX available for main.pdf\nusage: synctex edit -o page:x:y:file\n".repeat(20);
  const failure = syncTeXFailure(new Error("Command failed"), stderr);
  assert.match(failure.message, /No SyncTeX available/);
  assert.doesNotMatch(failure.message, /usage:|tekai.synctex.executable/);
  assert(failure.message.length < 400);
  assert.equal(failure.details, stderr.trim());
  assert.match(syncTeXFailure(Object.assign(new Error("spawn ENOENT"), { code: "ENOENT" }), "").message, /tekai.synctex.executable/);
});
