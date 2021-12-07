use itertools::Itertools;

fn main() {
    let input = parse_input(include_str!("input"));

    dbg!(assert_eq!(part_one(&input), 328262));
    dbg!(assert_eq!(part_two(&input), 90040997));
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split(',').map(|p| p.parse().unwrap()).collect_vec()
}

fn part_one(positions: &[i32]) -> i32 {
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();

    (min_position..=max_position)
        .map(|t| positions.iter().map(|p| (*p - t).abs()).sum::<i32>())
        .min()
        .unwrap()
}

fn part_two(positions: &[i32]) -> i32 {
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();

    (min_position..=max_position)
        .map(|t| {
            positions
                .iter()
                .map(|p| {
                    let n = (*p - t).abs();

                    (n.pow(2) + n) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
}

#[test]
fn test_parts() {
    let input = parse_input("16,1,2,0,4,2,7,1,2,14");

    assert_eq!(part_one(&input), 37);
    assert_eq!(part_two(&input), 168);
}
