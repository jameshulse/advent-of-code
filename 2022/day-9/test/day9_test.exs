defmodule Day9Test do
  use ExUnit.Case

  @simple_input """
  R 4
  U 4
  L 3
  D 1
  R 4
  D 1
  L 5
  R 2
  """

  @advanced_input """
  R 5
  U 8
  L 8
  D 3
  R 17
  D 10
  L 25
  U 20
  """

  test "both parts" do
    input = File.read!("input")

    assert Day9.part1(input) == 5735
    assert Day9.part2(input) == 2478
  end

  test "part 1" do
    assert Day9.part1(@simple_input) == 13
  end

  test "step" do
    assert Day9.step({0, 0}, {0, 0}, "R") == {{1, 0}, {0, 0}}
    assert Day9.step({1, 0}, {0, 0}, "R") == {{2, 0}, {1, 0}}
    assert Day9.step({4, 0}, {3, 0}, "U") == {{4, -1}, {3, 0}}
    assert Day9.step({4, -1}, {3, 0}, "U") == {{4, -2}, {4, -1}}
  end

  test "part 2" do
    assert Day9.part2(@simple_input) == 1
    assert Day9.part2(@advanced_input) == 36
  end

  test "follow" do
    # Simple examples
    assert Day9.follow({0, 0}, {0, 0}) == {0, 0}
    assert Day9.follow({1, 0}, {0, 0}) == {0, 0}
    assert Day9.follow({2, 0}, {0, 0}) == {1, 0}
    assert Day9.follow({-2, 0}, {0, 0}) == {-1, 0}
    assert Day9.follow({0, -2}, {0, 0}) == {0, -1}
    assert Day9.follow({0, 2}, {0, 0}) == {0, 1}

    # Complex examples
    assert Day9.follow({0, 0}, {1, 2}) == {0, 1}
    assert Day9.follow({2, -1}, {0, 0}) == {1, -1}
    assert Day9.follow({1, -1}, {0, 0}) == {0, 0}
  end

  test "move" do
    assert Day9.move({0, 0}, "R") == {1, 0}
    assert Day9.move({1, 0}, "L") == {0, 0}
    assert Day9.move({0, 1}, "U") == {0, 0}
    assert Day9.move({0, 0}, "D") == {0, 1}
  end
end
