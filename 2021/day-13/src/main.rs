use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    dbg!(assert_eq!(part_one(input), 802));
    dbg!(part_two(input));
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

type Grid = HashSet<(usize,usize)>;

fn parse_input(input: &str) -> (Grid, Vec<Fold>) {
    let mut lines = input.lines();
    let grid: Grid = lines
        .by_ref()
        .take_while(|&l| !l.is_empty())
        .map(|l| l.split(',').collect_tuple().unwrap())
        .map(|(xstr, ystr)| (xstr.parse().unwrap(),ystr.parse().unwrap()))
        .collect();

    let folds: Vec<_> = lines
        .map(|l| l.split('=').collect_tuple().unwrap())
        .map(|(l, r)| if l.ends_with("x") { Fold::X(r.parse().unwrap()) } else { Fold::Y(r.parse().unwrap()) })
        .collect();

    (grid, folds)
}

fn part_one(input: &str) -> usize {
    let (mut grid, folds) = parse_input(input);

    fold(&mut grid, &folds[0]);

    grid.len()
}

fn part_two(input: &str) {
    let (mut grid, folds) = parse_input(input);

    for f in folds {
        fold(&mut grid, &f);
    }

    display_grid(&grid);
}

fn fold(grid: &mut Grid, fold: &Fold) {
    for (x, y) in grid.clone() {
        let skip_fold = match fold {
            Fold::X(col) => &x < col,
            Fold::Y(row) => &y < row,
        };

        if skip_fold {
            continue;
        }

        let to = match fold {
            Fold::X(col) => (x - (x - col) * 2, y),
            Fold::Y(row) => (x, y - (y - row) * 2),
        };

        grid.remove(&(x, y));
        grid.insert(to);
    }
}

fn display_grid(grid: &Grid) {
    let width = grid.iter().map(|&(x, _)| x).max().unwrap();
    let height = grid.iter().map(|&(_, y)| y).max().unwrap();

    for y in 0..=height {
        for x in 0..=width {
            if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0
            
            fold along y=7
            fold along x=5
        "};

        assert_eq!(part_one(input), 17);
    }

    #[test]
    fn test_part_two() {
        let input = indoc! {"
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0
            
            fold along y=7
            fold along x=5
        "};

        part_two(input);
    }
}