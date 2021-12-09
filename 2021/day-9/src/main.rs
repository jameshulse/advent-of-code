use itertools::Itertools;

fn main() {
    let input = include_str!("input").to_string();

    dbg!(assert_eq!(part_one(&input), 452));
    dbg!(assert_eq!(part_two(&input), 1263735));
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(&str::chars)
        .map(|chars| {
            chars
                .map(|ch| ch.to_string().parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn part_one(input: &str) -> usize {
    let lines = parse_input(input);

    find_low_points(&lines)
        .iter()
        .map(|(x, y)| lines.get(*y).unwrap().get(*x).unwrap())
        .fold(0, |a, n| a + n + 1)
}

fn find_low_points(lines: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            let mut compare_to: Vec<(usize, usize)> = vec![];

            if x > 0 {
                compare_to.push((x - 1, y));
            }

            if x < line.len() - 1 {
                compare_to.push((x + 1, y))
            }

            if y > 0 {
                compare_to.push((x, y - 1));
            }

            if y < lines.len() - 1 {
                compare_to.push((x, y + 1));
            }

            let is_low = compare_to.iter().all(|(cx, cy)| {
                lines
                    .get(*cy)
                    .ok_or(format!("Failed for num {} at ({}, {})", num, x, y))
                    .unwrap()
                    .get(*cx)
                    .ok_or(format!("Failed for num {} at ({}, {})", num, x, y))
                    .unwrap()
                    > num
            });

            if is_low {
                result.push((x, y));
            }
        }
    }

    result
}

fn fill_basin(
    map: &Vec<Vec<usize>>,
    visited: &mut Vec<(usize, usize)>,
    start_x: usize,
    start_y: usize,
) -> usize {
    let get_val = |x, y| map.get::<usize>(y).unwrap().get::<usize>(x).unwrap();
    let n = get_val(start_x, start_y);
    let mut size = 1;

    visited.push((start_x, start_y));

    // println!("Visiting: {} at ({},{})", n, start_x, start_y);

    // Left
    if start_x > 0
        && !visited.contains(&(start_x - 1, start_y))
        && get_val(start_x - 1, start_y) > n
        && get_val(start_x - 1, start_y) != &(9 as usize)
    {
        size += fill_basin(&map, visited, start_x - 1, start_y);
    }

    // Right
    if start_x < map.get(start_y).unwrap().len() - 1
        && !visited.contains(&(start_x + 1, start_y))
        && get_val(start_x + 1, start_y) > n
        && get_val(start_x + 1, start_y) != &(9 as usize)
    {
        size += fill_basin(&map, visited, start_x + 1, start_y);
    }

    // Up
    if start_y > 0
        && !visited.contains(&(start_x, start_y - 1))
        && get_val(start_x, start_y - 1) > n
        && get_val(start_x, start_y - 1) != &(9 as usize)
    {
        size += fill_basin(&map, visited, start_x, start_y - 1);
    }

    // Down
    if start_y < map.len() - 1
        && !visited.contains(&(start_x, start_y + 1))
        && get_val(start_x, start_y + 1) > n
        && get_val(start_x, start_y + 1) != &(9 as usize)
    {
        size += fill_basin(&map, visited, start_x, start_y + 1);
    }

    size
}

fn part_two(input: &str) -> usize {
    let map = parse_input(input);
    let low_points = find_low_points(&map);
    let mut basin_sizes: Vec<usize> = vec![];

    for p in low_points {
        basin_sizes.push(fill_basin(
            &map,
            &mut Vec::<(usize, usize)>::new(),
            p.0,
            p.1,
        ));
    }

    basin_sizes
        .iter()
        .sorted()
        .rev()
        .take(3)
        .fold(1, |a, s| a * s)
}

#[cfg(test)]
use indoc::indoc;

#[test]
fn test_fill_basin() {
    let input = indoc! {"
        943210
        894921
        789892
        896789
    "};
    let map = parse_input(&input);

    assert_eq!(fill_basin(&map, &mut Vec::<(usize, usize)>::new(), 5, 0), 9);
}

#[test]
fn test_parts() {
    let input = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    assert_eq!(part_one(input), 15);
    assert_eq!(part_two(input), 1134);
}
