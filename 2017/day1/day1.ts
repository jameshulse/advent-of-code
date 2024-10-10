import { expect, test } from "bun:test";

function solveCaptcha(input: string, offset: number) {
  let result = 0;

  for (let i = 0; i < input.length; i++) {
    if (input[i] === input[(i + offset) % input.length]) {
      result += Number(input[i]);
    }
  }

  return result;
}

async function fetchInput() {
  return Bun.file("./input").text();
}

test("part 1 examples", () => {
  expect(solveCaptcha("1122", 1)).toBe(3);
  expect(solveCaptcha("1111", 1)).toBe(4);
  expect(solveCaptcha("1234", 1)).toBe(0);
  expect(solveCaptcha("91212129", 1)).toBe(9);
});

test("part 1 actual", async () => {
  const input = await fetchInput();

  expect(solveCaptcha(input, 1)).toBe(1034);
});

test("part 2 examples", () => {
  expect(solveCaptcha("1212", 2)).toBe(6);
  expect(solveCaptcha("1221", 2)).toBe(0);
  expect(solveCaptcha("123425", 3)).toBe(4);
  expect(solveCaptcha("123123", 3)).toBe(12);
  expect(solveCaptcha("12131415", 4)).toBe(4);
});

test("part 1 actual", async () => {
  const input = await fetchInput();

  expect(solveCaptcha(input, input.length / 2)).toBe(1356);
});