pub mod standard;
use crate::board::{Board, SnakeMove};

pub trait Ruleset {
    /// Apply a given set of moves to the given board, and the state of the board.
    fn make_move(&mut self, board: &mut Board, moves: &[SnakeMove]);
    /// Check if a board is a game over state
    fn game_over(&self, board: &Board) -> bool;
}
