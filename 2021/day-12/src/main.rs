use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input").to_string();

    dbg!(assert_eq!(part_one(&input), 3708));
    dbg!(assert_eq!(part_two(&input), 0));
}

fn part_one(input: &str) -> usize {
    let graph = parse_input(input);

    traverse(&graph, &"start".to_owned(), &mut vec![], false)
}

// Recurse through cave
fn traverse<'a>(
    graph: &Graph,
    current_name: &'a String,
    visited: &mut Vec<&'a String>,
    advanced_visiting: bool,
) -> usize {
    let mut paths = 0;

    visited.push(current_name);

    if current_name == "end" {
        // println!("Found path to end: {}", visited.iter().join(","));
        return 1;
    }

    let current_node = graph.find(current_name);

    for neighbour_name in &current_node.connections {
        let neighbour_node = graph.find(neighbour_name);

        if !neighbour_node.is_large {
            if advanced_visiting {
                // Advanced rules: can visit 1 small cave twice
                let twice_visited = visited
                    .iter()
                    .unique()
                    .filter_map(|&v| {
                        if v.to_lowercase() == *v {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .map(|v| visited.iter().filter(|&&v2| v2 == v).count())
                    .any(|c| c > 1);

                if twice_visited && visited.contains(&neighbour_name) {
                    continue;
                }
            } else if visited.contains(&neighbour_name) {
                // Basic rules: can't visit the same small node twice
                continue;
            }
        }

        paths += traverse(
            graph,
            neighbour_name,
            &mut visited.clone(),
            advanced_visiting,
        );
    }

    paths
}

fn part_two(input: &str) -> usize {
    let graph = parse_input(input);

    traverse(&graph, &"start".to_owned(), &mut vec![], true)
}

#[derive(Hash, Debug)]
struct Cave {
    name: String,
    is_large: bool,
    connections: Vec<String>,
}

impl Cave {
    fn new(name: String) -> Cave {
        let is_large = name != "start" && name != "end" && name.to_ascii_uppercase() == name;

        Cave {
            name,
            is_large,
            connections: vec![],
        }
    }
}

#[derive(Debug)]
struct Graph {
    caves: HashMap<String, Cave>,
}

impl Graph {
    fn add_edge(&mut self, left: &str, right: &str) {
        let caves = &mut self.caves;
        let left_cave = caves
            .entry(left.to_owned())
            .or_insert_with(|| Cave::new(left.to_owned()));

        if right != "start" {
            left_cave.connections.push(right.to_owned());
        }

        let right_cave = caves
            .entry(right.to_owned())
            .or_insert_with(|| Cave::new(right.to_owned()));

        if left != "start" {
            right_cave.connections.push(left.to_owned());
        }
    }

    fn find(&self, name: &str) -> &Cave {
        &self.caves[name]
    }

    fn new() -> Graph {
        Graph {
            caves: HashMap::new(),
        }
    }
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph::new();

    input
        .lines()
        .map(|l| l.split('-').collect_tuple::<(&str, &str)>().unwrap())
        .for_each(|(from, to)| {
            graph.add_edge(from, to);
        });

    graph
}

mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parts() {
        let input = indoc! {"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "};

        assert_eq!(part_one(input), 10);
        assert_eq!(part_two(input), 36);
    }

    #[test]
    fn test_parse_input() {
        let graph = parse_input(indoc! {"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "});

        assert_eq!(graph.caves.len(), 6);
        assert_eq!(graph.caves["start"].connections, vec!["A", "b"]);
        assert_eq!(graph.caves["A"].connections, vec!["c", "b", "end"]);
        assert_eq!(graph.caves["b"].connections, vec!["A", "d", "end"]);
    }

    #[test]
    fn test_traverse_medium() {
        let input = indoc! {"
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        "};

        assert_eq!(part_one(input), 19);
        assert_eq!(part_two(input), 103);
    }

    #[test]
    fn test_traverse_large() {
        let input = indoc! {"
            fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW
        "};

        assert_eq!(part_one(input), 226);
        assert_eq!(part_two(input), 3509);
    }
}
