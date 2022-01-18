# input = "BBFFBBFRLL"
input = File.read('input')

def parse_seat(line)
    row_instructions = line[0..6].chars.map { |ch| ch == 'F' }
    col_instructions = line[7..].chars.map { |ch| ch == 'L' }
    
    row_loc = find_loc(row_instructions, 0, 127)
    col_loc = find_loc(col_instructions, 0, 7)

    # puts row_loc
    # puts col_loc

    row_loc * 8 + col_loc
end

def find_loc(instructions, low, high)
    take_low = instructions.shift

    if high - low == 1
        return take_low ? low : high
    end

    if take_low
        high -= ((high.to_f - low.to_f) / 2.0).ceil()
    else
        low += ((high.to_f - low.to_f) / 2.0).ceil()
    end

    find_loc(instructions, low, high)
end

part_one = input.lines.map { |line| parse_seat(line) }.max()

puts "Part one: %d" % [part_one]

non_consec = input.lines.map { |line| parse_seat(line) }.sort.each_cons(2).detect { |a,b| b - a == 2 }

part_two = non_consec[0] + 1

puts "Part two: %s -> %s" % [non_consec, part_two]