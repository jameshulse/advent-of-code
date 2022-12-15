defmodule Day11 do
  def part1(input) do
    monkeys = parse_input(input)

    run_simulation(monkeys, 20, :divide_by_3)
  end

  def part2(input) do
    monkeys = parse_input(input)
    modulo = monkeys |> Enum.reduce(1, fn m, a -> m[:test] * a end)

    run_simulation(monkeys, 10_000, {:modulo, modulo})
  end

  def run_simulation(monkeys, rounds, management_strategy) do
    Enum.reduce(0..(rounds * length(monkeys) - 1), monkeys, fn i, monkeys ->
      monkey_index = rem(i, length(monkeys))
      monkey = Enum.at(monkeys, monkey_index)

      item_count = length(monkey[:items])

      # Give items to other monkeys
      monkey[:items]
      |> Enum.map(fn worry ->
        # Calculate new worry value
        worry =
          case monkey[:operation] do
            {:square} -> worry * worry
            {:add, addend} -> worry + addend
            {:multiply, multiplier} -> worry * multiplier
          end

        worry =
          case management_strategy do
            :divide_by_3 -> div(worry, 3)
            {:modulo, modulo} -> rem(worry, modulo)
          end

        send_to =
          if rem(worry, monkey[:test]) === 0, do: monkey[:if_true], else: monkey[:if_false]

        {send_to, worry}
      end)
      |> Enum.reduce(monkeys, fn {send_to, worry}, monkeys ->
        List.update_at(monkeys, send_to, fn m ->
          Map.merge(m, %{
            items: m[:items] ++ [worry]
          })
        end)
      end)
      |> List.update_at(monkey_index, fn m ->
        Map.merge(m, %{
          items: [],
          activity: m[:activity] + item_count
        })
      end)
    end)
    |> Enum.map(fn m -> m[:activity] end)
    |> Enum.sort(:desc)
    |> Enum.take(2)
    |> Enum.reduce(1, &(&1 * &2))
  end

  def parse_input(input) do
    input
    |> String.split("\n\n")
    |> Enum.map(fn description ->
      [_, items, operation, test, if_true, if_false] = String.split(description, "\n", trim: true)

      [items] = String.split(items, "  Starting items: ", trim: true)
      [operation] = String.split(operation, "  Operation: new = ", trim: true)
      [test] = String.split(test, "  Test: divisible by ", trim: true)
      [if_true] = String.split(if_true, "    If true: throw to monkey ", trim: true)
      [if_false] = String.split(if_false, "    If false: throw to monkey ", trim: true)

      operation =
        case operation do
          "old * old" -> {:square}
          "old * " <> multiplier -> {:multiply, String.to_integer(multiplier)}
          "old + " <> addend -> {:add, String.to_integer(addend)}
        end

      %{
        items: items |> String.split(", ", trim: true) |> Enum.map(&String.to_integer/1),
        operation: operation,
        test: String.to_integer(test),
        if_true: String.to_integer(if_true),
        if_false: String.to_integer(if_false),
        activity: 0
      }
    end)
  end
end
