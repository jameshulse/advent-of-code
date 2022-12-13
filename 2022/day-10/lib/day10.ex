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

  def part2(input) do
    output = render_output(input)

    IO.puts(output)
  end

  @screen_width 40

  def render_output(input) do
    values = parse_instructions(input) |> run_program() |> Enum.with_index()

    for {value, i} <- values, into: "" do
      cycle = i + 1

      display =
        cond do
          abs(value - rem(i, 40)) <= 1 -> "#"
          true -> "."
        end

      if rem(cycle, @screen_width) == 0 do
        "#{display}\n"
      else
        display
      end
    end
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
    |> then(fn {values, register} -> values end)
  end

  def signal_strength(cycle, register), do: cycle * register
end
