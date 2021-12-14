use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input").to_string();

    dbg!(assert_eq!(part_one(&input), 370_407));
    dbg!(assert_eq!(part_two(&input), 3_249_889_609));
}

#[derive(PartialEq, Debug)]
enum LineState {
    Ok,
    Corrupt(char, char), // Expected, Found
    Incomplete(Vec<char>),
}

fn check_line(line: &str) -> LineState {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut stack = Vec::new();

    for ch in line.chars() {
        if pairs.contains_key(&ch) {
            stack.push(ch);
        } else {
            let last = stack.pop().unwrap();
            let expected = *pairs.get(&last).unwrap();

            if ch != expected {
                return LineState::Corrupt(expected, ch);
            }
        }
    }

    if !stack.is_empty() {
        return LineState::Incomplete(stack.into_iter().collect_vec());
    }

    LineState::Ok
}

fn part_one(input: &str) -> usize {
    let points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    input
        .lines()
        .map(check_line)
        .filter_map(|s| match s {
            LineState::Corrupt(_, found) => Some(points[&found]),
            _ => None,
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let scores = input
        .lines()
        .map(check_line)
        .filter_map(|s| match s {
            LineState::Incomplete(remaining) => Some(remaining),
            _ => None,
        })
        .map(|symbols| symbols.iter().map(|s| pairs[s]).rev().collect_vec())
        .map(|s| score_string(&s))
        .sorted()
        .collect_vec();

    dbg!(&scores);

    scores[((scores.len() as f32) / 2f32).floor() as usize]
}

fn score_string(input: &[char]) -> usize {
    let scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    input.iter().fold(0, |acc, ch| acc * 5 + scores[ch])
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parts() {
        let input = indoc! {"
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        "};

        assert_eq!(part_one(input), 26397);
        assert_eq!(part_two(input), 288957);
    }

    #[test]
    fn test_score_string() {
        assert_eq!(score_string(&"}}]])})]".chars().collect_vec()), 288957);
        assert_eq!(score_string(&")}>]})".chars().collect_vec()), 5566);
        assert_eq!(score_string(&"}}>}>))))".chars().collect_vec()), 1480781);
        assert_eq!(score_string(&"]]}}]}]}>".chars().collect_vec()), 995444);
        assert_eq!(score_string(&"])}>".chars().collect_vec()), 294);
    }

    #[test]
    fn test_check_lines_ok() {
        let inputs = indoc! {"
            ()
            []
            ([])
            {()()()}
            <([{}])>
            [<>({}){}[([])<>]]
            (((((((((())))))))))
        "};

        let mut results = inputs.lines().map(check_line);

        assert!(results.all(|l| l == LineState::Ok));
    }

    #[test]
    fn test_check_lines_corrupted() {
        assert_eq!(
            check_line("{([(<{}[<>[]}>{[]{[(<()>"),
            LineState::Corrupt(']', '}')
        );
        assert_eq!(
            check_line("[[<[([]))<([[{}[[()]]]"),
            LineState::Corrupt(']', ')')
        );
        assert_eq!(
            check_line("[{[{({}]{}}([{[{{{}}([]"),
            LineState::Corrupt(')', ']')
        );
        assert_eq!(
            check_line("[<(<(<(<{}))><([]([]()"),
            LineState::Corrupt('>', ')')
        );
        assert_eq!(
            check_line("<{([([[(<>()){}]>(<<{{"),
            LineState::Corrupt(']', '>')
        );
    }
}
