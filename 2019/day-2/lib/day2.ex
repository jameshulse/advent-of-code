defmodule Day2 do
  def start(_type, _args) do
    IO.puts(part1())
    # IO.puts(part2)
  end

  def part1 do
    program = load_state()

    program = execute_instruction(program, 0)

    List.first(program)
  end

  def execute_instruction(program, location) do
    case Enum.at(program, location) do
      99 ->
        program

      1 ->
        [left, right, output] = Enum.slice(program, (location + 1)..(location + 3))

        List.replace_at(program, output, Enum.at(program, left) + Enum.at(program, right))

      2 ->
        [left, right, output] = Enum.slice(program, (location + 1)..(location + 3))

        List.replace_at(program, output, Enum.at(program, left) * Enum.at(program, right))
    end
  end

  def load_state do
    File.read!("input")
    |> String.split(",", trim: true)
    |> Enum.map(&String.to_integer(&1))
  end
end
