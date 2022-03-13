defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "Part 1: program execution" do
    program = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]

    iter1 = Day2.execute_instruction(program, 0)
    iter2 = Day2.execute_instruction(iter1, 4)

    assert iter1 == [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    assert iter2 == [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
  end
end
