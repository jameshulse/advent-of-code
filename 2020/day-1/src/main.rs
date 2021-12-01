use itertools::Itertools;
use std::fs;

fn main() {
    println!("Hello, world!");

    let lines = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let values: Vec<u32> = lines
        .split("\n")
        .map(|val| val.parse::<u32>().expect("Invalid value in input file!"))
        .collect();

    let result = find_matching_combinations(values, 3);

    match result {
        Ok(val) => {
            println!("Found the combination: {:?}", val);
            println!("The result is: {:?}", val.iter().product::<u32>());
        }
        Err(e) => println!("{}", e),
    }
}

// TODO: Could I make 'input' an iterable so we don't have to map the whole file?
fn find_matching_combinations(
    input: Vec<u32>,
    combo_size: usize,
) -> Result<Vec<u32>, &'static str> {
    let mut combos = input.into_iter().combinations(combo_size);

    loop {
        match combos.next() {
            Some(val) if val.iter().sum::<u32>() == 2020 => break Ok(val),
            Some(_) => continue,
            None => break Err("Couldn't find matching pair"),
        };
    }
}
