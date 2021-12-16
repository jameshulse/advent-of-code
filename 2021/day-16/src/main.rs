use itertools::Itertools;
use bit_field::BitField;

fn main() {
    let input = include_str!("input");

    dbg!(assert_eq!(part_one(input), 0));
    // dbg!(assert_eq!(part_two(input), 2995));
}

fn part_one(input: &str) -> usize {
    let (version, type_id) = parse_header(input);

    dbg!(input);

    // Type 4 = literal value

    version as usize
}

fn parse_header(input: &str) -> (u8, u8) {
    let header = get_hex(&input[0..2]);

    (header.get_bits(5..8), header.get_bits(2..5))
}

fn get_hex(input: &str) -> u8 {
    u8::from_str_radix(input, 16).ok().unwrap()
}

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