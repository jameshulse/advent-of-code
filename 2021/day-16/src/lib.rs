use core::panic;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryString(String);

impl BinaryString {
    pub fn from_hex(input: &str) -> BinaryString {
        let result = input.chars().map(|ch| format!("{:04b}", ch.to_digit(16).unwrap())).collect_vec().join("");

        BinaryString(result)
    }

    // pub fn len(&self) -> usize {
    //     let BinaryString(bits) = self;

    //     bits.len()
    // }

    pub fn to_num(&self) -> usize {
        let BinaryString(bits) = self;

        bits.chars().rev().enumerate().fold(0, |r, (i, b)| {
            r + (2_usize.pow(i as u32)) * (if b == '1' { 1 } else { 0 })
        })
    }

    pub fn nth(&self, index: usize) -> char {
        let BinaryString(bits) = self;

        bits.chars().nth(index).unwrap()
    }
}

// #[derive(Debug, PartialEq)]
// pub struct PacketHeader { version: usize, type_id: usize }

// impl PacketHeader {
//     pub fn get_version(&self) -> usize { self.version }
//     pub fn get_type_id(&self) -> usize { self.type_id }
// }

// pub fn parse_header(input: &BinaryString) -> PacketHeader {
//     let BinaryString(bits) = input;

//     PacketHeader {
//         version: BinaryString(bits[0..3].to_string()).to_num(),
//         type_id: BinaryString(bits[3..6].to_string()).to_num(),
//     }
// }

pub fn extract_literal(input: &BinaryString) -> usize {
    let BinaryString(bits) = input;
    let mut result = String::new();

    for mut group in &bits.chars().skip(6).chunks(5) {
        let flag = group.next().unwrap();

        result += &group.join("");
        
        if flag == '0' {
            break;
        }
    }

    BinaryString(result).to_num()
}

pub fn get_sub_packets(input: &BinaryString) -> BinaryString {
    let packet_length = match input.nth(7) {
        '0' => 15,
        '1' => 11,
        _ => panic!("Invalid length bit"),
    };

    let BinaryString(bits) = input;

    BinaryString(bits[7..7+packet_length].to_string())
}

pub fn extract_packets(input: &BinaryString) -> Vec<&BinaryString> {
    let mut result: Vec<&BinaryString> = vec![];
    let BinaryString(bits) = input;
    let mut bit_iter = bits.chars();

    result.push(input);

    loop {
        let version = BinaryString((bit_iter.take(3).join("")).to_string()).to_num();
        let type_id = BinaryString((bit_iter.take(3).join("")).to_string()).to_num();

        // let header = parse_header(&packet);

        // if header.get_type_id() == 4 {
        //     result.push(&packet.clone());
        //     break;
        // }

        // let sub_packet_length = get_sub_packets(&packet);


    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_binary() {
        assert_eq!(BinaryString::from_hex("A"), BinaryString("1010".to_string()));
        assert_eq!(BinaryString::from_hex("D2FE28"), BinaryString("110100101111111000101000".to_string()));
    }

    #[test]
    fn test_parse_header() {
        assert_eq!(parse_header(&BinaryString::from_hex("D2FE28")), PacketHeader { version: 6, type_id: 4 });
    }

    #[test]
    fn test_extract_literal() {
        assert_eq!(extract_literal(&BinaryString::from_hex("D2FE28")), 2021);
    }
}