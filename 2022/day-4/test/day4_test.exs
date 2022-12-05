defmodule Day4Test do
  use ExUnit.Case

  @test_input """
  2-4,6-8
  2-3,4-5
  5-7,7-9
  2-8,3-7
  6-6,4-6
  2-6,4-8
  """

  test "part1" do
    assert Day4.part1(@test_input) == 2
  end

  test "part2" do
    assert Day4.part2(@test_input) == 4
  end

  test "both parts" do
    input = File.read!("input")

    assert Day4.part1(input) == 441
    assert Day4.part2(input) == 861
  end

  test "parse ranges" do
    assert Day4.parse_ranges("2-4,6-8") == {[2, 4], [6, 8]}
  end
end
