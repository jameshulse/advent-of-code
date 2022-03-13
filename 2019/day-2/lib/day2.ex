defmodule Day2 do
  def start(_type, _args) do
    IO.puts(part1())
    # IO.puts(part2)
  end

  def part1 do
    load_state()
    |> List.replace_at(1, 12)
    |> List.replace_at(2, 2)
    |> execute_program()
    |> List.first()
  end

  def part2 do
    load_state()
    |> List.replace_at(1, 12)
    |> List.replace_at(2, 2)
    # TODO
    |> List.first()
  end

  def execute_program(program, location \\ 0) do
    case execute_instruction(program, location) do
      {:step, output} -> execute_program(output, location + 4)
      {:exit, output} -> output
    end
  end

  def execute_instruction(program, location) do
    case Enum.slice(program, location..(location + 3)) do
      [99 | _] ->
        {:exit, program}

      [1, left, right, output] ->
        program =
          List.replace_at(program, output, Enum.at(program, left) + Enum.at(program, right))

        {:step, program}

      [2, left, right, output] ->
        program =
          List.replace_at(program, output, Enum.at(program, left) * Enum.at(program, right))

        {:step, program}
    end
  end

  def load_state do
    File.read!("input")
    |> String.split(",", trim: true)
    |> Enum.map(&String.to_integer(&1))
  end
end
