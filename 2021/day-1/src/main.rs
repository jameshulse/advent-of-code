use std::fs;

fn main() {
    let text = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let depths: Vec<u32> = text
        .lines()
        .map(|val| val.parse::<u32>().expect("Invalid value in input file!"))
        .collect();

    let step_one_increases = step_one(&depths);
    let step_two_increases = step_two(&depths);

    println!("Step One: {:?}", step_one_increases);
    println!("Step two: {:?}", step_two_increases);
}

fn step_one(depths: &Vec<u32>) -> usize {
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

fn step_two(depths: &Vec<u32>) -> usize {
    depths
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_one() {
        let depths: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(step_one(&depths), 7)
    }

    #[test]
    fn test_step_two() {
        let depths: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(step_two(&depths), 5)
    }
}
