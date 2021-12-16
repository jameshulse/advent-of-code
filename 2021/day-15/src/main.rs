use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::{cell::RefCell, collections::HashMap};

fn main() {
    let input = include_str!("input");

    dbg!(assert_eq!(part_one(input), 685));
    dbg!(assert_eq!(part_two(input), 0));
}

// #[derive(Debug)]
// struct Cavern {
//     map: Vec<Vec<u8>>,
//     nodes: RefCell<HashMap<(usize, usize), Box<Node>>>,
// }

// #[derive(Copy, Clone, Eq/*, Ord*/, PartialEq, Hash, Debug)]
// struct Node {
//     x: usize,
//     y: usize,
//     risk: usize,
// }

// impl Cavern {
//     fn new(map: Vec<Vec<u8>>) -> Cavern {
//         Cavern {
//             map,
//             nodes: RefCell::new(HashMap::new())
//         }
//     }

//     fn height(&self) -> usize {
//         self.map.len()
//     }

//     fn width(&self) -> usize {
//         self.map.len()
//     }

//     fn get_node(&self, x: usize, y: usize) -> &Box<Node> {
//         let mut nodes = self.nodes.borrow_mut();

//         nodes.entry((x, y)).or_insert_with(|| Box::new(Node { x, y, risk: self.map[y][x] as usize }));

//         self.nodes.borrow()[&(x, y)]
//     }

//     fn find_neighbours(&mut self, node: &Node) -> Vec<&Node> {
//         [
//             // (node.x > 0, (node.x - 1, node.y))
//             // (node.x < self.width() - 1, (node.x + 1, node.y)),
//             // (node.y > 0, (node.x, node.y - 1)),
//             // (node.y < self.height() - 1, (node.x, node.y + 1)),
//         ]
//             .iter()
//             .flat_map(|&(f, (x, y))| {
//                 if f { Some(self.get_node(x, y).as_ref()) } else { None }
//             })
//             .collect_vec()
//     }
// }

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let map = input
        .lines()
        .map(|l| l.bytes().map(|ch| (ch - b'0') as usize).collect_vec())
        .collect::<Vec<Vec<usize>>>();

    map
}

fn part_one(input: &str) -> usize {
    let cavern = parse_input(input);
    let start = (0, 0);
    let end = (cavern.len() - 1, cavern.len() - 1);

    let shortest_path = find_shortest_path(&cavern, start, end);

    shortest_path.unwrap().1
}

fn find_shortest_path(
    map: &[Vec<usize>],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(Vec<(usize, usize)>, usize)> {
    dijkstra(
        &start,
        |&(x, y)| {
            [
                (x > 0, (x - 1, y)),
                (x < map.len() - 1, (x + 1, y)),
                (y > 0, (x, y - 1)),
                (y < map.len() - 1, (x, y + 1)),
            ]
            .iter()
            .flat_map(|&(f, (nx, ny))| {
                if f {
                    Some(((nx, ny), map[ny][nx]))
                } else {
                    None
                }
            })
            .collect_vec()
        },
        |&n| n == end,
    )
}

fn expand_map(map: Vec<Vec<usize>>, times: usize) -> Vec<Vec<usize>> {
    vec![]
}

fn part_two(input: &str) -> usize {
    let cavern = expand_map(parse_input(input), 5);
    let start = (0, 0);
    let end = (cavern.len() - 1, cavern.len() - 1);

    let shortest_path = find_shortest_path(&cavern, start, end);

    shortest_path.unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn test_expand_map() {
        let input = indoc! {"
            789
            789
            789
        "};

        let expanded = expand_map(parse_input(input), 1);

        assert_eq!(expanded, vec![vec![7, 8, 9, 8, 9, 1]])
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
