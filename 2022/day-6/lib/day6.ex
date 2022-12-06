defmodule Day6 do
  def part1(input) do
    find_marker(input, 4)
  end

  def part2(input) do
    find_marker(input, 14)
  end

  def find_marker(input, marker_size) do
    {_, i} =
      input
      |> String.graphemes()
      |> Enum.chunk_every(marker_size, 1)
      |> Enum.with_index()
      |> Enum.find(fn {segment, i} -> is_marker(segment) end)

    i + marker_size
  end

  def is_marker(segment) do
    uniq_count = segment |> Enum.uniq() |> Enum.count()

    uniq_count == length(segment)
  end
end
