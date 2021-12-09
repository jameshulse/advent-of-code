use itertools::{iproduct, Itertools};

fn main() {
    let input = include_str!("input").to_string();

    dbg!(assert_eq!(part_one(&input), 452));
    dbg!(assert_eq!(part_two(&input), 1263735));
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(&str::chars)
        .map(|chars| chars.map(|ch| (ch as u8) - b'0').collect())
        .collect_vec()
}

fn part_one(input: &str) -> usize {
    let lines = parse_input(input);

    find_low_points(&lines)
        .iter()
        .map(|(x, y)| lines.get(*y).unwrap().get(*x).unwrap())
        .fold(0, |a, n| a + (*n as usize) + 1)
}

fn get_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    [
        (x > 0, ((x as isize) - 1, (y as isize))),
        (x < width - 1, ((x as isize) + 1, (y as isize))),
        (y > 0, ((x as isize), (y as isize) - 1)),
        (y < height - 1, ((x as isize), (y as isize) + 1)),
    ]
    .into_iter()
    .filter_map(|(cond, val)| {
        if cond {
            Some((val.0 as usize, val.1 as usize))
        } else {
            None
        }
    })
    .collect_vec()
}

fn find_low_points(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let width = map[0].len(); // TODO: Better way to get width
    let height = map.len();

    iproduct!(0..height, 0..width)
        .filter(|&(y, x)| {
            get_neighbours(x, y, width, height)
                .into_iter()
                .all(|(cx, cy)| map[cy][cx] > map[y][x])
        })
        .map(|(y, x)| (x, y))
        .collect_vec()
}

fn fill_basin(
    map: &[Vec<u8>],
    visited: &mut Vec<(usize, usize)>,
    start_x: usize,
    start_y: usize,
) -> usize {
    if visited.contains(&(start_x, start_y)) {
        return 0;
    }

    visited.push((start_x, start_y));

    let n = map[start_y][start_x];
    let width = map[0].len(); // TODO: Build these into the 'map' object
    let height = map.len();

    get_neighbours(start_x, start_y, width, height)
        .into_iter()
        .filter(|&(x, y)| map[y][x] > n && map[y][x] != 9)
        .fold(1, |a, (x, y)| a + fill_basin(map, visited, x, y))
}

fn part_two(input: &str) -> usize {
    let map = parse_input(input);

    find_low_points(&map)
        .iter()
        .map(|p| fill_basin(&map, &mut Vec::<(usize, usize)>::new(), p.0, p.1))
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
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
    let map = parse_input(input);

    assert_eq!(fill_basin(&map, &mut Vec::<(usize, usize)>::new(), 5, 0), 9);
}

#[test]
fn test_get_neighbours() {
    assert_eq!(get_neighbours(0, 0, 5, 5), vec![(1, 0), (0, 1)]);
}

#[test]
fn test_find_low_points() {
    let input = indoc! {"
        210
        921
    "};
    let map = parse_input(input);

    assert_eq!(find_low_points(&map), vec![(2, 0)]);
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
