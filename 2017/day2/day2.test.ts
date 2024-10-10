import { expect, test } from "bun:test";
import { combinations } from "../lib/permutate";

function checksumHighLow(input: string) {
  let result = 0;
  const lines = input.split("\n");

  for (let line of lines) {
    const values = line.split("\t").map(Number);

    const low = Math.min(...values);
    const high = Math.max(...values);

    result += high - low;
  }

  return result;
}

const input = await Bun.file("./input").text();

test("part 1 example", () => {
  expect(checksumHighLow("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8")).toBe(18);
});

test("part 1 actual", async () => {
  expect(checksumHighLow(input)).toBe(34581);
});

function checksumDivisble(input: string) {
  const lines = input.split("\n");
  let result = 0;

  for (let line of lines) {
    const values = combinations(line.split("\t").map(Number)).map((combo) => combo.toSorted((a, b) => a - b));
    const combo = values.find(([a, b]) => b % a === 0);

    if (! combo) {
      console.error({line, values})

      throw new Error("No combo found");
    }

    result += combo[1] / combo[0];
  }

  return result;
}

test("part 2 examples", () => {
  expect(checksumDivisble("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5")).toBe(9);
});

test("part 2 actual", async () => {
  expect(checksumDivisble(input)).toBe(214);
});