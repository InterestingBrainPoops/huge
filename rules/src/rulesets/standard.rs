use super::Ruleset;

pub struct Standard;

impl Ruleset for Standard {
    fn make_move(&mut self, board: crate::board::Board, moves: &[crate::board::SnakeMove]) {
        // move all the snakes
        for snake in &mut board.snakes {
            let move_pos = moves.iter().position(|s| s.id == snake.id).unwrap();
            snake.apply_move(moves[move_pos].direction);
        }
        // feed the snakes
        let mut new_food = vec![];
        for food in &board.food {
            let mut eaten = false;
            for snake in &mut board.snakes {
                if *food == snake.body[0] {
                    snake.feed();
                    eaten = true;
                }
            }
            if !eaten {
                new_food.push(*food);
            }
        }
        board.food = new_food;

        // out of bounds eliminations
        let mut new_snakes = vec![];
        for snake in &board.snakes {
            if snake.out_of_bounds(board.width, board.height) {
                continue;
            }

            if snake.health == 0 {
                continue;
            }

            new_snakes.push(snake.clone());
        }
        self.board.snakes = new_snakes;

        // collision eliminations
        let mut new_snakes = vec![];
        for snake in &board.snakes {
            if Snake::snake_body_collision(snake, snake) {
                continue;
            }

            let mut bodycollision = false;
            for other in &board.snakes {
                if snake.id != other.id && Snake::snake_body_collision(snake, other) {
                    bodycollision = true;
                    break;
                }
            }

            if bodycollision {
                continue;
            }

            let mut headcollision = false;

            for other in &board.snakes {
                if snake.id != other.id && Snake::snake_lost_head_collision(snake, other) {
                    headcollision = true;
                    break;
                }
            }

            if headcollision {
                continue;
            }
            new_snakes.push(snake.clone());
        }
        board.snakes = new_snake
    }

    fn game_over(&self, board: &crate::board::Board) -> bool {
        // no snakes left
        let no_snakes = board.snakes.is_empty();
        // one snake left
        let one_left = board.snakes.len() == 1;
        no_snakes || one_left
    }
}
