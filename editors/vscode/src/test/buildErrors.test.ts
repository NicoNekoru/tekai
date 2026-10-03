import assert from "node:assert/strict";
import * as path from "node:path";
import { test } from "node:test";
import { parseTexErrors } from "../buildErrors";

test("compiler diagnostics resolve included files from the root document", () => {
  const directory = path.resolve("paper space/current");
  const errors = parseTexErrors("Error: TeX engine failed with status exit status: 1\nsections/6_prediction.tex:6: Missing \\endcsname inserted.\n<to be read again>\n\\protect\nsections/6_prediction.tex:6: ==> Fatal error occurred\nTeX log: output/pdf/main.log\n", directory);
  assert.deepEqual(errors, [{ file: path.join(directory, "sections/6_prediction.tex"), line: 6, message: "Missing \\endcsname inserted." }]);
  assert.deepEqual(parseTexErrors("invalid config\n! Emergency stop.\nfile.tex:0: bad\n", directory), []);
  assert.equal(parseTexErrors(`${path.join(directory, "main.tex")}:15: Undefined control sequence.\r\n`, directory)[0].line, 15);
});
