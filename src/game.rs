use super::board::Board;

#[derive(Debug, Clone, Copy)]
pub enum Turn {
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
    pub turn: Turn,
}

impl GameBuilder {
    pub fn build(board: Board) -> Game {
        Game {
            board,
            turn: Turn::Black,
        }
    }
}
