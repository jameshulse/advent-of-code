defmodule Day5 do
  def parse_input(input) do
    [diagram, moves] = String.split(input, "\n\n", trim: true)

    stacks =
      diagram
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        line
        |> String.graphemes()
        |> Enum.chunk_every(4)
        |> Enum.map(fn g -> Enum.join(g) |> String.replace(~r/[\[\]\s]/, "") end)
      end)
      |> Enum.drop(-1)
      |> Enum.zip_with(& &1)
      |> Enum.map(fn stack ->
        stack
        |> Enum.reverse()
        |> Enum.filter(fn x -> x != "" end)
      end)

    moves = String.split(moves, "\n", trim: true)

    {stacks, moves}
  end

  def perform_move(stacks, move, move_type \\ :single) do
    {count, from, to} = parse_move(move)

    new_from = Enum.at(stacks, from - 1) |> Enum.reverse() |> Enum.drop(count) |> Enum.reverse()

    moved =
      if move_type == :single do
        Enum.at(stacks, from - 1) |> Enum.reverse() |> Enum.take(count)
      else
        Enum.at(stacks, from - 1) |> Enum.reverse() |> Enum.take(count) |> Enum.reverse()
      end

    new_to = Enum.at(stacks, to - 1) ++ moved

    stacks
    |> List.replace_at(from - 1, new_from)
    |> List.replace_at(to - 1, new_to)
  end

  def get_top(stacks) do
    stacks |> Enum.map(fn s -> List.last(s) end) |> Enum.join()
  end

  def part1(input) do
    {stacks, moves} = parse_input(input)

    moves
    |> Enum.reduce(stacks, fn m, s -> perform_move(s, m) end)
    |> get_top()
  end

  def parse_move(move) do
    [_, count, _, from, _, to] = move |> String.split(" ", trim: true)

    count = String.to_integer(count)
    from = String.to_integer(from)
    to = String.to_integer(to)

    {count, from, to}
  end

  def part2(input) do
    {stacks, moves} = parse_input(input)

    moves
    |> Enum.reduce(stacks, fn m, s -> perform_move(s, m, :multiple) end)
    |> get_top()
  end
end
