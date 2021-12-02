use itertools::Itertools;
use std::fs;

fn main() {
    let lines = fs::read_to_string("input").expect("Couldn't read input file.");

    dbg!(part_one(&lines));
    dbg!(part_two(&lines));
}

fn part_one(input: &str) -> usize {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;

    for line in input.lines() {
        let parts = &line.split(' ').collect_vec();
        let value = parts[1].parse::<usize>().unwrap();

        match parts[0] {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => continue,
        }
    }

    horizontal * depth
}

fn part_two(input: &str) -> usize {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    for line in input.lines() {
        let parts = &line.split(' ').collect_vec();
        let value = parts[1].parse::<usize>().unwrap();

        match parts[0] {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => continue,
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(part_one(input), 150);
    }

    #[test]
    fn test_part_two() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(part_two(input), 900);
    }
}
