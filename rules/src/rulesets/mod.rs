pub mod standard;
use crate::board::{Board, SnakeMove};

pub trait Ruleset {
    fn make_move(&mut self, board: Board, moves: &[SnakeMove]);
    fn game_over(&self, board: &Board) -> bool;
}
