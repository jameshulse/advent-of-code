# input = """\
# abc

# a
# b
# c

# ab
# ac

# a
# a
# a
# a

# b
# """

input = File.read('input')

part_one = input.split(/(\n\n)/).each_slice(2).map { |answers, *| answers.delete("\n").chars.uniq.count }.sum

puts "Part one: %d" % [part_one]

def count_all_yes(answers)
    respondents = answers.lines.count

    answers.delete("\n").chars.group_by { |ch| ch }.map { |k,v| v.length }.filter { |v| v == respondents }.count
end

part_two = input.split(/(\n\n)/).each_slice(2).map { |answers, *| count_all_yes(answers) }.sum

puts "Part two: %d" % [part_two]