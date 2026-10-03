import assert from "node:assert/strict";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import * as os from "node:os";
import * as path from "node:path";
import test from "node:test";
import { findProjectConfig } from "../config";

test("config search uses the nearest ancestor and sees creation/deletion without stale caching", async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), "tekai-config-test-"));
  try {
    const rootConfig = path.join(directory, "tekai.toml");
    const oldPaper = path.join(directory, "writeups", "old");
    const currentPaper = path.join(directory, "writeups", "current", "sections");
    await mkdir(oldPaper, { recursive: true });
    await mkdir(currentPaper, { recursive: true });
    await writeFile(rootConfig, "[lint]\n");
    assert.equal(await findProjectConfig(currentPaper), rootConfig);
    const sharedConfig = path.join(directory, "writeups", "tekai.toml");
    await writeFile(sharedConfig, "[lint]\nindent_style = 'tabs'\n");
    const oldConfig = path.join(oldPaper, "tekai.toml");
    await writeFile(oldConfig, "[lint]\nindent_style = 'spaces'\n");
    assert.equal(await findProjectConfig(currentPaper), sharedConfig);
    assert.equal(await findProjectConfig(oldPaper), oldConfig);
    await rm(oldConfig);
    assert.equal(await findProjectConfig(oldPaper), sharedConfig);
    await mkdir(oldConfig);
    assert.equal(await findProjectConfig(oldPaper), sharedConfig, "a directory called tekai.toml is not a config file");
  } finally { await rm(directory, { recursive: true, force: true }); }
});
