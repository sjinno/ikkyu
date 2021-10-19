mod board;
mod game;

use board::Board;
use game::GameBuilder;

fn main() {
    let board = Board::new();
    let mut game = GameBuilder::build(board);
    game.board.draw();
}
