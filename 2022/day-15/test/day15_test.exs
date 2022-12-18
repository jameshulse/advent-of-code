defmodule Day15Test do
  use ExUnit.Case

  @test_input """
  Sensor at x=2, y=18: closest beacon is at x=-2, y=15
  Sensor at x=9, y=16: closest beacon is at x=10, y=16
  Sensor at x=13, y=2: closest beacon is at x=15, y=3
  Sensor at x=12, y=14: closest beacon is at x=10, y=16
  Sensor at x=10, y=20: closest beacon is at x=10, y=16
  Sensor at x=14, y=17: closest beacon is at x=10, y=16
  Sensor at x=8, y=7: closest beacon is at x=2, y=10
  Sensor at x=2, y=0: closest beacon is at x=2, y=10
  Sensor at x=0, y=11: closest beacon is at x=2, y=10
  Sensor at x=20, y=14: closest beacon is at x=25, y=17
  Sensor at x=17, y=20: closest beacon is at x=21, y=22
  Sensor at x=16, y=7: closest beacon is at x=15, y=3
  Sensor at x=14, y=3: closest beacon is at x=15, y=3
  Sensor at x=20, y=1: closest beacon is at x=15, y=3
  """

  test "both parts" do
    input = File.read!("input")

    assert Day15.part1(input, 2_000_000) == 4_951_427
    assert Day15.part2(input, 0..4_000_000) == 13_029_714_573_243
  end

  test "part1" do
    assert Day15.part1(@test_input, 10) == 26
  end

  test "part2" do
    assert Day15.part2(@test_input, 0..20) == 56_000_011
  end

  test "intersection" do
    assert Day15.intersection({8, 7}, 9, -2) == 8..8
    assert Day15.intersection({8, 7}, 9, 1) == 5..11
    assert Day15.intersection({8, 7}, 9, 14) == 6..10
    assert Day15.intersection({8, 7}, 9, -3) == :none
    assert Day15.intersection({8, 7}, 9, 17) == :none
  end

  test "reduce_ranges" do
    assert Day15.reduce_ranges([12..12, 2..14, 2..2, -2..2, 16..24, 14..18]) == [-2..24]
    assert Day15.reduce_ranges([0..6, 7..20]) == [0..20]

    assert Day15.reduce_ranges([1_178_096..2_044_526, 0..1_248_064]) == [0..2_044_526]

    assert Day15.reduce_ranges(
             [3_666_940..4_000_000, 2_658_353..3_257_427, 3_257_429..3_859_523, 0..2_780_821]
             |> Enum.sort()
           ) == [0..3_257_427, 3_257_429..4_000_000]
  end

  test "overlap?" do
    assert Day15.overlap?(0..5, 6..10) == true
    assert Day15.overlap?(0..5, 2..8) == true
    assert Day15.overlap?(3..4, 5..5) == true
    assert Day15.overlap?(6..9, 3..4) == false
    assert Day15.overlap?(3..4, 6..9) == false
  end
end
