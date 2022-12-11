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

  def part2(input) do
    forest =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn line -> String.graphemes(line) |> Enum.map(&String.to_integer/1) end)

    forest_transverse = forest |> Enum.zip_with(& &1)

    scenic_scores =
      for {row, y} <- Enum.with_index(forest), {col, x} <- Enum.with_index(forest_transverse) do
        {left, right} = Enum.split(row, x)
        {up, down} = Enum.split(col, y)

        height = Enum.at(row, x)

        left_score = count_view(Enum.reverse(left), height)
        up_score = count_view(Enum.reverse(up), height)
        right_score = count_view(Enum.drop(right, 1), height)
        down_score = count_view(Enum.drop(down, 1), height)

        left_score * up_score * right_score * down_score
      end

    Enum.max(scenic_scores)
  end

  def count_view(view, height, count \\ 0)
  def count_view([], _height, count), do: count
  def count_view(_view, 0, _count), do: 0

  def count_view([next | rest], height, count) do
    cond do
      next < height -> count_view(rest, height, count + 1)
      true -> count + 1
    end
  end
end
