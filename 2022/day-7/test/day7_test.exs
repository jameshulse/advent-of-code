defmodule Day7Test do
  use ExUnit.Case

  @test_input """
  $ cd /
  $ ls
  dir a
  14848514 b.txt
  8504156 c.dat
  dir d
  $ cd a
  $ ls
  dir e
  29116 f
  2557 g
  62596 h.lst
  $ cd e
  $ ls
  584 i
  $ cd ..
  $ cd ..
  $ cd d
  $ ls
  4060174 j
  8033020 d.log
  5626152 d.ext
  7214296 k
  """

  test "both parts" do
    input = File.read!("input")

    assert Day7.part1(input) == 1_077_191
    assert Day7.part2(input) == -1
  end

  test "part 1" do
    assert Day7.part1(@test_input) == 95437
  end

  test "part 2" do
    assert Day7.part2(@test_input) == 24_933_642
  end

  describe "parse_command" do
    test "handles changing into a directory" do
      fs = %{"/" => 0}

      assert Day7.parse_command("$ cd a", {"/", fs}) == {"/a", %{"/" => 0, "/a" => 0}}
    end

    test "handles going up a directory" do
      fs = %{"/" => 0, "/a/b" => 0}

      assert Day7.parse_command("$ cd ..", {"/a/b", fs}) == {"/a", fs}
    end

    test "adds file sizes to current directory size" do
      fs = %{"/" => 0, "/a" => 0}

      assert Day7.parse_command("10 d.log", {"/a", fs}) == {"/a", %{"/" => 0, "/a" => 10}}
    end
  end
end
