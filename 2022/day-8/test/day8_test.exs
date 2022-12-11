defmodule Day8Test do
  use ExUnit.Case
  doctest Day8

  @test_input """
  30373
  25512
  65332
  33549
  35390
  """

  test "both parts" do
    input = File.read!("input")

    assert Day8.part1(input) == 1832
    assert Day8.part2(input) == -1
  end

  test "part 1" do
    assert Day8.part1(@test_input) == 21
  end

  test "find_visible" do
    assert Day8.find_visible([{2, {0, 1}}, {5, {1, 1}}, {5, {2, 1}}, {1, {3, 1}}, {2, {4, 1}}]) ==
             [{0, 1}, {1, 1}]

    assert Day8.find_visible([{0, {4, 4}}, {9, {3, 4}}, {3, {2, 4}}, {5, {1, 4}}, {3, {0, 4}}]) ==
             [{4, 4}, {3, 4}]
  end

  test "part 2" do
    assert Day8.part2(@test_input) == 8
  end

  test "count_view" do
    assert Day8.count_view([], 5) == 0
    assert Day8.count_view([5, 2], 5) == 1
    assert Day8.count_view([1, 2], 5) == 2
    assert Day8.count_view([4, 9], 5) == 2
  end
end
