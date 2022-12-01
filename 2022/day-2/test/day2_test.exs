defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "run parts" do
    IO.puts("Part 1: #{Day2.part1()}")
    IO.puts("Part 2: #{Day2.part2()}")
  end
end
