use std::fmt::{Display, Formatter, Result as FmtResult, Write};

#[derive(Clone, Copy, Debug)]
pub enum Grid {
    Empty,
    Black,
    White,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            &Grid::Empty => f.write_char('·'),
            &Grid::Black => f.write_char('●'),
            &Grid::White => f.write_char('○'),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub grids: [[Grid; 19]; 19],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grids: [[Grid::Empty; 19]; 19],
        }
    }

    pub fn draw(&self) {
        for row in self.grids {
            for col in row {
                print!("{} ", col);
            }
            println!();
        }
    }
}
