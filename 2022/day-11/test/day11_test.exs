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

  test "both parts" do
    input = File.read!("input")

    assert Day11.part1(input) == -1
  end

  test "part 1" do
    assert Day11.part1(@test_input) == 10605
  end

  test "parse_input" do
    assert Day11.parse_input(@test_input) == [
             %{
               items: [79, 98],
               operation: {:multiply, 19},
               test: 23,
               if_true: 2,
               if_false: 3,
               activity: 0
             },
             %{
               items: [54, 65, 75, 74],
               operation: {:add, 6},
               test: 19,
               if_true: 2,
               if_false: 0,
               activity: 0
             },
             %{
               items: [79, 60, 97],
               operation: {:square},
               test: 13,
               if_true: 1,
               if_false: 3,
               activity: 0
             },
             %{items: [74], operation: {:add, 3}, test: 17, if_true: 0, if_false: 1, activity: 0}
           ]
  end
end
