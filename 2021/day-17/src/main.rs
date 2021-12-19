use itertools::iproduct;
use regex::Regex;

fn main() {
    let input = include_str!("input");

    dbg!(assert_eq!(part_one(input), 17766));
    dbg!(assert_eq!(part_two(input), 1733));
}

fn part_one(input: &str) -> isize {
    let target = parse_input(input);
    let mut highest_point = 0;

    for (x_init_vel, y_init_vel) in iproduct!(0..1000, 0..1000) {
        let mut x = 0;
        let mut y = 0;
        let mut x_vel = x_init_vel;
        let mut y_vel = y_init_vel;
        let mut local_highest = 0;

        loop {
            if target.contains(x, y) {
                highest_point = highest_point.max(local_highest);
                break;
            }

            if x > target.x_to || y < target.y_to {
                break;
            }

            x += x_vel;
            y += y_vel;

            local_highest = local_highest.max(y);

            x_vel = (x_vel - 1).max(0);
            y_vel -= 1;
        }
    }

    highest_point
}

fn part_two(input: &str) -> isize {
    let target = parse_input(input);
    let mut hits = 0;

    for (x_init_vel, y_init_vel) in iproduct!(0..100, -200..1000) {
        hits += if fire(&target, x_init_vel, y_init_vel) { 1 } else { 0 };
    }

    hits
}

fn fire(target: &Target, x_init_vel: isize, y_init_vel: isize) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut x_vel = x_init_vel;
    let mut y_vel = y_init_vel;

    loop {
        if target.contains(x, y) {
            return true;
        }

        if x > target.x_to || y < target.y_from {
            break;
        }

        x += x_vel;
        y += y_vel;

        x_vel = (x_vel - 1).max(0);
        y_vel -= 1;
    }

    false
}

#[derive(Debug, PartialEq)]
struct Target {
    x_from: isize,
    x_to: isize,
    y_from: isize,
    y_to: isize,
}

impl Target {
    fn contains(&self, x: isize, y: isize) -> bool {
        x >= self.x_from && x <= self.x_to && y >= self.y_from && y <= self.y_to
    }
}

fn parse_input(input: &str) -> Target {
    let re = Regex::new(r"target area: x=(?P<xf>-?\d+)\.\.(?P<xt>-?\d+), y=(?P<yf>-?\d+)\.\.(?P<yt>-?\d+)").unwrap();
    let captures = &re.captures(input).unwrap();

    Target {
        x_from: captures.name("xf").unwrap().as_str().parse().unwrap(),
        x_to: captures.name("xt").unwrap().as_str().parse().unwrap(),
        y_from: captures.name("yf").unwrap().as_str().parse().unwrap(),
        y_to: captures.name("yt").unwrap().as_str().parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("target area: x=20..30, y=-10..-5"), Target {
            x_from: 20,
            x_to: 30,
            y_from: -10,
            y_to: -5,
        });

        assert_eq!(parse_input("target area: x=48..70, y=-189..-148"), Target {
            x_from: 48,
            x_to: 70,
            y_from: -189,
            y_to: -148
        });
    }

    #[test]
    fn test_parts() {
        assert_eq!(part_one("target area: x=20..30, y=-10..-5"), 45);
        assert_eq!(part_two("target area: x=20..30, y=-10..-5"), 112);
    }

    #[test]
    fn test_fire() {
        assert!(fire(&Target {
            x_from: 20,
            x_to: 30,
            y_from: -10,
            y_to: -5,
        }, 6, 0));
    }
}