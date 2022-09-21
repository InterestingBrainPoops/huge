use rules::board::Board;

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub you_id: usize,
}
