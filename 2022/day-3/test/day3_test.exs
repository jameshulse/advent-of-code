defmodule Day3Test do
  use ExUnit.Case
  doctest Day3

  test "both parts" do
    input = File.read!("input")

    assert Day3.part1(input) == 7597
    assert Day3.part2(input) == 0
  end

  test "find duplicates" do
    assert Day3.find_duplicate("vJrwpWtwJgWrhcsFMMfFFhFp") === "p"
    assert Day3.find_duplicate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL") === "L"
    assert Day3.find_duplicate("PmmdzqPrVvPwwTWBwg") === "P"
    assert Day3.find_duplicate("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn") === "v"
    assert Day3.find_duplicate("ttgJtRGJQctTZtZT") === "t"
    assert Day3.find_duplicate("CrZsJsPPZsGzwwsLwLmpwMDw") === "s"
  end

  test "part 1" do
    input = """
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    """

    assert Day3.part1(input) === 157
  end
end
