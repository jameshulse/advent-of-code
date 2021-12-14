use itertools::Itertools;

fn main() {
    let input = include_str!("input").to_string();

    dbg!(assert_eq!(part_one(&input, 80), 362666));
    dbg!(assert_eq!(part_two(&input, 256), 1640526601595));
}

// Naive solution
fn part_one(input: &str, simulation_days: usize) -> usize {
    let mut all_fish: Vec<u8> = input.split(',').map(|d| d.parse().unwrap()).collect_vec();

    println!("Initial state: {}", input);

    for _ in 1..=simulation_days {
        let mut new_fish: Vec<u8> = vec![];

        for fish in &mut all_fish {
            if *fish == 0 {
                new_fish.push(8);

                *fish = 6;
            } else {
                *fish -= 1;
            }
        }

        all_fish.append(&mut new_fish);
    }

    all_fish.len()
}

fn part_two(input: &str, simulation_days: usize) -> usize {
    let mut days = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    // Set state
    for val in input.split(',').map(|d| d.parse::<usize>().unwrap()) {
        days[val] += 1;
    }

    println!("{:?}", days);

    for _ in 1..=simulation_days {
        let spawning = days[0];

        for timer in 0..8 {
            days[timer] = days[timer + 1]; // Reduce timers
        }

        days[6] += spawning;
        days[8] = spawning;
    }

    days.iter().sum()
}

#[cfg(test)]
use indoc::indoc;

#[test]
fn test_parts() {
    let input = indoc! {"3,4,3,1,2"}.to_string();

    assert_eq!(part_one(&input, 18), 26);
    assert_eq!(part_two(&input, 18), 26);
    assert_eq!(part_two(&input, 256), 26984457539);
}
