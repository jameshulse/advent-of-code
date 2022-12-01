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
    lines
    |> String.split("\n\n", trim: true)
    |> Enum.map(fn elf ->
      String.split(elf, "\n", trim: true)
      |> Enum.map(fn l -> String.to_integer(l) end)
      |> Enum.sum()
    end)
    |> Enum.max()
  end

  def sum_top_3(lines) do
    lines
    |> String.split("\n\n", trim: true)
    |> Enum.map(fn elf ->
      String.split(elf, "\n", trim: true)
      |> Enum.map(fn l -> String.to_integer(l) end)
      |> Enum.sum()
    end)
    |> Enum.sort(:desc)
    |> Enum.take(3)
    |> Enum.sum()
  end
end
