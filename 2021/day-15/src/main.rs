use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    dbg!(assert_eq!(part_one(input), 0));
    // dbg!(assert_eq!(part_two(input), 0));
}

type Cavern = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Cavern {
    input
        .lines()
        .map(|l| l.bytes().map(|ch| ch - b'0').collect_vec())
        .collect::<Cavern>()
}

fn part_one(input: &str) -> usize {
    0
}

fn find_paths() -> () {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

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
        // assert_eq!(part_two(input), 0);
    }
}
