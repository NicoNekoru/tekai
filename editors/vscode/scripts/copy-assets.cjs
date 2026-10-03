const fs = require("node:fs");
const path = require("node:path");
const output = path.resolve(__dirname, "../dist/viewer");
fs.mkdirSync(output, { recursive: true });
fs.cpSync(path.resolve(__dirname, "../media"), output, { recursive: true });
const pdfjs = path.dirname(require.resolve("pdfjs-dist/package.json"));
for (const name of ["build/pdf.mjs", "build/pdf.worker.mjs", "web/pdf_viewer.mjs", "web/pdf_viewer.css", "web/images", "cmaps", "standard_fonts", "wasm", "LICENSE"]) {
  const destination = path.join(output, "pdfjs", name);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  const compatible = /^(build|web)\//.test(name) ? path.join(pdfjs, "legacy", name) : path.join(pdfjs, name);
  fs.cpSync(compatible, destination, { recursive: true });
}
