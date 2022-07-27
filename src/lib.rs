use anyhow::{Context, Result};
use core::fmt;
use std::io::stdin;

struct Player1 {}

struct Player2 {}
const BOARD_HEIGHT: u8 = 3;
const BOARD_LENGTH: u8 = 3;

#[derive(Debug, Clone, PartialEq)]
pub enum MoveStates {
    X,
    O,
    N,
}

impl fmt::Display for MoveStates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MoveStates::X => write!(f, "X"),
            MoveStates::O => write!(f, "O"),
            MoveStates::N => write!(f, "N"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    board: Vec<Vec<MoveStates>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![
                vec![MoveStates::N, MoveStates::N, MoveStates::N],
                vec![MoveStates::N, MoveStates::N, MoveStates::N],
                vec![MoveStates::N, MoveStates::N, MoveStates::N],
            ],
        }
    }
    pub fn get_move() -> Result<(usize, usize)> {
        println!("What is your move's X co-ordinate?: ");
        let mut user_cords1 = String::new();
        stdin()
            .read_line(&mut user_cords1)
            .ok()
            .expect("Failed to read line.");

        println!("What is your move's Y co-ordinate?: ");
        let mut user_cords2 = String::new();
        stdin()
            .read_line(&mut user_cords2)
            .ok()
            .expect("Failed to read line.");

        let coords = (
            user_cords1
                .trim()
                .parse::<usize>()
                .context("Failed to parse")?,
            user_cords2
                .trim()
                .parse::<usize>()
                .context("Failed to parse")?,
        );
        Ok(coords)
    }

    pub fn make_a_move(&mut self, cords: (usize, usize), player: MoveStates) -> &mut Self {
        let (x, y) = cords;
        match self.board[x][y] {
            MoveStates::X => panic!("Illegal move!"),
            MoveStates::O => panic!("Illegal move!"),
            MoveStates::N => self.board[x][y] = player,
        }
        self
    }
    pub fn get_winner(&self) -> bool {
        let board = self.board.clone();
        let (cols, rows, diagonals) = Game::get_cols();
        let mut state_counter_x = 0;
        let mut state_counter_y = 0;
        let mut counter = 0;
        for (x, y) in cols {
            counter += 1;
            if counter == 7 {
                state_counter_x = 0;
                state_counter_y = 0;
                counter = 0;
            };

            match board[x][y] {
                MoveStates::X => state_counter_x += 1,
                MoveStates::O => state_counter_y += 1,
                MoveStates::N => state_counter_x += 0,
            }
            if state_counter_x >= 3 {
                println!("X won");
                return true;
            } else if state_counter_y >= 3 {
                println!("Y won");
                return true;
            }
        }

        state_counter_x = 0;
        state_counter_y = 0;
        counter = 0;
        for (x, y) in rows {
            counter += 1;
            if counter == 7 {
                state_counter_x = 0;
                state_counter_y = 0;
                counter = 0;
            };
            match board[x][y] {
                MoveStates::X => state_counter_x += 1,
                MoveStates::O => state_counter_y += 1,
                MoveStates::N => state_counter_x += 0,
            }
            if state_counter_x >= 3 {
                println!("X won");
                return true;
            } else if state_counter_y >= 3 {
                println!("Y won");
                return true;
            }
        }

        state_counter_x = 0;
        state_counter_y = 0;
        counter = 0;
        for (x, y) in diagonals {
            counter += 1;
            if counter == 7 {
                state_counter_x = 0;
                state_counter_y = 0;
                counter = 0;
            };
            match board[x][y] {
                MoveStates::X => state_counter_x += 1,
                MoveStates::O => state_counter_y += 1,
                MoveStates::N => state_counter_x += 0,
            }
            if state_counter_x >= 3 {
                println!("X won");
                return true;
            } else if state_counter_y >= 3 {
                println!("Y won");
                return true;
            }
        }
        return false;
    }

    pub fn get_cols() -> (
        Vec<(usize, usize)>,
        Vec<(usize, usize)>,
        Vec<(usize, usize)>,
    ) {
        let mut cols = vec![];
        let mut rows = vec![];
        for i in 0..3 {
            for j in 0..3 {
                cols.push((i, j));
            }
        }

        for j in 0..3 {
            for i in 0..3 {
                rows.push((i, j));
            }
        }
        let diagonals = vec![(0, 0), (1, 1), (2, 2), (0, 2), (2, 0)];
        (cols, rows, diagonals)
    }

    pub fn render(&self) {
        println!(
            "    0    1    2\n-----------------\n0 | {}, | {}, | {} |",
            self.board[0][0], self.board[0][1], self.board[0][2]
        );
        println!("-----------------");
        println!(
            "1 | {}, | {}, | {} |",
            self.board[1][0], self.board[1][1], self.board[1][2]
        );
        println!("-----------------");
        println!(
            "2 | {}, | {}, | {} |",
            self.board[2][0], self.board[2][1], self.board[2][2]
        );
        println!("-----------------");
    }
}
