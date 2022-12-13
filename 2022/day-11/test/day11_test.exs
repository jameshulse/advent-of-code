defmodule Day11Test do
  use ExUnit.Case

  @test_input """
  Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
  """

  test "part 1" do
    assert Day11.part1(@test_input) == -1
  end
  
  test "parse_input" do
    assert Day1.parse_input(@test_input) == [
      %{items: [79, 98], op: {:multiply, 19}, test: 23, true: 2, false, 3}
      %{items: [54, 65, 75, 74], op: {:add, 6}, test: 19, true: 2, false, 0}
      %{items: [79, 60, 97], op: {:square}, test: 13, true: 1, false, 3}
      %{items: [74], op: {:add, 3}, test: 17, true: 0, false, 1}
    ]
  end
end
