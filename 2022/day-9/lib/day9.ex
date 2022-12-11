defmodule Day9 do
  def part1(input) do
    parse_moves(input)
    |> Enum.reduce({{0, 0}, {0, 0}, MapSet.new([{0, 0}])}, fn dir, {head, tail, visited} ->
      {new_head, new_tail} = step_simple({head, tail}, dir)

      {new_head, new_tail, MapSet.put(visited, new_tail)}
    end)
    |> then(fn {_, _, visited} -> MapSet.size(visited) end)
  end

  def parse_moves(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.flat_map(fn line ->
      [dir, count] = String.split(line, " ")

      count = String.to_integer(count)

      Enum.to_list(1..count) |> Enum.map(fn _ -> dir end)
    end)
  end

  def move({x, y}, dir) do
    case dir do
      "R" -> {x + 1, y}
      "L" -> {x - 1, y}
      "D" -> {x, y + 1}
      "U" -> {x, y - 1}
    end
  end

  def step_simple({head, {tx, ty} = tail}, dir) do
    {hx, hy} = move(head, dir)

    dx = abs(hx - tx)
    dy = abs(hy - ty)

    if dx > 1 or dy > 1 do
      {{hx, hy}, head}
    else
      {{hx, hy}, tail}
    end
  end

  def part2(input) do
    -1
  end
end
