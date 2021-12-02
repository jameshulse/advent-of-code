use itertools::Itertools;
use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let values: Vec<u32> = lines
        .lines()
        .map(|val| val.parse::<u32>().expect("Invalid value in input file!"))
        .collect();

    let result = find_matching_combinations(&values, 3);

    match result {
        Some(val) => {
            println!("Found the combination: {:?}", val);
            println!("The result is: {:?}", val.iter().product::<u32>());
        }
        None => println!("Couldn't find matching combination"),
    }
}

// TODO: Could I make 'input' an iterable so we don't have to map the whole file?
fn find_matching_combinations(input: &Vec<u32>, combo_size: usize) -> Option<Vec<u32>> {
    return input
        .iter()
        .cloned() // TODO: Can I get rid of this? IT will use a lot more memory
        .combinations(combo_size)
        .find(|x| x.iter().cloned().sum::<u32>() == 2020)
        .clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_two_combos() {
        let values = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_matching_combinations(&values, 2);

        assert_eq!(result.unwrap(), vec![1721, 299])
    }

    #[test]
    fn test_find_three_combos() {
        let values = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_matching_combinations(&values, 3);

        assert_eq!(result.unwrap(), vec![979, 366, 675]);
    }
}
