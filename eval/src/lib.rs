use game::Game;
use rules::board::Board;

pub trait Evaluation {
    fn calculate(&self, game: &Game) -> f64;
}
