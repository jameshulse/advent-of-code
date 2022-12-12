defmodule Day9 do
  def part1(input) do
    parse_moves(input)
    |> Enum.reduce({{0, 0}, {0, 0}, MapSet.new([{0, 0}])}, fn dir, {head, tail, visited} ->
      {new_head, new_tail} = step(head, tail, dir)

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

  def step(head, {tx, ty} = tail, dir) do
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
    knots = 1..10 |> Enum.map(fn _ -> {0, 0} end) |> Enum.to_list()

    parse_moves(input)
    # |> Enum.take(6)
    |> IO.inspect()
    |> Enum.reduce({knots, MapSet.new()}, fn dir, {knots, visited} ->
      {knots, last} =
        Enum.map_reduce(knots, nil, fn k, prev ->
          if prev == nil do
            head = move(k, dir)

            {head, head}
          else
            new_next = follow(prev, k)

            {new_next, new_next}
          end
        end)
        |> IO.inspect(label: "Inner")

      {knots, MapSet.put(visited, last)}
    end)
    # |> IO.inspect(label: "Outer")
    |> then(fn {_knots, visited} -> MapSet.size(visited) end)
  end

  def follow({hx, hy} = head, {tx, ty} = tail) do
    dx = hx - tx
    dy = hy - ty
    distance = max(abs(dx), abs(dy))

    # IO.inspect({head, tail, dx, dy}, label: "follow")

    case {distance, dx, dy} do
      {0, _, _} ->
        tail

      {1, _, _} ->
        tail

      # Diagonals
      {_, _, _} when dx < 0 and dy < 0 ->
        {tx - 1, ty - 1}

      {_, _, _} when dx > 0 and dy < 0 ->
        {tx + 1, ty - 1}

      {_, _, _} when dx > 0 and dy > 0 ->
        {tx + 1, ty + 1}

      {_, _, _} when dx < 0 and dy > 0 ->
        {tx - 1, ty + 1}

      # Right, left, down, up
      {_, 2, 0} ->
        {tx + 1, ty}

      {_, -2, 0} ->
        {tx - 1, ty}

      {_, 0, 2} ->
        {tx, ty + 1}

      {_, 0, -2} ->
        {tx, ty - 1}
        # Otherwise, don't move
        # {_, _, _} -> tail
    end
  end
end
