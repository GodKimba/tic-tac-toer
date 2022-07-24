use anyhow::Result;
use core::fmt;
use std::io::stdin;

struct Player1 {}

struct Player2 {}

#[derive(Debug)]
enum MoveStates {
    X,
    O,
    None,
}

impl fmt::Display for MoveStates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MoveStates::X => write!(f, "X"),
            MoveStates::O => write!(f, "O"),
            MoveStates::None => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    board: Vec<Vec<MoveStates>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![
                vec![MoveStates::None, MoveStates::X, MoveStates::None],
                vec![MoveStates::None, MoveStates::X, MoveStates::None],
                vec![MoveStates::None, MoveStates::X, MoveStates::None],
            ],
        }
    }
    pub fn get_move() -> (u8, u8) {
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
                .parse::<u8>()
                .ok()
                .expect("Program only process numbers."),
            user_cords2
                .parse::<u8>()
                .ok()
                .expect("Program only process numbers."),
        );

        println!("({}, {})", user_cords1.trim(), user_cords2.trim());
        coords
    }

    pub fn make_a_move(&mut self, cords: (u8, u8), player: MoveStates) -> Self {}

    pub fn render(&self) {
        println!(
            "     0      1     2\n-----------------------\n1 | {}, | {}, | {} |",
            self.board[0][0], self.board[0][1], self.board[0][2]
        );
        println!("-----------------------");
        println!(
            "2 | {}, | {}, | {} |",
            self.board[1][0], self.board[1][1], self.board[1][2]
        );
        println!("-----------------------");
        println!(
            "3 | {}, | {}, | {} |",
            self.board[2][0], self.board[2][1], self.board[2][2]
        );
        println!("-----------------------");
    }
}
