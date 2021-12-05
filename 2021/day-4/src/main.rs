mod game;

use itertools::Itertools;

fn main() {
    let lines = include_str!("input");

    dbg!(assert_eq!(part_one(lines), 51034));
    dbg!(assert_eq!(part_two(lines), 5434));
}

fn build_boards(input: &str) -> Vec<game::Board> {
    let lines: Vec<&str> = input.lines().collect_vec();
    let mut boards: Vec<game::Board> = vec![];

    for line in lines.iter().skip(1) {
        if line.is_empty() {
            boards.push(game::Board {
                rows: vec![],
                won: false,
            });
            continue;
        }

        let board_line = line
            .split_whitespace()
            .map(|v| game::Place {
                value: v.to_owned(),
                drawn: false,
            })
            .collect_vec();

        boards.last_mut().unwrap().rows.push(board_line);
    }

    boards
}

fn part_one(input: &str) -> usize {
    let draws = input.lines().collect_vec()[0].split(',');
    let mut boards = build_boards(input);

    // Do draws
    for draw in draws {
        for board in boards.iter_mut() {
            board.mark_draw(draw);

            if board.check_win() {
                return draw.parse::<usize>().unwrap() * board.calculate_total();
            }
        }
    }

    0
}

fn part_two(input: &str) -> usize {
    let draws = input.lines().collect_vec()[0].split(',');
    let mut boards = build_boards(input);
    let mut last_draw: usize = 0;
    let mut winning_total: usize = 0;

    // Do draws
    for draw in draws {
        for board in boards.iter_mut() {
            board.mark_draw(draw);

            if !board.won && board.check_win() {
                last_draw = draw.parse().unwrap();
                winning_total = board.calculate_total();
            }
        }
    }

    last_draw * winning_total
}

#[cfg(test)]
use indoc::indoc;

#[test]
fn test_parts() {
    let input = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7"
    };

    assert_eq!(part_one(input), 4512);
    assert_eq!(part_two(input), 1924);
}

#[test]
fn test_mark_draw() {
    let input = indoc! {"
        7

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19
    "};

    let mut board = build_boards(input);

    board[0].mark_draw("7");

    assert!(board[0].rows[2][4].drawn);
}

#[test]
fn test_win() {
    let mut winning_row = game::Board {
        rows: vec![vec![
            game::Place {
                value: "14".to_string(),
                drawn: true,
            },
            game::Place {
                value: "21".to_string(),
                drawn: true,
            },
            game::Place {
                value: "17".to_string(),
                drawn: true,
            },
            game::Place {
                value: "24".to_string(),
                drawn: true,
            },
            game::Place {
                value: "4".to_string(),
                drawn: true,
            },
        ]],
        won: false,
    };

    let mut winning_col = game::Board {
        rows: vec![
            vec![game::Place {
                value: "14".to_string(),
                drawn: true,
            }],
            vec![game::Place {
                value: "21".to_string(),
                drawn: true,
            }],
            vec![game::Place {
                value: "17".to_string(),
                drawn: true,
            }],
            vec![game::Place {
                value: "24".to_string(),
                drawn: true,
            }],
            vec![game::Place {
                value: "2".to_string(),
                drawn: true,
            }],
        ],
        won: false,
    };

    let mut losing_board = game::Board {
        rows: vec![
            vec![game::Place {
                value: "14".to_string(),
                drawn: false,
            }],
            vec![game::Place {
                value: "21".to_string(),
                drawn: false,
            }],
            vec![game::Place {
                value: "17".to_string(),
                drawn: false,
            }],
            vec![game::Place {
                value: "24".to_string(),
                drawn: false,
            }],
            vec![game::Place {
                value: "2".to_string(),
                drawn: false,
            }],
        ],
        won: false,
    };

    assert!(winning_row.check_win());
    assert!(winning_col.check_win());
    assert!(!losing_board.check_win());
}
