use itertools::Itertools;
use std::{borrow::Borrow, collections::HashMap};

fn main() {
    let entries = &parse_input(include_str!("input"));

    dbg!(assert_eq!(part_one(entries), 245));
    dbg!(assert_eq!(part_two(entries), 0));
}

fn part_one(entries: &Vec<Entry>) -> usize {
    let unique_sizes = [2, 3, 4, 7];

    entries
        .iter()
        .map(|e| {
            e.output
                .iter()
                .filter(|s| unique_sizes.contains(&s.len()))
                .count()
        })
        .sum()
}

fn part_two(entries: &Vec<Entry>) -> usize {
    0
}

struct Entry {
    input: Vec<String>,
    output: Vec<String>,
}

// TODO: impl for Iter so I can call e.g entry.input.find_by_len(2)
fn find_by_len(target: &[String], len: usize) -> &String {
    target.iter().find(|e| e.len() == len).unwrap()
}

// Example used for explained solutions below:
// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab

// Solve for A
// "ab".len == 2 ->> "1"
// "dab".len == 3 ->> "7"
// "dab".replace("ab") -> "d" ->> "a" is "d"

// Solve for B and E
// (5 len chars) cdfbe gcdfa fbcad -> aabbcccdddefffg -> 1 instance of 'e', 'g'
// "eafb" (4) contains 'e', therefore "b" is solved as "e"
// "eafb" (4) doesn't contain 'g', therefore "e" is solved as "g"

// Solve for D
// (4) remove "c" and "f" from 1 and "b" to leave "f"
// "eafb".replace("ab", "") -> "ef", "ef".replace("e", "") -> "b" is solved as "f"
fn solve_entry(entry: &Entry) -> usize {
    let mut mappings: HashMap<&str, String> = HashMap::new();

    let one = find_by_len(&entry.input, 2);
    let four = find_by_len(&entry.input, 4);
    let seven = find_by_len(&entry.input, 3);
    let eight = find_by_len(&entry.input, 7);

    // Solve for "a"
    mappings.insert("a", seven.replace(one, ""));

    // Solve for "b" and "e"
    // - Find the two characters which only occur once (representing 'b' and 'e' in digits 2 and 5)
    let len_five = entry.input.iter().filter(|e| e.len() == 5).collect_vec();

    let single_chars = len_five
        .iter()
        .flat_map(|s| s.chars())
        .sorted()
        .group_by(|&c| c)
        .into_iter()
        .filter_map(|(ch, g)| match g.collect_vec().len() {
            1 => Some(ch),
            _ => None,
        })
        .collect_vec();

    for ch in single_chars {
        if four.contains(ch) {
            // If the character matches a character in digit 4 then it is the "b"
            mappings.insert("b", ch.to_string());
        } else {
            // If it doesn't match a character in digit 4 then it must be the "e"
            mappings.insert("e", ch.to_string());
        }
    }

    // Solve for "g" - Take 5 len digits, remove 'a', 'd' and find triple occurence
    let triple_chars = len_five
        .iter()
        .flat_map(|s| {
            let temp = &s.replace(mappings.get("a").unwrap(), "");

            temp.chars().collect_vec()
        })
        .sorted()
        .group_by(|&c| c)
        .into_iter()
        .filter_map(|(ch, g)| match g.collect_vec().len() {
            1 => Some(ch),
            _ => None,
        })
        .collect_vec();

    // mappings.insert("c", double_chars[0].to_string());

    // Solve for "d"
    let solved_d = four
        .replace(one, "")
        .replace(mappings.get("b").unwrap(), "");

    mappings.insert("d", solved_d);

    // <DEBUGGING>
    assert_eq!(*&mappings.get("a").unwrap(), &"d".to_string());
    assert_eq!(*&mappings.get("b").unwrap(), &"e".to_string());
    assert_eq!(*&mappings.get("e").unwrap(), &"g".to_string());
    assert_eq!(*&mappings.get("d").unwrap(), &"f".to_string());
    // </DEBUGGING>

    // Solve for ...

    // TODO: calculate result
    0
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .map(|(sig_in, sig_out)| Entry {
            input: sig_in
                .trim()
                .split(' ')
                .map(|s| s.chars().sorted().collect::<String>())
                .collect(),
            output: sig_out
                .trim()
                .split(' ')
                .map(|s| s.chars().sorted().collect::<String>())
                .collect(),
        })
        .collect()
}

#[cfg(test)]
use indoc::indoc;

#[test]
fn test_parse_input() {
    let entries = parse_input(indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "});

    assert_eq!(entries.len(), 10);
    assert_eq!(entries[0].output.len(), 4);
}

#[test]
fn test_parts() {
    let entries = parse_input(indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "});

    assert_eq!(part_one(&entries), 26);
    assert_eq!(part_two(&entries), 61229);
}

#[test]
fn test_solve_entry() {
    let entries = parse_input(
        indoc! {"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"},
    );

    assert_eq!(solve_entry(&entries[0]), 5353);
}

// #[test]
// fn test_digit_to_binary() {
//     assert_eq!(digit_to_binary(&"cf".to_owned()), 0b0010010);
// }
