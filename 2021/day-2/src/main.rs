use itertools::Itertools;

fn main() {
    let lines = include_str!("input");

    dbg!(part_one(&lines));
    dbg!(part_two(&lines));
}

// Converts e.g "forward 5" to ("foward", 5)
fn parse_command(line: &str) -> (&str, usize) {
    let parts = &line.split_whitespace().collect_vec();

    (parts[0], parts[1].parse::<usize>().unwrap())
}

fn part_one(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;

    for (command, value) in input.lines().map(parse_command) {
        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => continue,
        }
    }

    horizontal * depth
}

fn part_two(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (command, value) in input.lines().map(parse_command) {
        match command {
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
    fn test_parts() {
        let input = include_str!("input.test");

        assert_eq!(part_one(input), 150);
        assert_eq!(part_two(input), 900);
    }
}
