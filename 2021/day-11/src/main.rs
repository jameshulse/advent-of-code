use itertools::{iproduct, Itertools};
use std::collections::VecDeque;
use std::fmt;

fn main() {
    let input = include_str!("input").to_string();

    dbg!(assert_eq!(part_one(&input), 1627));
    dbg!(assert_eq!(part_two(&input), 329));
}

fn part_one(input: &str) -> usize {
    let mut map = parse_input(input);

    (0..100).map(|_| simulate_step(&mut map)).sum()
}

fn part_two(input: &str) -> usize {
    let mut map = parse_input(input);
    let total = map.width * map.height;

    (0..usize::MAX)
        .map(|i| (i, simulate_step(&mut map)))
        .find(|&(_, f)| f == total)
        .unwrap()
        .0
        + 1
}

#[derive(PartialEq, Debug)]
struct Octopus {
    x: usize,
    y: usize,
    energy: u32,
}

impl Octopus {
    // Returns whether the octopus has flashed
    fn glow(&mut self) -> bool {
        self.energy += 1;

        if self.energy > 9 {
            self.energy = 0;

            return true;
        }

        false
    }
}

#[derive(PartialEq)]
struct Cavern {
    data: Vec<Vec<Octopus>>,
    width: usize,
    height: usize,
}

impl Cavern {
    fn get(&self, x: usize, y: usize) -> &Octopus {
        &self.data[y][x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Octopus {
        &mut self.data[y][x]
    }

    fn find_neighbours(&self, octo: &Octopus) -> Vec<(usize, usize)> {
        let x = octo.x;
        let y = octo.y;

        [
            (x > 0 && y > 0, (x - 1, y - 1)),              // top left
            (y > 0, (x, y - 1)),                           // top
            (x < self.width - 1 && y > 0, (x + 1, y - 1)), // top right
            (x < self.width - 1, (x + 1, y)),              // right
            (x < self.width - 1 && y < self.height - 1, (x + 1, y + 1)), // bottom right
            (y < self.height - 1, (x, y + 1)),             // bottom
            (x > 0 && y < self.height - 1, (x - 1, y + 1)), // bottom left
            (x > 0, (x - 1, y)),                           // left
        ]
        .iter()
        .filter_map(|(p, (x, y))| if *p { Some((*x, *y)) } else { None })
        .collect_vec()
    }
}

impl std::string::ToString for Cavern {
    fn to_string(&self) -> String {
        (0..self.height)
            .map(|y| (0..self.width).map(|x| self.data[y][x].energy).join(""))
            .join("\n")
            + "\n"
    }
}

impl fmt::Debug for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.to_string())
        // f.write_str("DEBUG FOR CAVERN")
    }
}

fn parse_input(input: &str) -> Cavern {
    let data = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, ch)| Octopus {
                    x,
                    y,
                    energy: ch.to_digit(10).unwrap(),
                })
                .collect_vec()
        })
        .collect_vec();

    Cavern {
        width: data[0].len(),
        height: data.len(),
        data,
    }
}

fn simulate_step(map: &mut Cavern) -> usize {
    let mut flashing: VecDeque<(usize, usize)> = VecDeque::new();
    let mut flashes = 0;

    // Increase everyones energy
    for (y, x) in iproduct!((0..map.height), (0..map.width)) {
        let octo = map.get_mut(x, y);

        if octo.glow() {
            flashes += 1;
            flashing.push_back((x, y));
        }
    }

    // Flash
    while let Some((x, y)) = flashing.pop_back() {
        let octo = map.get(x, y);

        for &mut (nx, ny) in &mut map.find_neighbours(octo) {
            let neighbour = map.get_mut(nx, ny);

            if neighbour.energy == 0 {
                continue;
            }

            if neighbour.glow() {
                flashing.push_back((nx, ny));
                flashes += 1;
            }
        }
    }

    flashes
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parts() {
        let input = indoc! {"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "};

        assert_eq!(part_one(input), 1656);
    }

    #[test]
    fn test_simulate_step_small_example() {
        let mut map = parse_input(indoc! {"
            11111
            19991
            19191
            19991
            11111
        "});

        println!("{}", map.to_string());

        simulate_step(&mut map);

        println!("{}", map.to_string());

        assert_eq!(
            map.to_string(),
            indoc! {"
                34543
                40004
                50005
                40004
                34543
            "}
        );

        simulate_step(&mut map);

        assert_eq!(
            map.to_string(),
            indoc! {"
                45654
                51115
                61116
                51115
                45654
            "}
        );
    }

    #[test]
    fn test_simulate_step_large_example() {
        let mut map = parse_input(indoc! {"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "});

        simulate_step(&mut map);

        assert_eq!(
            map.to_string(),
            indoc! {"
                6594254334
                3856965822
                6375667284
                7252447257
                7468496589
                5278635756
                3287952832
                7993992245
                5957959665
                6394862637
            "}
        );

        simulate_step(&mut map);

        assert_eq!(
            map.to_string(),
            indoc! {"
                8807476555
                5089087054
                8597889608
                8485769600
                8700908800
                6600088989
                6800005943
                0000007456
                9000000876
                8700006848
            "}
        );

        simulate_step(&mut map);

        assert_eq!(
            map.to_string(),
            indoc! {"
                0050900866
                8500800575
                9900000039
                9700000041
                9935080063
                7712300000
                7911250009
                2211130000
                0421125000
                0021119000
            "}
        );

        simulate_step(&mut map);

        assert_eq!(
            map.to_string(),
            indoc! {"
                2263031977
                0923031697
                0032221150
                0041111163
                0076191174
                0053411122
                0042361120
                5532241122
                1532247211
                1132230211
            "}
        );
    }

    #[test]
    fn test_find_neighbours() {
        let map = parse_input(indoc! {"
            11111
            19991
            19191
            19991
            11111
        "});

        assert_eq!(
            map.find_neighbours(map.get(0, 0)),
            vec![(1, 0), (1, 1), (0, 1)]
        );

        assert_eq!(
            map.find_neighbours(map.get(1, 1)),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (2, 1),
                (2, 2),
                (1, 2),
                (0, 2),
                (0, 1)
            ]
        );

        assert_eq!(
            map.find_neighbours(map.get(4, 4)),
            vec![(3, 3), (4, 3), (3, 4)]
        );
    }

    #[test]
    fn test_flash_together() {
        let mut map = parse_input(indoc! {"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "});

        (0..=192).for_each(|_| {
            simulate_step(&mut map);
        });

        assert_eq!(
            map.to_string(),
            indoc! {"
                5877777777
                8877777777
                7777777777
                7777777777
                7777777777
                7777777777
                7777777777
                7777777777
                7777777777
                7777777777
            "}
        );

        simulate_step(&mut map);

        assert_eq!(
            map.to_string(),
            indoc! {"
                6988888888
                9988888888
                8888888888
                8888888888
                8888888888
                8888888888
                8888888888
                8888888888
                8888888888
                8888888888
            "}
        );

        let flashes = simulate_step(&mut map);

        assert_eq!(
            map.to_string(),
            indoc! {"
                0000000000
                0000000000
                0000000000
                0000000000
                0000000000
                0000000000
                0000000000
                0000000000
                0000000000
                0000000000
            "}
        );

        assert_eq!(flashes, 100);
    }
}
