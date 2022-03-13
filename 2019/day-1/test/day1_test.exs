defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "part1: calculates correct fuel" do
    assert Day1.calculate_fuel(12) == 2
    assert Day1.calculate_fuel(14) == 2
    assert Day1.calculate_fuel(1969) == 654
    assert Day1.calculate_fuel(100_756) == 33583
  end

  test "part2: calculates recursive fuel correctly" do
    assert Day1.recurse_fuel(14) == 2
    assert Day1.recurse_fuel(1969) == 966
    assert Day1.recurse_fuel(100_756) == 50346
  end
end
