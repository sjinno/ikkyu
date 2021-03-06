use super::board::{Board, Grid};

use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
    Black,
    White,
}

// Probably not going to have this feature just yet.
// enum Agehama {
//     Black(u8),
//     White(u8),
// }

pub struct GameBuilder {
    pub board: Board,
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    turn: Turn,
}

#[derive(Debug)]
pub enum ParseError {
    WrongNumberOfInput,
    InvalidInput,
}

impl GameBuilder {
    pub fn build(board: Board) -> Game {
        Game {
            board,
            turn: Turn::Black,
        }
    }
}

impl Game {
    pub fn init(&mut self) {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = match parse_input(input) {
                Ok(parsed_input) => parsed_input,
                Err(ParseError::WrongNumberOfInput) => {
                    println!("WRONG NUMBER OF INPUT. PLEASE TRY AGAIN.");
                    [42, 42]
                }
                Err(ParseError::InvalidInput) => {
                    println!("INVALID INPUT FOUND. PLEASE TRY AGAIN.");
                    [42, 42]
                }
            };

            // Input failed.
            if input == [42, 42] {
                continue;
            };

            if self.turn == Turn::Black {
                self.board.grids[input[1] - 1][input[0] - 1] = Grid::Black;
                self.turn = Turn::White;
            } else {
                self.board.grids[input[1] - 1][input[0] - 1] = Grid::White;
                self.turn = Turn::Black;
            }

            self.board.draw();
        }
    }
}

fn parse_input(input: String) -> Result<[usize; 2], ParseError> {
    let split: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap_or(42))
        .collect();
    dbg!(&split);

    if split.len() != 2 {
        Err(ParseError::WrongNumberOfInput)
    } else if split.iter().any(|v| *v > 19 || *v < 1) {
        Err(ParseError::InvalidInput)
    } else {
        Ok([split[0], split[1]])
    }
}
