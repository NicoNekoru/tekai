const path = require("node:path");
const fs = require("node:fs");
const os = require("node:os");
const { runTests } = require("@vscode/test-electron");
const root = path.resolve(__dirname, "..");
const temporary = fs.realpathSync(fs.mkdtempSync(path.join(os.tmpdir(), "tekai-vscode-test-")));
const workspace = path.join(temporary, "paper");
fs.cpSync(path.join(root, "test-fixtures"), workspace, { recursive: true });
const extensions = path.join(temporary, "extensions");
const workshop = process.env.WORKSHOP_EXTENSION_PATH;
if (workshop) {
  fs.cpSync(workshop, path.join(extensions, path.basename(workshop)), { recursive: true });
  // Configure before either extension activates. Workshop must not launch a
  // competing compiler in response to opening or saving our fixture.
  fs.mkdirSync(path.join(workspace, ".vscode"));
  fs.writeFileSync(path.join(workspace, ".vscode/settings.json"), JSON.stringify({
    "latex-workshop.latex.autoBuild.run": "never",
    "latex-workshop.linting.chktex.enabled": false,
    "latex-workshop.linting.lacheck.enabled": false,
    "latex-workshop.latex.autoClean.run": "never",
    "latex-workshop.latex.autoBuild.cleanAndRetry.enabled": false,
    "latex-workshop.latex.build.enableMagicComments": false,
    "latex-workshop.latex.build.fromFolder": ".",
    "latex-workshop.latex.outDir": "%WORKSPACE_FOLDER%/build",
    "latex-workshop.latex.recipes": [{ name: "Tekai", tools: ["tekai"] }],
    "latex-workshop.latex.tools": [{ name: "tekai", command: process.env.TEKAI_EXECUTABLE ?? "tekai", args: ["build", "%DOC_EXT%", "--synctex"] }],
  }, null, 2));
}
runTests({
  vscodeExecutablePath: process.env.VSCODE_EXECUTABLE,
  extensionDevelopmentPath: root,
  extensionTestsPath: path.join(root, "dist/integration/index.js"),
  launchArgs: [workspace, ...(workshop ? [] : ["--disable-extensions"]), "--disable-workspace-trust", "--skip-welcome", "--skip-release-notes", "--user-data-dir", path.join(temporary, "user"), "--extensions-dir", extensions],
}).catch((error) => { console.error(error); process.exitCode = 1; });
