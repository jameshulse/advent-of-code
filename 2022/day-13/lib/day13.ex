defmodule Day13 do
  def part1(input) do
    input
    |> String.split("\n\n", trim: true)
    |> Enum.with_index()
    |> Enum.map(fn {pair, i} ->
      [first, second] = String.split(pair, "\n", trim: true)

      {:ok, first, ""} = JSON.Parser.Array.parse(first)
      {:ok, second, ""} = JSON.Parser.Array.parse(second)

      {{first, second}, i + 1}
    end)
    |> Enum.filter(fn {{left, right}, _num} -> compare(left, right) end)
    # |> IO.inspect(label: "Result", charlists: false)
    |> Enum.map(fn {_arrays, num} -> num end)
    |> Enum.sum()
  end

  def part2(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      {:ok, packet, ""} = JSON.Parser.Array.parse(line)
      packet
    end)
    |> Enum.concat([[[2]], [[6]]])
    |> Enum.sort(&compare/2)
    |> Enum.with_index()
    |> Enum.filter(fn {packet, _i} -> packet == [[2]] or packet == [[6]] end)
    |> Enum.map(fn {_packet, i} -> i + 1 end)
    # |> IO.inspect(charlists: false)
    |> Enum.reduce(1, &*/2)
  end

  def compare(left, right) do
    {l, left} = List.pop_at(left, 0)
    {r, right} = List.pop_at(right, 0)

    # IO.inspect(%{comparing: {l, r}, remaining: {left, right}}, charlists: false)

    cond do
      is_integer(l) and is_integer(r) ->
        cond do
          l == r -> compare(left, right)
          l < r -> true
          l > r -> false
        end

      l == nil and r == nil ->
        nil

      l == nil ->
        true

      r == nil ->
        false

      is_list(l) and is_list(r) ->
        Enum.zip(l, r)
        |> Enum.map(&compare/2)

      is_integer(l) and is_list(r) ->
        compare([l] ++ left, r ++ right)

      is_list(l) and is_integer(r) ->
        compare(l ++ left, [r] ++ right)

      true ->
        raise "Fall through!"
    end
  end
end
