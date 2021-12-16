use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::cmp;

fn main() {
    let input = include_str!("input");

    dbg!(assert_eq!(part_one(input), 685));
    dbg!(assert_eq!(part_two(input), 0));
}

type Cavern = Vec<Vec<usize>>;

fn parse_input(input: &str) -> Cavern {
    let map = input
        .lines()
        .map(|l| l.bytes().map(|ch| (ch - b'0') as usize).collect_vec())
        .collect::<Cavern>();

    map
}

fn part_one(input: &str) -> usize {
    let cavern = parse_input(input);
    let start = (0, 0);
    let end = (cavern.len() - 1, cavern.len() - 1);

    let shortest_path = dijkstra(
        &start,
        |&(x, y)| {
            [
                (x > 0, (x - 1, y)),
                (x < cavern.len() - 1, (x + 1, y)),
                (y > 0, (x, y - 1)),
                (y < cavern.len() - 1, (x, y + 1)),
            ]
            .iter()
            .flat_map(|&(f, (nx, ny))| {
                if f {
                    Some(((nx, ny), cavern[ny][nx]))
                } else {
                    None
                }
            })
            .collect_vec()
        },
        |&n| n == end,
    );

    shortest_path.unwrap().1
}

fn get_risk_with_growth(cavern: &Cavern, x: usize, y: usize, ) -> usize {
    const MAX_RISK: usize = 9;

    let mapped_y = y % cavern.len();
    let mapped_x = x % cavern.len();
    
    // dbg!(cavern.len(), x, y, mapped_x, mapped_y);
    // dbg!(cavern);

    let original_risk = cavern[mapped_y][mapped_x];
    let distance: usize = y / cavern.len() + x / cavern.len();

    // dbg!(original_risk, distance);

    cmp::max(1, (original_risk + distance) % MAX_RISK)
}

fn part_two(input: &str) -> usize {
    const GROWTH_FACTOR: usize = 5;

    let cavern = parse_input(input);
    let start = (0, 0);
    let end = (cavern.len() * GROWTH_FACTOR - 1, cavern.len() * GROWTH_FACTOR - 1);

    let shortest_path = dijkstra(
        &start,
        |&(x, y)| {
            [
                (x > 0, (x - 1, y)),
                (x < cavern.len() * GROWTH_FACTOR - 1, (x + 1, y)),
                (y > 0, (x, y - 1)),
                (y < cavern.len() * GROWTH_FACTOR - 1, (x, y + 1)),
            ]
            .iter()
            .flat_map(|&(f, (nx, ny))| {
                if f {
                    Some(((nx, ny), get_risk_with_growth(&cavern, nx, ny)))
                } else {
                    None
                }
            })
            .collect_vec()
        },
        |&n| n == end,
    );

    shortest_path.unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    /**
     891912123
     891912123
     891912123
     912123234
     912123234
     912123234
     123234345
     123234345
     123234345
     */

    #[test]
    fn test_get_risk_with_growth() {
        let cavern = vec![
            vec![8, 9, 1],
            vec![8, 9, 1],
            vec![8, 9, 1]
        ];

        assert_eq!(get_risk_with_growth(&cavern, 4, 5), 2);
        assert_eq!(get_risk_with_growth(&cavern, 7, 8), 4);
    }

    #[test]
    fn test_parts() {
        let input = indoc! {"
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581
        "};

        assert_eq!(part_one(input), 40);
        assert_eq!(part_two(input), 315);
    }
}
