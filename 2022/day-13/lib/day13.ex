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
    |> Enum.reduce(1, &*/2)
  end

  def compare(l, r) do
    case check_order(l, r) do
      :ok -> true
      :inconclusive -> true
      :wrong -> false
    end
  end

  defp check_order(l, r) do
    cond do
      is_integer(l) and is_integer(r) -> compare_ints(l, r)
      is_list(l) and is_list(r) -> compare_lists(l, r)
      is_integer(l) and is_list(r) -> check_order([l], r)
      is_list(l) and is_integer(r) -> check_order(l, [r])
    end
  end

  defp compare_ints(a, b) do
    cond do
      a < b -> :ok
      a > b -> :wrong
      a == b -> :inconclusive
    end
  end

  defp compare_lists(a, b) do
    cond do
      a == [] and b == [] -> :inconclusive
      a == [] -> :ok
      b == [] -> :wrong
      true -> compare_lists_by_element(a, b)
    end
  end

  defp compare_lists_by_element(a, b) do
    {first_a, remaining_a} = List.pop_at(a, 0)
    {first_b, remaining_b} = List.pop_at(b, 0)

    order = check_order(first_a, first_b)

    if order != :inconclusive do
      order
    else
      compare_lists(remaining_a, remaining_b)
    end
  end
end
