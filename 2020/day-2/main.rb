# input = """\
# 1-3 a: abcde
# 1-3 b: cdefg
# 2-9 c: ccccccccc
# """

input = File.open('input').readlines

# Part 1
def check_letter_range(line)
    (rules, password) = line.split(": ")
    (range, letter) = rules.split(" ")
    (low, high) = range.split("-")

    # puts "Requires %d to %d [%s] in password [%s]" % [low, high, letter, password.strip]

    password.strip.count(letter).between?(low.to_i, high.to_i)
end

part_one = input.inject(0) { |sum, line| sum + (check_letter_range(line) ? 1 : 0) }

puts "[Part 1] Valid passwords: " + part_one.to_s

# Part 2
def check_letter_positions(line)
    (rules, password) = line.split(": ")
    (range, letter) = rules.split(" ")
    (first_position, second_position) = range.split("-")

    in_first_position = password[first_position.to_i - 1] == letter
    in_second_position = password[second_position.to_i - 1] == letter

    in_first_position ^ in_second_position
end

part_two = input.inject(0) { |sum, line| sum + (check_letter_positions(line) ? 1 : 0) }

puts "[Part 2] Valid passwords: " + part_two.to_s