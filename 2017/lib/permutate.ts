export function combinations<T>(input: T[]) {
  const result: T[][] = [];
  
  for (let i = 0; i < input.length; i++) {
    const remaining = input.slice(i + 1);
    const combination = input[i];

    for (let j = 0; j < remaining.length; j++) {
      result.push([combination, remaining[j]]);
    }
  }

  return result;
}
