defmodule Day12Test do
  use ExUnit.Case

  @test_input """
  Sabqponm
  abcryxxl
  accszExk
  acctuvwj
  abdefghi
  """

  test "both parts" do
    input = File.read!("input")

    assert Day12.part1(input) == 423
    assert Day12.part2(input) == 416
  end

  test "part1" do
    assert Day12.part1(@test_input) == 31
  end

  test "part2" do
    assert Day12.part2(@test_input) == 29
  end

  test "build_map" do
    assert Day12.build_map(@test_input) == %{
             size: {8, 5},
             start: {0, 0},
             goal: {5, 2},
             map: [
               0,
               0,
               1,
               16,
               15,
               14,
               13,
               12,
               0,
               1,
               2,
               17,
               24,
               23,
               23,
               11,
               0,
               2,
               2,
               18,
               25,
               25,
               23,
               10,
               0,
               2,
               2,
               19,
               20,
               21,
               22,
               9,
               0,
               1,
               3,
               4,
               5,
               6,
               7,
               8
             ]
           }
  end
end
