use std::cell::{RefCell, RefMut};

use rules::rulesets::Ruleset;

pub struct MonteCarloTreeSearch<R: Ruleset, E> {
    ruleset: R,
    evaluation: E,
    root: Node,
}

struct Node {
    // state: Game,
    children: Vec<Node>,
    num_visits: u64,
    cumulative_score: f64,
}

impl<R: Ruleset, E> MonteCarloTreeSearch<R, E> {
    pub fn new(ruleset: R, evaluation: E) -> MonteCarloTreeSearch<R, E> {
        MonteCarloTreeSearch {
            ruleset,
            evaluation,
            root: Node {
                children: vec![],
                num_visits: 0,
                cumulative_score: 0.0,
            },
        }
    }
    /* def run(node, num_rollout):
       "one iteration of select->expand->simulation-> backup"
       path = select(node)
       leaf = path[-1]
       expand(leaf)
       reward = 0
       for i in range(num_rollout):
           reward += simulate(leaf)
       backup(path, reward)
    */
    pub fn run(&mut self) {
        let path = self.select();
        let leaf = &path[0..(path.len() - 2)];
        self.expand(leaf);
        // let reward = self.evaluation.calculate(leaf.state);

        // Self::backup()
    }

    fn select(&self) -> Vec<usize> {
        todo!();
    }

    fn expand(&mut self, path: &[usize]) {
        todo!();
    }

    fn backup(&mut self, path: &[usize], reward: f64) {
        todo!()
    }
}
