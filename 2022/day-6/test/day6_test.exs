defmodule Day6Test do
  use ExUnit.Case

  test "part 1" do
    assert Day6.part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 7
    assert Day6.part1("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5
    assert Day6.part1("nppdvjthqldpwncqszvftbrmjlhg") == 6
    assert Day6.part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10
    assert Day6.part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11
  end

  test "part 2" do
    assert Day6.part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 19
    assert Day6.part2("bvwbjplbgvbhsrlpgdmjqwftvncz") == 23
    assert Day6.part2("nppdvjthqldpwncqszvftbrmjlhg") == 23
    assert Day6.part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 29
    assert Day6.part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 26
  end

  test "is_marker" do
    assert Day6.is_marker(['a', 'b', 'b', 'a']) == false
    assert Day6.is_marker(['a', 'b', 'c', 'd']) == true
  end

  test "real input" do
    input = File.read!("input")

    assert Day6.part1(input) == 1876
    assert Day6.part2(input) == 2202
  end
end
