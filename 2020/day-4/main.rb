input = """\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"""

input = File.read('input')

# Part 1
def contains_all_fields(passport)
    valid = ['byr:', 'iyr:', 'eyr:', 'hgt:', 'hcl:', 'ecl:', 'pid:'].all? do |field|
        passport.include?(field)
    end

    valid
end

part_one = input.split(/(\n\n)/).each_slice(2).map { |p, *| contains_all_fields(p) }.count(true)

puts "Part one: %d" % [part_one]

# Part 2
def validate_passport(raw_passport)
    passport = raw_passport.split(' ').map { |e| e.split(':') }.to_h

    passport['byr'].to_i.between?(1929, 2002) \
        && passport['iyr'].to_i.between?(2010, 2020) \
        && passport['eyr'].to_i.between?(2020, 2030)

    # todo...
end

part_two = input.split(/(\n\n)/).each_slice(2).map { |p, *| validate_passport(p) }.count(true)

puts "Part two: %d" % [part_two]