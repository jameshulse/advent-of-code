defmodule Day1 do
  def part1() do
    File.read!("input")
    |> max_calories()
  end

  def part2() do
    File.read!("input")
    |> sum_top_3()
  end

  def max_calories(lines) do
    get_calories_per_elf(lines)
    |> Enum.max()
  end

  def sum_top_3(lines) do
    get_calories_per_elf(lines)
    |> Enum.sort(:desc)
    |> Enum.take(3)
    |> Enum.sum()
  end

  def get_calories_per_elf(lines) do
    lines
    |> String.split("\n\n", trim: true)
    |> Enum.map(fn elf ->
      String.split(elf, "\n", trim: true)
      |> Enum.map(&String.to_integer(&1))
      |> Enum.sum()
    end)
  end
end
