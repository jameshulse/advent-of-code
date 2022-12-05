defmodule Day5Test do
  use ExUnit.Case

  @test_input """
      [D]    
  [N] [C]    
  [Z] [M] [P]
   1   2   3 

  move 1 from 2 to 1
  move 3 from 1 to 3
  move 2 from 2 to 1
  move 1 from 1 to 2
  """

  test "parse input" do
    {stacks, _} = Day5.parse_input(@test_input)

    assert stacks == [
             ["Z", "N"],
             ["M", "C", "D"],
             ["P"]
           ]
  end

  test "perform_move_single" do
    {stacks, _} = Day5.parse_input(@test_input)

    assert Day5.perform_move(stacks, "move 1 from 2 to 1") == [
             ["Z", "N", "D"],
             ["M", "C"],
             ["P"]
           ]
  end

  test "perform_move_multiple" do
    {stacks, _} = Day5.parse_input(@test_input)

    assert Day5.perform_move(stacks, "move 2 from 1 to 3", :multiple) == [
             [],
             ["M", "C", "D"],
             ["P", "Z", "N"]
           ]
  end

  test "get_top" do
    {stacks, _} = Day5.parse_input(@test_input)

    assert Day5.get_top(stacks) == "NDP"
  end

  test "part 1 with test input" do
    assert Day5.part1(@test_input) == "CMZ"
  end

  test "part 2 with test input" do
    assert Day5.part2(@test_input) == "MCD"
  end

  test "both parts" do
    input = File.read!("input")

    assert Day5.part1(input) == "VRWBSFZWM"
    assert Day5.part2(input) == "RBTWJWMCF"
  end
end
