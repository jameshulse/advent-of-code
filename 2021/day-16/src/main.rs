use day_16::*;

// use bit_field::{BitField};
// use bitvec::prelude::*;



fn main() {
    let input = include_str!("input");

    dbg!(assert_eq!(part_one(input), 0));
    // dbg!(assert_eq!(part_two(input), 2995));
}

fn part_one(input: &str) -> usize {
    println!("Input: {}", input);

    let mut versions: Vec<usize> = vec![];
    let input = BinaryString::from_hex(input);

    // TODO: Use extract_packets

    versions.iter().sum()
}

// fn parse_header(input: &str) -> (usize, usize) {
//     let header = parse_hex(&input[0..2]);

//     (header.get_bits(5..8), header.get_bits(2..5))
// }

// fn parse_literal(input: &str) -> usize {
//     let literal_start = input.len() * 4 - 6;
    
//     let parsed = parse_hex(input);
//     let literal_bits = BitSlice::<Msb0, _>::from_element(&parsed);
    
//     let mut result = BitVec::new();

//     for group in literal_bits[literal_bits.len()-literal_start..].chunks(5) {
//         let literal = &group[1..];

//         result.extend(literal);

//         if !group[0] {
//             break
//         }
//     }

//     slice_to_num(&result)
// }

// fn slice_to_num(slice: &BitVec) -> usize {
//     slice.iter().rev().enumerate().fold(0, |r, (i, b)| {
//         r + (2_usize.pow(i as u32)) * (if *b { 1 } else { 0 })
//     })
// }

// fn parse_hex(input: &str) -> usize {
//     usize::from_str_radix(input, 16).ok().unwrap()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("D2FE28"), 6);
        assert_eq!(part_one("8A004A801A8002F478"), 16);
        assert_eq!(part_one("620080001611562C8802118E34"), 12);
        assert_eq!(part_one("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part_one("A0016C880162017C3686B18A3D4780"), 31);
    }
}