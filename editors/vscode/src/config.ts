import { stat } from "node:fs/promises";
import * as path from "node:path";

// Match the CLI's root-document config search. Do not cache across commands:
// adding, changing, or deleting a nearer config must affect the next invocation.
export async function findProjectConfig(directory: string): Promise<string | undefined> {
  for (let current = path.resolve(directory); ; current = path.dirname(current)) {
    const candidate = path.join(current, "tekai.toml");
    try {
      if ((await stat(candidate)).isFile()) { return candidate; }
    } catch (error) {
      const code = (error as NodeJS.ErrnoException).code;
      if (code !== "ENOENT" && code !== "ENOTDIR") { throw error; }
    }
    if (path.dirname(current) === current) { return; }
  }
}
