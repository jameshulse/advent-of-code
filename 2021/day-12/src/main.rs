use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input").to_string();

    dbg!(assert_eq!(part_one(&input), 3708));
    // dbg!(assert_eq!(part_two(&input), 0));
}

fn part_one(input: &str) -> usize {
    let graph = parse_input(input);

    traverse(&graph, &"start".to_owned(), &mut vec![])
}

// Recurse through cave
fn traverse<'a>(graph: &Graph, current_name: &'a String, visited: &mut Vec<&'a String>) -> usize {
    let mut paths = 0;

    // println!("Visiting {}", current_name);

    visited.push(current_name);

    if current_name == "end" {
        println!("Found path to end: {}", visited.iter().join(","));
        return 1;
    }

    let current_node = graph.find(current_name);

    for neighbour_name in &current_node.connections {
        let neighbour_node = graph.find(neighbour_name);

        println!(
            "Current: {}, Neighbour: {}, Visited: {}",
            current_name,
            neighbour_name,
            visited.iter().join(",")
        );

        // Don't traverse past small nodes that have been visited
        if !neighbour_node.is_large && visited.contains(&neighbour_name) {
            continue;
        }

        paths += traverse(graph, neighbour_name, &mut visited.clone());
    }

    paths
}

// fn part_two(input: &str) -> usize {
//     0
// }

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
    fn add_edge(&mut self, from: &str, to: &str) {
        let caves = &mut self.caves;
        let from_cave = caves
            .entry(from.to_owned())
            .or_insert_with(|| Cave::new(from.to_owned()));

        from_cave.connections.push(to.to_owned());

        let to_cave = caves
            .entry(to.to_owned())
            .or_insert_with(|| Cave::new(to.to_owned()));

        if from != "start" && to != "end" {
            to_cave.connections.push(from.to_owned());
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
        assert_eq!(graph.caves["b"].connections, vec!["d", "end"]);
    }

    #[test]
    fn test_traverse_medium() {
        let graph = parse_input(indoc! {"
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
        "});

        assert_eq!(traverse(&graph, &"start".to_string(), &mut vec![]), 19);
    }

    #[test]
    fn test_traverse_large() {
        let graph = parse_input(indoc! {"
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
        "});

        assert_eq!(traverse(&graph, &"start".to_string(), &mut vec![]), 226);
    }
}
