// use std::{cell::RefCell, collections::HashMap};

// TODO

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