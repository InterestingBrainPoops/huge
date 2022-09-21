use eval::Evaluation;
use game::Game;
use rules::{
    board::{Direction, SnakeMove},
    rulesets::Ruleset,
};

pub const MIN_SCORE: f64 = 0_f64;
pub const MAX_SCORE: f64 = 1_f64;
pub const DRAW_SCORE: f64 = 0.5;

pub struct MinimaxTreeSearch<R: Ruleset, E: Evaluation> {
    ruleset: R,
    evaluation: E,
    root: Game,
}

struct Value {
    direction: Option<Direction>,
    score: f64,
}

pub struct Decision {
    direction: Direction,
    score: f64,
}

impl<R: Ruleset, E: Evaluation> MinimaxTreeSearch<R, E> {
    pub fn new(ruleset: R, eval: E, origin: Game) -> MinimaxTreeSearch<R, E> {
        MinimaxTreeSearch {
            ruleset,
            evaluation: eval,
            root: origin,
        }
    }

    pub fn get_best(&mut self) -> Decision {
        let x = self.minimax(3, MIN_SCORE, MAX_SCORE, None);
        Decision {
            direction: x.direction.unwrap(),
            score: x.score,
        }
    }
    fn generate_all_moves(&self, you_move: SnakeMove) -> Vec<Vec<SnakeMove>> {
        todo!();
    }
    fn minimax(
        &mut self,
        depth: u8,
        mut alpha: f64,
        mut beta: f64,
        you_move: Option<SnakeMove>,
    ) -> Value {
        // check whether or not the game is actually over
        if let Some(game_end) = self.ruleset.game_over(&self.root.board) {
            if let Some(id) = game_end {
                if id == self.root.you_id {
                    // you won
                    return Value {
                        score: MAX_SCORE,
                        direction: None,
                    };
                } else {
                    // you didnt win
                    return Value {
                        score: MIN_SCORE,
                        direction: None,
                    };
                }
            } else {
                // noone won (everyone is dead)
                return Value {
                    score: DRAW_SCORE,
                    direction: None,
                };
            }
        }

        if depth == 0 {
            return Value {
                score: self.evaluation.calculate(&self.root),
                direction: None,
            };
        }

        if let Some(you_move) = you_move {
            // let mut best_moves = vec![];
            let mut value = MAX_SCORE;

            for moves in &self.generate_all_moves(you_move) {
                self.ruleset.make_move(&mut self.root.board, moves);

                let eval = self.minimax(depth - 1, alpha, beta, None);

                if value >= eval.score {
                    // best_moves = moves.clone();
                    value = eval.score;
                }
                if value <= alpha {
                    break;
                }
                beta = beta.min(value);
            }
            Value {
                score: value,
                direction: None,
            }
        } else {
            let mut value = MIN_SCORE;
            let mut out = None;
            if self.ruleset.gen_moves(self.root.you_id).is_empty() {
                // If you don't have any moves, return up with i32::MIN, since you are basically dead
                return Value {
                    score: MAX_SCORE,
                    direction: Some(Direction::Up),
                };
            }
            for current_move in self.ruleset.gen_moves(self.root.you_id) {
                let eval = self.minimax(depth, alpha, beta, Some(current_move));
                if value <= eval.score {
                    out = Some(current_move.direction);
                    value = eval.score;
                }
                if value >= beta {
                    break;
                }
                alpha = alpha.max(value);
            }
            Value {
                score: value,
                direction: out,
            }
        }
    }
}
