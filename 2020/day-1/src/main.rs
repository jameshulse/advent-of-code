use itertools::Itertools;
use std::fs;

fn main() {
    println!("Hello, world!");

    let lines = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let mut combos = lines
        .split("\n")
        .map(|val| val.parse::<u32>().expect("Nope!"))
        .combinations(2);

    let result: Result<Vec<u32>, &str> = loop {
        match combos.next() {
            Some(val) if val[0] + val[1] == 2020 => break Ok(val),
            Some(_) => continue,
            None => break Err("Couldn't find matching pair"),
        };
    };

    match result {
        Ok(val) => {
            println!("Found the combintation: {:?}", val);
            println!("The result is: {:?}", val[0] * val[1]);
        }
        Err(e) => println!("{}", e),
    }
}
