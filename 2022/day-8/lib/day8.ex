defmodule Day8 do
  def part1(input) do
    forest =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn line -> String.graphemes(line) |> Enum.map(&String.to_integer/1) end)
      |> Enum.with_index()
      |> Enum.map(fn {line, y} ->
        line |> Enum.with_index() |> Enum.map(fn {h, x} -> {h, {x, y}} end)
      end)

    forest_transverse = forest |> Enum.zip_with(& &1)

    all_lines =
      forest ++
        forest_transverse ++
        Enum.map(forest, &Enum.reverse/1) ++
        Enum.map(forest_transverse, &Enum.reverse/1)

    all_visible = all_lines |> Enum.flat_map(&find_visible/1) |> then(&MapSet.new/1)

    MapSet.size(all_visible)
  end

  def find_visible(line) do
    line
    |> Enum.with_index()
    |> Enum.reduce({0, []}, fn {{height, pos}, i}, {tallest, visible} = acc ->
      cond do
        i == 0 -> {height, visible ++ [pos]}
        height > tallest -> {height, visible ++ [pos]}
        true -> acc
      end
    end)
    |> then(fn {tallest, visible} -> visible end)
  end
end
