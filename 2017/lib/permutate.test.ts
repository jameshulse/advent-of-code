import { expect, test } from "bun:test";
import { combinations } from "./permutate";

test("combinations", () => {
  expect(combinations([1, 2, 3])).toEqual([
    [1, 2],
    [1, 3],
    [2, 3],
  ]);
});