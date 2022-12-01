defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "run parts" do
    IO.puts("Part 1: #{Day1.part1()}")
    IO.puts("Part 2: #{Day1.part2()}")
  end

  test "part 1: calculates maximum calories carried by an elf" do
    input = """
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    """

    assert Day1.max_calories(input) == 24_000
  end

  test "part 2: calculates sum of top 3 elves" do
    input = """
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    """

    assert Day1.sum_top_3(input) == 45_000
  end
end
