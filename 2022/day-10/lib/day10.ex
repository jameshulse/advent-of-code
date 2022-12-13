defmodule Day10 do
  def part1(input) do
    parse_instructions(input)
    |> run_program()
    |> Enum.with_index()
    |> Enum.drop(19)
    |> Enum.take_every(40)
    |> Enum.map(fn {value, i} -> signal_strength(i + 1, value) end)
    |> Enum.sum()
    |> then(fn value -> value end)
  end

  def parse_instructions(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      if line == "noop" do
        {:noop, nil}
      else
        [instr, arg] = String.split(line, " ")

        {:addx, String.to_integer(arg)}
      end
    end)
  end

  def run_program(instructions) do
    instructions
    |> Enum.flat_map_reduce(1, fn {instr, arg}, register ->
      case instr do
        :noop -> {[register], register}
        :addx -> {[register, register], register + arg}
      end
    end)
    |> then(fn {values, register} -> values ++ [register] end)
  end

  def signal_strength(cycle, register), do: cycle * register
end
