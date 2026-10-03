import assert from "node:assert/strict";
import { test } from "node:test";
import { createPdfReloader } from "../media/pdf-reloader.mjs";

function task(name, error) {
  return { destroyed: false, promise: error ? Promise.reject(error) : Promise.resolve({ name, getPage: async () => ({}) }),
    async destroy() { this.destroyed = true; } };
}

test("a failed reload retains the old PDF and a later successful build replaces it", async () => {
  let displayed;
  const tasks = [];
  const failures = [];
  const reload = createPdfReloader({
    load: async (name) => {
      if (name === "missing") { throw new Error("404"); }
      const candidate = task(name, name === "partial" ? new Error("Invalid PDF structure") : undefined);
      tasks.push(candidate);
      return candidate;
    },
    commit: (pdf) => { displayed = pdf.name; },
    onError: (error, preserved) => failures.push([error.message, preserved]),
  });
  await reload("missing");
  assert.deepEqual(failures.pop(), ["404", false]);
  await reload("first");
  await reload("missing");
  await reload("partial");
  assert.equal(displayed, "first");
  assert.equal(tasks[0].destroyed, false);
  assert.equal(tasks[1].destroyed, true);
  assert.deepEqual(failures, [["404", true], ["Invalid PDF structure", true]]);
  await reload("fixed");
  assert.equal(displayed, "fixed");
  assert.equal(tasks[0].destroyed, true);
  assert.equal(tasks[2].destroyed, false);
});

test("a superseded document is destroyed without replacing the current PDF", async () => {
  let release;
  let started;
  const waiting = new Promise((resolve) => { started = resolve; });
  const slow = { destroyed: false, promise: new Promise((resolve) => { release = () => resolve({ name: "slow", getPage: async () => ({}) }); }),
    async destroy() { this.destroyed = true; } };
  const commits = [];
  const reload = createPdfReloader({
    load: async (name) => { if (name === "slow") { started(); return slow; } return task(name); },
    commit: (pdf) => commits.push(pdf.name),
    onError: (error) => { throw error; },
  });
  const first = reload("slow");
  await waiting;
  const second = reload("latest");
  release();
  await Promise.all([first, second]);
  assert.deepEqual(commits, ["latest"]);
  assert.equal(slow.destroyed, true);
});
