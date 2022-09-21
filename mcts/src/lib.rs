use eval::Evaluation;
use game::Game;
use rules::rulesets::Ruleset;

pub struct MonteCarloTreeSearch<R: Ruleset, E: Evaluation> {
    ruleset: R,
    evaluation: E,
    root: Node,
}

#[derive(Clone)]
struct Node {
    state: Game,
    children: Vec<Node>,
    num_visits: u64,
    cumulative_score: f64,
    simulated: bool,
}

impl Node {
    fn get_best(&self) -> (usize, &Node) {
        self.children
            .iter()
            .enumerate()
            .max_by(|node1, node2| {
                node1
                    .1
                    .ucb(self.num_visits)
                    .total_cmp(&node2.1.ucb(self.num_visits))
            })
            .unwrap()
    }
    fn ucb(&self, parent_visits: u64) -> f64 {
        if self.num_visits == 0 {
            return f64::MAX - 100.0;
        }
        (self.cumulative_score / (self.num_visits as f64))
            + 2_f64.sqrt() * ((parent_visits as f64).ln() / (self.num_visits as f64)).sqrt()
    }
    fn fill(&mut self) {
        assert!(self.children.is_empty());
    }
}

impl<R: Ruleset, E: Evaluation> MonteCarloTreeSearch<R, E> {
    pub fn new(root: &Game, ruleset: R, evaluation: E) -> MonteCarloTreeSearch<R, E> {
        MonteCarloTreeSearch {
            ruleset,
            evaluation,
            root: Node {
                state: root.clone(),
                children: vec![],
                num_visits: 0,
                cumulative_score: 0.0,
                simulated: false,
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
    pub fn run(&mut self) -> bool {
        let mut path = self.select();

        let leaf_path = &path[0..(path.len() - 1)];
        let leaf_node = self.get(leaf_path);

        if self.ruleset.game_over(&leaf_node.state.board).is_some() {
            return true;
        }

        self.expand(&mut path);

        let reward = self.evaluation.calculate(&leaf_node.state);

        self.backup(&path, reward);

        false
    }

    fn select(&self) -> Vec<usize> {
        let mut path = vec![];
        let mut current = &self.root;
        let mut idx;
        while current.children.iter().all(|x| x.simulated) {
            (idx, current) = current.get_best();
            path.push(idx);
        }
        path
    }

    fn expand(&mut self, path: &mut Vec<usize>) {
        let node = self.get_mut(path);
        node.fill();
        node.simulated = true;
        path.push(node.get_best().0);
    }

    fn backup(&mut self, path: &[usize], mut reward: f64) {
        for x in (1..path.len()).rev() {
            let x = self.get_mut(&path[0..x]);
            x.cumulative_score += reward;
            x.num_visits += 1;
            reward = 1.0 - reward;
        }
    }

    fn get(&self, path: &[usize]) -> Node {
        let mut current = &self.root;
        for x in path {
            current = &current.children[*x];
        }
        current.clone()
    }
    fn get_mut(&mut self, path: &[usize]) -> &mut Node {
        let mut current = &mut self.root;
        for x in path {
            current = &mut current.children[*x];
        }
        current
    }
}
