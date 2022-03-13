defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "Part 1: instruction execution" do
    program = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]

    {:step, iter1} = Day2.execute_instruction(program, 0)
    {:step, iter2} = Day2.execute_instruction(iter1, 4)

    assert iter1 == [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    assert iter2 == [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]

    {:exit, iter3} = Day2.execute_instruction(iter2, 8)

    assert iter3 == [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
  end

  test "Part 1: program execution" do
    program = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]

    assert Day2.execute_program(program) == [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
  end
end
