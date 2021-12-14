use itertools::Itertools;
use std::cmp;
use std::fmt;

#[derive(fmt::Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_coords(coords: String) -> Self {
        coords
            .split_once(',')
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .map(|(x, y)| Self { x, y })
            .unwrap()
    }
}

fn main() {
    let lines = parse_input(include_str!("input"));

    dbg!(assert_eq!(part_one(&lines), 7414));
    dbg!(assert_eq!(part_two(&lines), 19676));
}

fn parse_input(raw: &str) -> Vec<(Point, Point)> {
    raw.lines()
        .map(|l| {
            let parts = l.split_once(" -> ").unwrap();

            (
                Point::from_coords(parts.0.to_string()),
                Point::from_coords(parts.1.to_string()),
            )
        })
        .collect()
}

// Determines the maximum size of 2D vector to store all lines
fn init_diagram(lines: &[(Point, Point)]) -> Vec<Vec<usize>> {
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        max_x = cmp::max(max_x, cmp::max(line.0.x, line.1.x));
        max_y = cmp::max(max_y, cmp::max(line.0.y, line.1.y));
    }

    vec![vec![0; max_x + 1]; max_y + 1]
}

fn count_intersections(diagram: &[Vec<usize>]) -> usize {
    diagram
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&v| *v > 1)
        .count()
}

// TODO: Any improvements in logic?
fn mark_line(diagram: &mut Vec<Vec<usize>>, p_from: &Point, p_to: &Point) -> usize {
    let mut max_seen = 0;
    let mut x = p_from.x;
    let mut y = p_from.y;

    loop {
        diagram[y][x] += 1;

        if diagram[y][x] > max_seen {
            max_seen = diagram[y][x];
        }

        if x == p_to.x && y == p_to.y {
            break;
        }

        if x < p_to.x {
            x += 1;
        } else if x > p_to.x {
            x -= 1;
        }

        if y < p_to.y {
            y += 1;
        } else if y > p_to.y {
            y -= 1;
        }
    }

    max_seen
}

fn visualize_diagram(diagram: &[Vec<usize>]) {
    for row in diagram.iter() {
        println!(
            "{}",
            row.iter()
                .map(|v| if *v == 0 {
                    ".".to_string()
                } else {
                    v.to_string()
                })
                .join("")
        )
    }
}

fn part_one(input: &[(Point, Point)]) -> usize {
    let mut diagram: Vec<Vec<usize>> = init_diagram(input);
    let mut max_value = 0;

    for line in input {
        // Handle straight lines only
        if line.0.x == line.1.x || line.0.y == line.1.y {
            let local_max = mark_line(&mut diagram, &line.0, &line.1);

            max_value = cmp::max(max_value, local_max);
        }
    }

    count_intersections(&diagram)
}

fn part_two(input: &[(Point, Point)]) -> usize {
    let mut diagram: Vec<Vec<usize>> = init_diagram(input);
    let mut max_value = 0;

    for line in input {
        let local_max = mark_line(&mut diagram, &line.0, &line.1);

        max_value = if local_max > max_value {
            local_max
        } else {
            max_value
        };
    }

    count_intersections(&diagram)
}

#[cfg(test)]
use indoc::indoc;

#[test]
fn test_parts() {
    let input = parse_input(indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "});

    assert_eq!(part_one(&input), 5);
    assert_eq!(part_two(&input), 12);
}

#[test]
fn test_from_coords() {
    let result = Point::from_coords("1,2".to_string());
    let expected = Point { x: 1, y: 2 };

    assert_eq!(result.x, expected.x);
    assert_eq!(result.y, expected.y);
}

#[test]
fn test_count_intersections() {
    let diagram = vec![vec![0, 2, 5], vec![1, 5, 5], vec![3, 4, 5], vec![0, 5, 5]];

    assert_eq!(count_intersections(&diagram), 9);
}

#[test]
fn test_mark_line() {
    let mut diagram = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let test_from = Point { x: 4, y: 0 };
    let test_to = Point { x: 0, y: 0 };

    let max_seen = mark_line(&mut diagram, &test_from, &test_to);

    assert_eq!(max_seen, 1);
    assert_eq!(diagram[0], vec![1, 1, 1, 1, 1]);
    assert_eq!(diagram[1], vec![0, 0, 0, 0, 0]);
}
