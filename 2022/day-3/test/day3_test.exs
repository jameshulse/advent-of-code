defmodule Day3Test do
  use ExUnit.Case
  doctest Day3

  @test_input """
  vJrwpWtwJgWrhcsFMMfFFhFp
  jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
  PmmdzqPrVvPwwTWBwg
  wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
  ttgJtRGJQctTZtZT
  CrZsJsPPZsGzwwsLwLmpwMDw
  """

  test "both parts" do
    input = File.read!("input")

    assert Day3.part1(input) == 7597
    assert Day3.part2(input) == 0
  end

  test "find duplicate item in rucksack" do
    unique_items =
      @test_input
      |> String.split("\n", trim: true)
      |> Enum.map(&Day3.find_duplicate(&1))

    assert unique_items == ["p", "L", "P", "v", "t", "s"]
  end

  test "part 1" do
    assert Day3.part1(@test_input) === 157
  end

  test "find common items in groups of rucksacks" do
    all_rucksacks = @test_input |> String.split("\n", trim: true)

    assert Day3.find_common(all_rucksacks |> Enum.take(3)) == "r"
    assert Day3.find_common(all_rucksacks |> Enum.drop(3) |> Enum.take(3)) == "Z"
  end

  test "part 2" do
    assert Day3.part2(@test_input) === 70
  end
end
