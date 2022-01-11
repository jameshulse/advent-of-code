# input = """\
# ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
# byr:1937 iyr:2017 cid:147 hgt:183cm

# iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
# hcl:#cfa07d byr:1929

# hcl:#ae17e1 iyr:2013
# eyr:2024
# ecl:brn pid:760753108 byr:1931
# hgt:179cm

# hcl:#cfa07d eyr:2025 pid:166559648
# iyr:2011 ecl:brn hgt:59in
# """

input = File.read('input')

# Part 1
def contains_all_fields(passport)
    valid = ['byr:', 'iyr:', 'eyr:', 'hgt:', 'hcl:', 'ecl:', 'pid:'].all? do |field|
        passport.include?(field)
    end

    valid
end

part_one = input.split(/(\n\n)/).each_slice(2).inject(0) { |n, p| n + (contains_all_fields(p[0]) ? 1 : 0) }

puts "Part one: %d" % part_one

# Part 2
puts "Part two: todo"