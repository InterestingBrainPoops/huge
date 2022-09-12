pub struct Board {
    pub height: u32,
    pub width: u32,
    pub hazards: Vec<Coordinate>,
    pub food: Vec<Coordinate>,
    pub snakes: Vec<Snake>,
}

pub struct Snake {
    pub id: usize,
    pub health: i32,
    pub body: Vec<Coordinate>,
}

impl Snake {
    pub fn feed(&mut self) {
        self.duplicate_tail();
        self.health = 100;
    }

    fn duplicate_tail(&mut self) {
        self.body.push(*self.body.last().unwrap());
    }
    pub fn apply_move(&mut self, direction: Direction) {
        self.body.pop();
        let new_head = self.body[0] + direction;
        self.body.insert(0, new_head);
        self.health -= 1;
    }

    pub fn out_of_bounds(&self, width: u8, height: u8) -> bool {
        self.body[0].x < 0
            || self.body[0].x >= width as i32
            || self.body[0].y < 0
            || self.body[0].y >= height as i32
    }

    pub fn snake_body_collision(snake1: &Snake, snake2: &Snake) -> bool {
        snake2.body[1..].contains(&snake1.body[0])
    }

    pub fn snake_lost_head_collision(snake1: &Snake, snake2: &Snake) -> bool {
        snake1.body[0] == snake2.body[0] && snake1.body.len() <= snake2.body.len()
    }

    pub fn get_moves(&self) -> Vec<SnakeMove> {
        vec![
            SnakeMove::new(Direction::Up, self.id),
            SnakeMove::new(Direction::Right, self.id),
            SnakeMove::new(Direction::Left, self.id),
            SnakeMove::new(Direction::Down, self.id),
        ]
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub struct SnakeMove {
    pub id: usize,
    pub direction: Direction,
}

impl SnakeMove {
    pub fn new(dir: Direction, id: usize) -> SnakeMove {
        SnakeMove { direction: dir, id }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
