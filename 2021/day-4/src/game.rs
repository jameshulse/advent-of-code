use itertools::Itertools;
use std::fmt;

#[derive(fmt::Debug)]
pub struct Place {
    pub value: String,
    pub drawn: bool,
}

#[derive(fmt::Debug)]
pub struct Board {
    pub rows: Vec<Vec<Place>>,
    pub won: bool,
}

impl Board {
    pub fn mark_draw(&mut self, draw: &str) {
        for row in self.rows.iter_mut() {
            for col in row.iter_mut() {
                if col.value == draw {
                    col.drawn = true
                }
            }
        }
    }

    pub fn check_win(&mut self) -> bool {
        if self.won {
            return self.won;
        }

        let mut is_winner = false;

        // Check rows
        for row in self.rows.iter() {
            if row.iter().all(|v| v.drawn) {
                is_winner = true;
                break;
            }
        }
        // Check columns
        if !is_winner {
            for i in 0..self.rows[0].len() {
                let values = self.rows.iter().map(|l| &l[i]).collect_vec();

                if values.iter().all(|v| v.drawn) {
                    is_winner = true;
                    break;
                }
            }
        }

        self.won = is_winner;

        is_winner
    }

    pub fn calculate_total(&self) -> usize {
        let mut total = 0;
        for row in self.rows.iter() {
            for col in row.iter() {
                if !col.drawn {
                    total += col.value.parse::<usize>().unwrap();
                }
            }
        }
        total
    }
}
