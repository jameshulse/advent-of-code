defmodule Day6 do
  def part1(input) do
    find_marker(input, 4)
  end

  def part2(input) do
    find_marker(input, 14)
  end

  def find_marker(input, marker_size) do
    input
    |> String.to_charlist()
    |> Enum.chunk_every(marker_size, 1)
    |> Enum.with_index()
    |> Enum.find(fn {segment, _i} -> is_marker(segment) end)
    |> then(fn {_, i} -> i + marker_size end)
  end

  def is_marker(segment), do: Enum.uniq(segment) == segment
end
