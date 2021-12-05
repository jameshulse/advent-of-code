use indoc::indoc;
use itertools::Itertools;

fn main() {
    let lines = include_str!("input");

    dbg!(part_one(lines));
    dbg!(part_two(lines));
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
            .map(|l| (l[i..i + 1]).parse::<usize>().unwrap())
            .sum::<usize>()
            < report_count / 2
        {
            // More zeros
            epsilon |= 1 << (report_width - i - 1);
        } else {
            // More ones
            gamma |= 1 << (report_width - i - 1);
        }
    }

    gamma * epsilon
}

enum Commonality {
    Most,
    Least,
}

fn part_two(input: &str) -> usize {
    let oxygen = find_report(input, Commonality::Most);
    let scrubber = find_report(input, Commonality::Least);

    oxygen * scrubber
}

fn find_report(input: &str, commonality: Commonality) -> usize {
    let mut lines: Vec<&str> = input.lines().collect_vec();

    for i in 0..lines[0].len() {
        let ones = lines
            .iter()
            .map(|l| (l[i..i + 1]).parse::<usize>().unwrap())
            .sum::<usize>();
        let zeros = lines.len() - ones;
        let find_digit = match commonality {
            Commonality::Most => {
                if ones >= zeros {
                    "1"
                } else {
                    "0"
                }
            }
            Commonality::Least => {
                if ones >= zeros {
                    "0"
                } else {
                    "1"
                }
            }
        };

        lines.retain(|&l| &l[i..i + 1] == find_digit);

        if lines.len() == 1 {
            println!("One line remaining: {:?}", lines[0]);

            return usize::from_str_radix(lines[0], 2).unwrap();
        }
    }

    0
}

#[test]
fn test_parts() {
    let input = indoc! {"
        00100
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
