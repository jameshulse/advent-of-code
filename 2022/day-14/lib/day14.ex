defmodule Day14 do
  @rock 1
  @sand 2

  def part1(input) do
    {map, _floor} = build_map(input)

    count_drops_simple(map)
  end

  def part2(input) do
    {map, floor} = build_map(input)

    count_drops_advanced(map, floor)
  end

  def count_drops_simple(map, counter \\ 0, source \\ {500, 0}) do
    case drop_sand(map, source) do
      :abyss -> counter
      {:stopped, new_map, _coord} -> count_drops_simple(new_map, counter + 1, source)
    end
  end

  def count_drops_advanced(map, floor, counter \\ 0, source \\ {500, 0}) do
    case drop_sand(map, source, floor) do
      {:stopped, _map, {500, 0}} -> counter + 1
      {:stopped, new_map, _} -> count_drops_advanced(new_map, floor, counter + 1, source)
    end
  end

  def drop_sand(map, {x, y} = coord, floor \\ nil) do
    cond do
      blocked?(map, {x, y + 1}) and !blocked?(map, {x - 1, y + 1}) ->
        drop_sand(map, {x - 1, y + 1}, floor)

      blocked?(map, {x, y + 1}) and !blocked?(map, {x + 1, y + 1}) ->
        drop_sand(map, {x + 1, y + 1}, floor)

      blocked?(map, {x, y + 1}) ->
        {:stopped, Map.put(map, {x, y}, @sand), {x, y}}

      floor != nil and y + 1 == floor ->
        {:stopped, Map.put(map, {x, y}, @sand), {x, y}}

      floor == nil and abyss?(map, {x, y}) ->
        :abyss

      true ->
        drop_sand(map, {x, y + 1}, floor)
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
    |> Enum.reduce({Map.new(), 0}, fn {{x_from, y_from}, {x_to, y_to}}, {map, floor} ->
      map =
        for x <- x_from..x_to, y <- y_from..y_to, into: map do
          {{x, y}, @rock}
        end

      {map, max(max(floor, y_to), y_from)}
    end)
    |> then(fn {map, max_height} -> {map, max_height + 2} end)
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
