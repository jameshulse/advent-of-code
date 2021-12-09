use itertools::Itertools;
use std::collections::HashMap;

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
// Remove "c" from 1 digits -> "f" is solved as "b"

// Solution:
// A -> d
// B -> e
// C -> a
// D -> f
// E -> g
// F -> b
// G -> c
fn solve_mappings(entry: &Entry) -> HashMap<String, String> {
    let one = find_by_len(&entry.input, 2);
    let four = find_by_len(&entry.input, 4);
    let seven = find_by_len(&entry.input, 3);

    // Solve for "a"
    let solved_a = seven
        .chars()
        .filter(|&ch| !one.contains(ch))
        .collect::<String>();

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

    let mut solved_b = String::new();
    let mut solved_e = String::new();

    for ch in single_chars {
        if four.contains(ch) {
            // If the character matches a character in digit 4 then it is the "b"
            solved_b = ch.to_string();
        } else {
            // If it doesn't match a character in digit 4 then it must be the "e"
            solved_e = ch.to_string();
        }
    }

    // Solve for "d"
    let solved_d = four
        .chars()
        .filter(|&ch| !one.contains(ch) && !solved_b.contains(ch))
        .collect::<String>();

    // Solve for "g" - Take 5 len digits, remove 'a', 'd' and find triple occurence
    let triple_chars = len_five
        .iter()
        .flat_map(|s| {
            s.replace(&solved_a, "")
                .replace(&solved_d, "")
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

    let solved_g = triple_chars[0].to_string();

    // mappings.insert(triple_chars[0].to_string(), "g".to_string());

    // Solve for "c" -> Take 6 digit, remove 'd' and find letter occurring twice
    let len_six = entry.input.iter().filter(|e| e.len() == 6).collect_vec();

    let double_chars = len_six
        .iter()
        .flat_map(|s| {
            s.replace(&solved_e, "")
                .replace(&solved_d, "")
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

    let solved_c = double_chars[0].to_string();

    // mappings.insert(double_chars[0].to_string(), "c".to_string());

    // Solve for "f" -> Remove 'c' from 1
    let solved_f = one.replace(&solved_c, "");

    let mut mappings: HashMap<String, String> = HashMap::new();

    mappings.insert(solved_a.to_string(), "a".to_string());
    mappings.insert(solved_b.to_string(), "b".to_string());
    mappings.insert(solved_c.to_string(), "c".to_string());
    mappings.insert(solved_d.to_string(), "d".to_string());
    mappings.insert(solved_e.to_string(), "e".to_string());
    mappings.insert(solved_f.to_string(), "f".to_string());
    mappings.insert(solved_g.to_string(), "g".to_string());

    mappings
}

// bcdef -[map]> fgabd -[sort]> abdfg -[match]> "5"
fn resolve_output(mappings: &HashMap<String, String>, output: &Vec<String>) -> usize {
    let mut out_str: String = "".to_string();

    for unmapped in output {
        let mapped = unmapped
            .chars()
            .map(|c| mappings.get(&c.to_string()).unwrap())
            .sorted()
            .join("");

        let result: Option<&str> = match mapped.len() {
            2 => Some("1"),
            4 => Some("4"),
            3 => Some("7"),
            7 => Some("8"),
            _ => match mapped.as_str() {
                "abcefg" => Some("0"),
                "acdeg" => Some("2"),
                "acdfg" => Some("3"),
                "abdfg" => Some("5"),
                "abdefg" => Some("6"),
                "abcdfg" => Some("9"),
                _ => None,
            },
        };

        out_str += result.unwrap();
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

    assert_eq!(mappings.get("d").unwrap(), &"a".to_string());
    assert_eq!(mappings.get("e").unwrap(), &"b".to_string());
    assert_eq!(mappings.get("a").unwrap(), &"c".to_string());
    assert_eq!(mappings.get("f").unwrap(), &"d".to_string());
    assert_eq!(mappings.get("g").unwrap(), &"e".to_string());
    assert_eq!(mappings.get("b").unwrap(), &"f".to_string());
    assert_eq!(mappings.get("c").unwrap(), &"g".to_string());
}

#[test]
fn test_resolve_output() {
    let entries = parse_input(
        indoc! {"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"},
    );
    let mut mappings = HashMap::new();

    mappings.insert("d".to_string(), "a".to_string());
    mappings.insert("e".to_string(), "b".to_string());
    mappings.insert("a".to_string(), "c".to_string());
    mappings.insert("f".to_string(), "d".to_string());
    mappings.insert("g".to_string(), "e".to_string());
    mappings.insert("b".to_string(), "f".to_string());
    mappings.insert("c".to_string(), "g".to_string());

    assert_eq!(resolve_output(&mappings, &entries[0].output), 5353);
}

#[test]
fn test_example_1() {
    let entries = parse_input(
        indoc! {"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"},
    );
    let mappings = solve_mappings(&entries[0]);

    assert_eq!(resolve_output(&mappings, &entries[0].output), 8394);
}
