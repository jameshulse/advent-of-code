defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "calculate score" do
    assert Day2.calculate_score("A", "Y") == 8
    assert Day2.calculate_score("B", "X") == 1
    assert Day2.calculate_score("C", "Z") == 6
  end

  test "follow outcomes" do
    assert Day2.follow_guide("A", "Y") == 4
    assert Day2.follow_guide("B", "X") == 1
    assert Day2.follow_guide("C", "Z") == 7
  end

  test "run parts" do
    IO.puts("Part 1: #{Day2.part1()}")
    IO.puts("Part 2: #{Day2.part2()}")
  end

  test "part 1" do
    assert Day2.part1() == 11666
  end
end
