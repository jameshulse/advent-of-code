defmodule Day4 do
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_ranges(&1))
    |> Enum.filter(fn {[left_low, left_high], [right_low, right_high]} ->
      cond do
        left_low >= right_low and left_high <= right_high -> true
        right_low >= left_low and right_high <= left_high -> true
        true -> false
      end
    end)
    |> Enum.count()
  end

  def part2(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_ranges(&1))
    |> Enum.filter(fn {[left_low, left_high], [right_low, right_high]} ->
      cond do
        left_high <= right_high and left_high >= right_low -> true
        right_high <= left_high and right_high >= left_low -> true
        true -> false
      end
    end)
    |> Enum.count()
  end

  def parse_ranges(line) do
    [left, right] = String.split(line, ",")

    [left_low, left_high] = String.split(left, "-")
    [right_low, right_high] = String.split(right, "-")

    {
      [String.to_integer(left_low), String.to_integer(left_high)],
      [String.to_integer(right_low), String.to_integer(right_high)]
    }
  end
end
