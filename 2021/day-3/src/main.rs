use indoc::indoc;
use itertools::Itertools;

fn main() {
    let lines = include_str!("input");

    dbg!(part_one(&lines));
}

fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect_vec();
    let report_count = lines.len();
    let report_width = lines[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..report_width {
        if lines
            .iter()
            .map(|&l| (&l[i..i + 1]).parse::<usize>().unwrap())
            .sum::<usize>()
            < report_count / 2
        {
            // More zeros
            epsilon |= 1 << report_width - i - 1;
        } else {
            // More ones
            gamma |= 1 << report_width - i - 1;
        }
    }

    gamma * epsilon
}

fn part_two(input: &str) -> () {}

#[test]
fn test_day_2() {
    let input = indoc! {"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010"
    };

    assert_eq!(part_one(input), 198);
    assert_eq!(part_two(input), 230);
}
