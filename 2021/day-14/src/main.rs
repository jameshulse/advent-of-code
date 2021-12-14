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

    let result = (1..=10).fold(template.to_owned(), |p, _| grow_simple(p, &rules));

    let letters = result.chars().counts_by(|ch| ch);
    let most_common = letters.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let least_common = letters.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();

    most_common.1 - least_common.1
}

fn grow_simple(polymer: String, rules: &Rules) -> String {
    let mut result = polymer
        .chars()
        .tuple_windows()
        .map(|(l, r)| {
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

// Let's get smarter...
fn part_two(input: &str) -> usize {
    let (template, rules) = parse_input(input);

    println!("Template:      {}", template);

    let mut pair_counts = template
        .chars()
        .tuple_windows()
        .counts_by(|(l, r)| format!("{}{}", l, r));

    let mut letter_counts: HashMap<char, usize> = HashMap::new();

    for i in 1..=10 {
        println!("Iteration: {:2}", i);

        for (pair, count) in pair_counts.clone() {
            if count == 0 {
                continue;
            }

            *pair_counts.entry(pair.clone()).or_insert(1) -= count; // Remove current pair

            match rules.get(&pair.clone()) {
                Some(ch) => {
                    let pair_l = pair.chars().nth(0).unwrap();
                    let pair_r = pair.chars().nth(1).unwrap();

                    println!(
                        "Pair: {} x {:3} ({},{}) -> {}",
                        pair, count, pair_l, pair_r, ch
                    );

                    // Update letter counts to aid with result
                    *letter_counts.entry(pair_l).or_insert(0) += count;
                    *letter_counts.entry(pair_r).or_insert(0) += count;
                    *letter_counts.entry(ch).or_insert(0) += count;

                    // Update pair counts
                    *pair_counts.entry(format!("{}{}", pair_l, ch)).or_insert(0) += count;
                    *pair_counts.entry(format!("{}{}", ch, pair_r)).or_insert(0) += count;
                }
                None => continue,
            }
        }
    }

    dbg!(&letter_counts);

    // TODO: Find highest + lowest letter counts

    0
}

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

        let polymer = grow_simple(template.to_string(), &rules);
        assert_eq!(polymer, "NCNBCHB");

        let polymer = grow_simple(polymer, &rules);
        assert_eq!(polymer, "NBCCNBBBCBHCB");

        let polymer = grow_simple(polymer, &rules);
        assert_eq!(polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");

        let polymer = grow_simple(polymer, &rules);
        assert_eq!(polymer, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
    }

    #[test]
    fn test_parts() {
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
        assert_eq!(part_two(input), 2188189693529);
    }
}
