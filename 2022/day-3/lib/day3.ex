defmodule Day3 do
  @alphabet "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&find_duplicate(&1))
    |> Enum.map(&priority(&1))
    |> Enum.sum()
  end

  def find_duplicate(rucksack) do
    compartment_size = trunc(String.length(rucksack) / 2)

    left_items = String.slice(rucksack, 0, compartment_size) |> String.graphemes() |> Enum.uniq()

    right_items =
      String.slice(rucksack, compartment_size, compartment_size)
      |> String.graphemes()
      |> Enum.uniq()

    unique = Enum.uniq(left_items) ++ Enum.uniq(right_items)

    unique
    |> Enum.group_by(fn x -> x end)
    |> Enum.flat_map(fn {k, v} -> if length(v) > 1, do: [k], else: [] end)
    |> List.first()
  end

  def part2(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.chunk(3)
    |> Enum.map(&find_common(&1))
    |> Enum.map(&priority(&1))
    |> Enum.sum()
  end

  def find_common(rucksacks) do
    rucksacks
    |> Enum.map(&String.graphemes(&1))
    |> Enum.reduce(&MapSet.intersection(MapSet.new(&1), MapSet.new(&2)))
    |> MapSet.to_list()
    |> List.first()
  end

  def priority(code) do
    {i, _} = :binary.match(@alphabet, code)
    i + 1
  end
end
