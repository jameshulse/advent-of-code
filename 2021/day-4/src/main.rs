use indoc::indoc;
use itertools::Itertools;
use std::fmt;

fn main() {
    let lines = include_str!("input");

    dbg!(part_one(&lines));
    dbg!(part_two(&lines));
}

struct Board {
    rows: Vec<Vec<Place>>,
    won: bool,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("rows", &self.rows)
            .field("won", &self.won)
            .finish()
    }
}

struct Place {
    value: String,
    drawn: bool,
}

impl fmt::Debug for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("value", &self.value)
            .field("drawn", &self.drawn)
            .finish()
    }
}

fn build_boards(input: &str) -> Vec<Board> {
    let lines: Vec<&str> = input.lines().collect_vec();
    let mut boards: Vec<Board> = vec![];

    for i in 1..lines.len() {
        if lines[i] == "" {
            boards.push(Board {
                rows: vec![],
                won: false,
            });
            continue;
        }

        let board_line = lines[i]
            .split_whitespace()
            .map(|v| Place {
                value: v.to_owned(),
                drawn: false,
            })
            .collect_vec();

        boards.last_mut().unwrap().rows.push(board_line);
    }

    boards
}

fn mark_draw(board: &mut Board, draw: &str) {
    for i in 0..board.rows.len() {
        for j in 0..board.rows[i].len() {
            if board.rows[i][j].value == draw {
                board.rows[i][j].drawn = true
            }
        }
    }
}

fn check_win(board: &Board) -> bool {
    // Check rows
    for row in board.rows.iter() {
        if row.iter().all(|v| v.drawn) {
            // dbg!("winning row", row);
            dbg!(board);
            return true;
        }
    }

    // Check columns
    for i in 0..5 {
        let values = board.rows.iter().map(|l| &l[i]).collect_vec();

        if values.iter().all(|v| v.drawn) {
            dbg!("winning col", values);
            return true;
        }
    }

    return false;
}

fn part_one(input: &str) -> usize {
    let draws = input.lines().collect_vec()[0];
    let mut boards = build_boards(input);
    let mut winning_draw: usize = 0;
    let mut winning_total = 0;

    // Do draws
    for draw in draws.split(",") {
        println!("Pulling draw: {:?}", draw);

        for board in boards.iter_mut() {
            mark_draw(board, draw);

            if !board.won && check_win(board) {
                board.won = true;
                winning_draw = draw.parse().unwrap();

                for row in board.rows.iter() {
                    for col in row.iter() {
                        dbg!(col.drawn);

                        if !col.drawn {
                            winning_total += col.value.parse::<usize>().unwrap();
                        }
                    }
                }

                break;
            }
        }

        // No more draws required
        if winning_total != 0 {
            break;
        }
    }

    dbg!(winning_draw, winning_total);

    return winning_draw * winning_total;
}

fn part_two(input: &str) -> usize {
    let draws = input.lines().collect_vec()[0];
    let mut boards = build_boards(input);
    let mut last_draw: usize = 0;
    let mut winning_total: usize = 0;

    // Do draws
    for draw in draws.split(",") {
        for board in boards.iter_mut() {
            mark_draw(board, draw);

            if !board.won && check_win(board) {
                board.won = true;
                last_draw = draw.parse().unwrap();

                winning_total = 0;

                for row in board.rows.iter() {
                    for col in row.iter() {
                        if !col.drawn {
                            winning_total += col.value.parse::<usize>().unwrap();
                        }
                    }
                }

                dbg!(last_draw, winning_total);
            }
        }
    }

    return last_draw * winning_total;
}

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
    mark_draw(&mut board[0], "7");

    assert_eq!(board[0].rows[2][4].drawn, true);
}

// 51034 too high

// #[test]
// fn test_win() {
//     let winning_row = vec![
//         vec!["14", "21", "17", "24", "4"],
//         vec!["10", "16", "15", "9", "19"],
//         vec!["X", "X", "X", "X", "X"],
//         vec!["22", "11", "13", "6", "5"],
//         vec!["2", "0", "12", "3", "7"],
//     ];

//     let winning_col = vec![
//         vec!["14", "X", "17", "24", "4"],
//         vec!["X", "X", "15", "9", "19"],
//         vec!["X", "X", "23", "26", "20"],
//         vec!["X", "X", "X", "X", "5"],
//         vec!["2", "X", "12", "3", "7"],
//     ];

//     let losing = vec![
//         vec!["X", "21", "X", "24", "4"],
//         vec!["10", "X", "15", "X", "19"],
//         vec!["X", "8", "X", "26", "X"],
//         vec!["X", "11", "X", "X", "5"],
//         vec!["X", "0", "12", "3", "X"],
//     ];

//     assert_eq!(check_win(winning_row), true);
//     assert_eq!(check_win(winning_col), true);
//     assert_eq!(check_win(losing), false);
// }
