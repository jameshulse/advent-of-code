# tree_input = """..##.......
# #...#...#..
# .#....#..#.
# ..#.#...#.#
# .#...##..#.
# ..#.##.....
# .#.#.#....#
# .#........#
# #.##...#...
# #...##....#
# .#..#...#.#""".lines

tree_input = File.read('input').lines

def count_trees(lines, right, down=1)
    x = 0
    y = 0
    trees = 0

    while y < lines.length()
        l = lines[y].strip()

        if l[x] == '#'
            trees += 1
        end

        x = (x + right) % l.length
        y += down
    end

    return trees
end

puts [ \
    count_trees(tree_input, 1), \
    count_trees(tree_input, 3), \
    count_trees(tree_input, 5), \
    count_trees(tree_input, 7), \
    count_trees(tree_input, 1, 2) \
].reduce(:*)