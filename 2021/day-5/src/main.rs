use indoc::indoc;
use itertools::Itertools;
use std::cmp;
use std::fmt;

#[derive(fmt::Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn make_point(coords: String) -> Point {
    let parts = coords.split(',').collect_vec();

    Point {
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap(),
    }
}

fn main() {
    let lines = parse_input(include_str!("input"));

    dbg!(part_one(&lines));
    dbg!(part_two(&lines));
}

fn parse_input(raw: &str) -> Vec<String> {
    raw.lines().map(|line| line.to_string()).collect()
}

fn get_diagram_size(lines: &[(Point, Point)]) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        if line.0.x > max_x {
            max_x = line.0.x;
        }
        if line.0.y > max_y {
            max_y = line.0.y
        }
        if line.1.x > max_x {
            max_x = line.1.x;
        }
        if line.1.y > max_y {
            max_y = line.1.y
        }
    }

    (max_x + 1, max_y + 1)
}

fn count_with_value(diagram: &Vec<Vec<usize>>, search_val: usize) -> usize {
    let mut with_val = 0;

    for row in diagram.iter() {
        for val in row.iter() {
            with_val += if *val == search_val { 1 } else { 0 };
        }
    }

    with_val
}

fn mark_line(diagram: &mut Vec<Vec<usize>>, p_from: &Point, p_to: &Point) -> usize {
    let mut max_seen = 0;

    for y in (cmp::min(p_from.y, p_to.y))..(cmp::max(p_from.y, p_to.y) + 1) {
        for x in (cmp::min(p_from.x, p_to.x))..(cmp::max(p_from.x, p_to.x) + 1) {
            diagram[y][x] += 1;

            if diagram[y][x] > max_seen {
                max_seen = diagram[y][x];
            }
        }
    }

    max_seen
}

fn part_one(input: &[String]) -> usize {
    let all_lines = input
        .iter()
        .map(|l| {
            let parts = l.split(' ').collect_vec();

            (
                make_point(parts[0].to_string()),
                make_point(parts[2].to_string()),
            )
        })
        .collect_vec();
    let size = get_diagram_size(&all_lines);

    let mut diagram: Vec<Vec<usize>> = vec![vec![0; size.0]; size.1];
    let mut max_value = 0;

    dbg!(size);
    dbg!(all_lines.len());

    for line in all_lines {
        let p_from = line.0;
        let p_to = line.1;

        // Find straight lines
        if p_from.x == p_to.x {
            println!("Straight line x= ({:?}) to ({:?})", p_from, p_to);

            for y in (cmp::min(p_from.y, p_to.y))..(cmp::max(p_from.y, p_to.y) + 1) {
                diagram[y][p_from.x] += 1;

                if diagram[y][p_from.x] > max_value {
                    max_value = diagram[y][p_from.x];
                }
            }
        } else if p_from.y == p_to.y {
            println!("Straight line y= ({:?}) to ({:?})", p_from, p_to);

            for x in (cmp::min(p_from.x, p_to.x))..(cmp::max(p_from.x, p_to.x) + 1) {
                diagram[p_from.y][x] += 1;

                if diagram[p_from.y][x] > max_value {
                    max_value = diagram[p_from.y][x];
                }
            }
        }
    }

    dbg!(&diagram);
    dbg!(max_value);

    count_with_value(&diagram, max_value)
}

fn part_two(input: &[String]) -> usize {
    todo!();
}

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
    // assert_eq!(part_two(&input), 0);
}

#[test]
fn test_make_point() {
    let result = make_point("1,2".to_string());
    let expected = Point { x: 1, y: 2 };

    assert_eq!(result.x, expected.x);
    assert_eq!(result.y, expected.y);
}

#[test]
fn test_count_with_value() {
    let diagram = vec![vec![0, 2, 5], vec![0, 5, 5], vec![3, 4, 5], vec![0, 5, 5]];

    assert_eq!(count_with_value(&diagram, 0), 3);
    assert_eq!(count_with_value(&diagram, 2), 1);
    assert_eq!(count_with_value(&diagram, 3), 1);
    assert_eq!(count_with_value(&diagram, 4), 1);
    assert_eq!(count_with_value(&diagram, 5), 6);
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
