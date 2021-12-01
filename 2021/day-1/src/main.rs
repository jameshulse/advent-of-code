use itertools::Itertools;
use std::fs;

fn main() {
    // let depths: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let lines = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let depths: Vec<u32> = lines
        .split("\n")
        .map(|val| val.parse::<u32>().expect("Invalid value in input file!"))
        .collect();

    let depth_triplets: Vec<u32> = depths
        .into_iter()
        .tuple_windows::<(_, _, _)>()
        .map(|t| t.0 + t.1 + t.2)
        .collect();

    let mut increases = 0;

    for i in 1..depth_triplets.len() {
        if depth_triplets[i] > depth_triplets[i - 1] {
            increases += 1;
        }
    }

    println!("{:?}", increases)
}
