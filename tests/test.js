const { readFile } = require("fs/promises");
const { parsePicRec } = require("../pkg");
const { deepEqual } = require("node:assert");
const { describe, it } = require("node:test");

describe("parsePicRec", () => {
  it("File not contain i64", async () => {
    const picFile = await readFile("./tests/files/1513650994.data");
    const res = parsePicRec(new Uint8Array(picFile.buffer));
    const expected = await readFile(
      "./tests/files/1513650994.data.json",
      "utf-8"
    );
    deepEqual(JSON.parse(res), JSON.parse(expected));
  });
  it("File contain i64", async () => {
    const errorFile = await readFile("./tests/files/1620540174.data");
    const res2 = parsePicRec(new Uint8Array(errorFile.buffer));
    const expected2 = await readFile(
      "./tests/files/1620540174.data.json",
      "utf-8"
    );
    deepEqual(JSON.parse(res2), JSON.parse(expected2));
  });
});
