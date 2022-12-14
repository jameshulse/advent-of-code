defmodule Day12 do
  def part1(input) do
    %{size: size, map: map, start: start, goal: goal} = build_map(input)

    path = navigate(map, size, start, goal)

    length(path)
  end

  def part2(input) do
    %{size: size, map: map, goal: goal} = build_map(input)

    starting_points =
      map
      |> Enum.with_index()
      |> Enum.filter(fn {h, _i} -> h == 0 end)
      |> Enum.map(fn {_h, i} -> index_to_coord(size, i) end)

    starting_points
    |> Enum.map(fn s -> navigate(map, size, s, goal) end)
    |> Enum.map(fn path -> length(path) end)
    |> Enum.filter(fn l -> l > 0 end)
    |> Enum.sort(:asc)
    |> List.first()
  end

  def navigate(map, size, start, goal) do
    # Get visitable nodes from current position
    nodes = fn {x, y} = current ->
      current_idx = coord_to_index(size, current)
      current_height = Enum.at(map, current_idx)

      [{x + 1, y}, {x - 1, y}, {x, y - 1}, {x, y + 1}]
      |> Enum.filter(&inside?(size, &1))
      |> Enum.filter(fn neighbour ->
        neighbour_idx = coord_to_index(size, neighbour)
        neighbour_height = Enum.at(map, neighbour_idx)

        neighbour_height <= current_height + 1
      end)
    end

    # Ignore heuristics
    dist = fn _a, _b -> 1 end
    h = fn _a, _b -> 1 end

    Astar.astar({nodes, dist, h}, start, fn v -> v == goal end)
  end

  def build_map(input) do
    lines = String.split(input, "\n", trim: true)

    height = length(lines)
    width = lines |> List.first() |> then(&String.length/1)

    map = Enum.flat_map(lines, &to_charlist/1)

    start = Enum.find_index(map, fn c -> c == ?S end)
    goal = Enum.find_index(map, fn c -> c == ?E end)

    map = Enum.map(map, &alpha_to_height/1)

    %{
      size: {width, height},
      map: map,
      start: index_to_coord({width, height}, start),
      goal: index_to_coord({width, height}, goal)
    }
  end

  def inside?(size, coord)

  def inside?({width, height}, {x, y}) do
    x >= 0 and x < width and y >= 0 and y < height
  end

  def alpha_to_height(alpha) when alpha == ?S, do: 0
  def alpha_to_height(alpha) when alpha == ?E, do: ?z - ?a
  def alpha_to_height(alpha), do: alpha - ?a

  def index_to_coord({width, _height}, index) do
    {rem(index, width), div(index, width)}
  end

  def coord_to_index({width, _height}, {x, y}) do
    y * width + x
  end
end
