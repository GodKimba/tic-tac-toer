use anyhow::{Context, Result};
use core::fmt;
use std::{io::stdin, string::ParseError};

struct Player1 {}

struct Player2 {}

#[derive(Debug, Clone)]
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
        println!("({}, {})", user_cords1.trim(), user_cords2.trim());
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
