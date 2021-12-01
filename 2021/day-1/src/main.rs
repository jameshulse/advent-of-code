// use itertools::Itertools;
use std::fs;

fn main() {
    // let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let lines = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let depths: Vec<u32> = lines
        .split("\n")
        .map(|val| val.parse::<u32>().expect("Invalid value in input file!"))
        .collect();

    // let increases = depths.into_iter().next_tuple(2);

    let mut increases = 0;

    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            increases += 1;
        }
    }

    println!("{:?}", increases)
}
