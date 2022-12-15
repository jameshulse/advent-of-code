defmodule Day13Test do
  use ExUnit.Case
  doctest Day13

  @test_input """
  [1,1,3,1,1]
  [1,1,5,1,1]

  [[1],[2,3,4]]
  [[1],4]

  [9]
  [[8,7,6]]

  [[4,4],4,4]
  [[4,4],4,4,4]

  [7,7,7,7]
  [7,7,7]

  []
  [3]

  [[[]]]
  [[]]

  [1,[2,[3,[4,[5,6,7]]]],8,9]
  [1,[2,[3,[4,[5,6,0]]]],8,9]
  """

  test "both parts" do
    input = File.read!("input")

    assert Day13.part1(input) == 5825
    assert Day13.part2(input) == 24477
  end

  test "part1" do
    assert Day13.part1(@test_input) == 13
  end

  test "part2" do
    assert Day13.part2(@test_input) == 140
  end

  test "compare: final answers" do
    assert Day13.compare([1, 1, 3, 1, 1], [1, 1, 5, 1, 1]) == true
    assert Day13.compare([[1], [2, 3, 4]], [[1], 4]) == true
    assert Day13.compare([9], [[8, 7, 6]]) == false
    assert Day13.compare([[4, 4], 4, 4], [[4, 4], 4, 4, 4]) == true
    assert Day13.compare([7, 7, 7, 7], [7, 7, 7]) == false
    assert Day13.compare([], [3]) == true
    assert Day13.compare([[[]]], [[]]) == false

    assert Day13.compare([1, [2, [3, [4, [5, 6, 7]]]], 8, 9], [
             1,
             [2, [3, [4, [5, 6, 0]]]],
             8,
             9
           ]) == false

    assert Day13.compare([[1], [2, 3, 4]], [[1], 4]) == true
    assert Day13.compare([1, [2, [3, [4, [5, 6, 0]]]], 8, 9], [[1], 4]) == true
    assert Day13.compare([[1], 4], [[2]]) == true
    assert Day13.compare([[[]]], [1, 1, 3, 1, 1]) == true

    assert Day13.compare([[1, 3]], [1, 2, 3]) == false
    assert Day13.compare([[8, [[7]]]], [[[[8]]]]) == false

    assert Day13.compare([[8, [[7]]]], [[[[8], [3]]]]) == true
  end
end
