input = """\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###\
"""

algo = input[..512].split('').map {|ch| ch == '#' }

Pixel = Struct.new(:x, :y)

input_image = {}

input[514..].split("\n").each.with_index do |line, y|
    line.split('').each.with_index do |ch, x|
        input_image[[x, y]] = ch == '#'
    end
end

# output_image = nil

def enhance(img)
    img.each do |key, on|
        x, y = key
        kernel = ""
        
        [
            [x - 1, y - 1],
            [x, y -1],
            [x + 1, y - 1],
            [x - 1, y],
            [x, y],
            [x + 1, y],
            [x - 1, y + 1],
            [x, y + 1],
            [x + 1, y + 1],
        ].each do |loc|
            kernel += img.key?(loc) ? "1" : "0"
        end

        idx = kernel.to_i(2)

        algo[kernel.to_i(2)]
    end
end

output_image = enhance(enhance(input_image))

puts "Result: " + output_image.reduce(0) { |r, (_, on)| r + (on ? 1 : 0) }.to_s

# puts input_image