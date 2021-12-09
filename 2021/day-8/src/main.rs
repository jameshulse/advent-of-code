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
    let mut output_sum = 0;

    for entry in entries {
        let mappings = solve_mappings(entry);

        output_sum += resolve_output(&mappings, &entry.output);
    }

    output_sum
}

struct Entry {
    input: Vec<String>,
    output: Vec<String>,
}

// TODO: impl for Iter so I can call e.g entry.input.find_by_len(2)?
fn find_by_len(target: &[String], len: usize) -> &String {
    target.iter().find(|e| e.len() == len).unwrap()
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
// "eafb" (4) remove unknown "c" and "f" from 1 and mapped "b" to leave solution for "d"
// "eafb".replace("ab", "") -> "ef", "ef".replace("e", "") -> "d" is solved as "f"

// Solve for G
// (5 len chars) -> remove mapped "a" and "d" -> letter occuring 3 times will be "g"
// aabbcccdddefffg -> remove mapped "a" + "d" -> aabbccceg, triple letter is "c" so -> "g" is solved as "c"

// Solve for C
// (6 len chars) -> remove mapped "d" + "e" -> letter occurring 2 times will be "c"
// cefabd cdfgeb cagedb -> (remove 'f' and 'g') -> ceabd cdeb caedb -> "c" solved as "a"

// Solve for F
// Remove "c" from 1 digits
fn solve_mappings(entry: &Entry) -> HashMap<&str, String> {
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

    // Solve for "d"
    let solved_d = four
        .replace(one, "")
        .replace(mappings.get("b").unwrap(), "");

    mappings.insert("d", solved_d);

    // Solve for "g" - Take 5 len digits, remove 'a', 'd' and find triple occurence
    let triple_chars = len_five
        .iter()
        .flat_map(|s| {
            s.replace(mappings.get("a").unwrap(), "")
                .replace(mappings.get("d").unwrap(), "")
                .chars()
                .collect_vec()
        })
        .sorted()
        .group_by(|&c| c)
        .into_iter()
        .filter_map(|(ch, g)| match g.collect_vec().len() {
            3 => Some(ch),
            _ => None,
        })
        .collect_vec();

    mappings.insert("g", triple_chars[0].to_string());

    // Solve for "c" -> Take 6 digit, remove 'd' and find letter occurring twice
    let len_six = entry.input.iter().filter(|e| e.len() == 6).collect_vec();

    let double_chars = len_six
        .iter()
        .flat_map(|s| {
            s.replace(mappings.get("e").unwrap(), "")
                .replace(mappings.get("d").unwrap(), "")
                .chars()
                .collect_vec()
        })
        .sorted()
        .group_by(|&c| c)
        .into_iter()
        .filter_map(|(ch, g)| match g.collect_vec().len() {
            2 => Some(ch),
            _ => None,
        })
        .collect_vec();

    mappings.insert("c", double_chars[0].to_string());

    // Solve for "f" -> Remove 'c' from 1
    let solved_f = one.replace(mappings.get("c").unwrap(), "");

    mappings.insert("f", solved_f);

    mappings
}

const ZERO: String = "abcefg".to_string();
const TWO: String = "acdeg".to_string();
const THREE: String = "acdfg".to_string();
const FIVE: String = "abdfg".to_string();
const SIX: String = "abdefg".to_string();
const NINE: String = "abcdfg".to_string();

fn resolve_output(mappings: &HashMap<&str, String>, output: &Vec<String>) -> usize {
    let mut out_str: String = "".to_string();

    for unmapped in output {
        let result = match unmapped.len() {
            2 => '1',
            4 => '4',
            3 => '7',
            7 => '8',
            _ => match unmapped
                .chars()
                .map(|c| mappings.get(&c.to_string()).unwrap())
                .join("")
            {
                ZERO => '0',
                TWO => '2',
                THREE => '2',
                _ => panic!("Mapping not found!"),
            },
        };

        out_str.push(result);
    }

    out_str.parse().unwrap()
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

    let mappings = solve_mappings(&entries[0]);

    assert_eq!(mappings.get("a").unwrap(), &"d".to_string());
    assert_eq!(mappings.get("b").unwrap(), &"e".to_string());
    assert_eq!(mappings.get("c").unwrap(), &"a".to_string());
    assert_eq!(mappings.get("d").unwrap(), &"f".to_string());
    assert_eq!(mappings.get("e").unwrap(), &"g".to_string());
    assert_eq!(mappings.get("f").unwrap(), &"b".to_string());
    assert_eq!(mappings.get("g").unwrap(), &"c".to_string());
}

#[test]
fn test_resolve_output() {
    let entries = parse_input(
        indoc! {"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"},
    );
    let mut mappings = HashMap::new();

    mappings.insert("a", "d".to_string());
    mappings.insert("b", "e".to_string());
    mappings.insert("c", "a".to_string());
    mappings.insert("d", "f".to_string());
    mappings.insert("e", "g".to_string());
    mappings.insert("f", "b".to_string());
    mappings.insert("g", "c".to_string());

    assert_eq!(resolve_output(&mappings, &entries[0].output), 5355);
}

// #[test]
// fn test_digit_to_binary() {
//     assert_eq!(digit_to_binary(&"cf".to_owned()), 0b0010010);
// }
