defmodule Day14Test do
  use ExUnit.Case

  @test_input """
  498,4 -> 498,6 -> 496,6
  503,4 -> 502,4 -> 502,9 -> 494,9
  """

  test "both parts" do
    input = File.read!("input")

    assert Day14.part1(input) == 892
    assert Day14.part2(input) == 27155
  end

  test "part1" do
    assert Day14.part1(@test_input) == 24
  end

  test "part2" do
    assert Day14.part2(@test_input) == 93
  end

  test "build_map" do
    {_map, floor} = Day14.build_map(@test_input)

    assert floor == 11
  end

  test "abyss?" do
    {map, _floor} = Day14.build_map(@test_input)

    assert Day14.abyss?(map, {500, 0}) == false
    assert Day14.abyss?(map, {493, 9}) == true
  end
end
