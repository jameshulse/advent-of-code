defmodule Day14 do
  @rock 1
  @sand 2

  def part1(input) do
    map = build_map(input)

    count_drops(map)
  end

  def part2(input) do
    map = build_map(input)

    -1
  end

  def count_drops(map, counter \\ 0, source \\ {500, 0}) do
    case drop_sand(map, source) do
      :abyss -> counter
      {:stopped, new_map} -> count_drops(new_map, counter + 1, source)
    end
  end

  def drop_sand(map, {x, y} = coord) do
    cond do
      blocked?(map, {x, y + 1}) and !blocked?(map, {x - 1, y + 1}) ->
        drop_sand(map, {x - 1, y + 1})

      blocked?(map, {x, y + 1}) and !blocked?(map, {x + 1, y + 1}) ->
        drop_sand(map, {x + 1, y + 1})

      blocked?(map, {x, y + 1}) ->
        new_map = Map.put(map, {x, y}, @sand)
        {:stopped, new_map}

      abyss?(map, {x, y}) ->
        :abyss

      true ->
        drop_sand(map, {x, y + 1})
    end
  end

  def abyss?(map, {x, y}) do
    any_can_block =
      Map.keys(map)
      |> Enum.any?(fn {block_x, block_y} ->
        block_x == x and block_y > y
      end)

    not any_can_block
  end

  def blocked?(map, coord), do: Map.has_key?(map, coord)

  def build_map(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.flat_map(&parse_path/1)
    |> Enum.reduce(Map.new(), fn {{x_from, y_from}, {x_to, y_to}}, map ->
      for x <- x_from..x_to, y <- y_from..y_to, into: map do
        {{x, y}, @rock}
      end
    end)
  end

  def parse_path(line) do
    coords =
      String.split(line, " -> ", trim: true)
      |> Enum.map(fn coord ->
        [x, y] = String.split(coord, ",", trim: true)

        {String.to_integer(x), String.to_integer(y)}
      end)

    Enum.zip(coords, Enum.drop(coords, 1))
  end
end
