use super::Ruleset;

pub struct Standard;

impl Ruleset for Standard {
    fn make_move(&mut self, board: &mut crate::board::Board, moves: &[crate::board::SnakeMove]) {
        // move all the snakes
        for snake in &mut board.snakes {
            let move_pos = moves.iter().position(|s| s.id == snake.id).unwrap();
            snake.apply_move(moves[move_pos].direction);
        }
        // feed the snakes
        board.maybe_feed();

        // out of bounds eliminations
        board.out_of_bounds_elims();

        // out of health eliminations
        board.out_of_health_elims();

        // collision eliminations
        board.apply_collision_eliminations();
    }

    fn game_over(&self, board: &crate::board::Board) -> bool {
        // no snakes left
        let no_snakes = board.snakes.is_empty();
        // one snake left
        let one_left = board.snakes.len() == 1;
        no_snakes || one_left
    }
}
