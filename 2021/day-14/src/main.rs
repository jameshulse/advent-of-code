use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    dbg!(assert_eq!(part_one(input), 2068));
    dbg!(assert_eq!(part_two(input), 0));
}

type Rules = HashMap<String, String>;

fn parse_input(input: &str) -> (&str, Rules) {
    let mut lines = input.lines();
    let template = lines.next().unwrap();

    lines.next(); // skip empty line

    let rules: Rules = lines
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(from, to)| (from.to_owned(), to.to_owned()))
        .collect();

    (template, rules)
}

// Naive solution
fn part_one(input: &str) -> usize {
    let (template, rules) = parse_input(input);

    println!("Template:      {}", template);

    let result = (1..=10).fold(template.to_owned(), |p, _| grow(p, &rules));

    let letters = result.chars().counts_by(|ch| ch);
    let most_common = letters.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let least_common = letters.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();

    most_common.1 - least_common.1
}

// Let's get smarter...
fn part_two(input: &str) -> usize {
    let (template, rules) = parse_input(input);

    println!("Template:      {}", template);

    let result = (1..=40).fold(template.to_owned(), |p, i| {
        println!("Iter {}", i);

        grow(p, &rules)
    });

    let letters = result.chars().counts_by(|ch| ch);
    let most_common = letters.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let least_common = letters.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();

    most_common.1 - least_common.1
}

fn grow(polymer: String, rules: &Rules) -> String {
    // dbg!(&polymer.chars().tuple_windows().collect::<Vec<(_, _)>>());

    let mut result = polymer
        .chars()
        .tuple_windows()
        .map(|(l, r)| {
            // TODO: Easier way to combine two chars?
            let pair = format!("{}{}", l, r);

            match rules.get(&pair) {
                Some(ch) => format!("{}{}", l, ch),
                None => pair,
            }
        })
        .join("");

    result.push(polymer.chars().last().unwrap());

    result
}

// fn part_two() {

// }

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_grow() {
        let input = indoc! {"
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
        "};

        let (template, rules) = parse_input(input);

        let polymer = grow(template.to_string(), &rules);

        assert_eq!(polymer, "NCNBCHB");

        let polymer = grow(polymer, &rules);

        assert_eq!(polymer, "NBCCNBBBCBHCB");

        let polymer = grow(polymer, &rules);

        assert_eq!(polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");

        let polymer = grow(polymer, &rules);

        assert_eq!(polymer, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
    }

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
        "};

        assert_eq!(part_one(input), 1588);
    }
}
