use std::ops::Add;

pub struct Board {
    pub height: u32,
    pub width: u32,
    pub hazards: Vec<Coordinate>,
    pub hazard_damage: i32,
    pub food: Vec<Coordinate>,
    pub snakes: Vec<Snake>,
}

impl Board {
    pub fn maybe_wrap_snakes(&mut self) {
        for snake in &mut self.snakes {
            if snake.head().x < 0 {
                snake.
            } 
        }
    }

    pub fn apply_hazards(&mut self) {
        for snake in &mut self.snakes {
            if self.hazards.contains(&snake.head()) {
                snake.health -= self.hazard_damage;
                snake.health = snake.health.min(100);
            }
        }
    }

    /// Feed snakes if they are ontop of a food
    pub fn maybe_feed(&mut self) {
        let mut new_food = vec![];
        for food in &self.food {
            let mut eaten = false;
            for snake in &mut self.snakes {
                if *food == snake.body[0] {
                    snake.feed();
                    eaten = true;
                }
            }
            if !eaten {
                new_food.push(*food);
            }
        }
        self.food = new_food;
    }
    /// Check if there are any collisions between snakes, and eliminate snakes if they are collided.
    /// Collisions checked here:
    /// Head to head
    ///  - Snake A is longer than Snake B, Snake A lives, snake B dies.
    ///  - Snake A is the same length as snake B, both snakes die.
    /// Head to body
    ///  - Snake A collided with a bodypart of Snake B
    pub fn apply_collision_eliminations(&mut self) {
        let mut new_snakes = vec![];
        for snake in &self.snakes {
            if Snake::snake_body_collision(snake, snake) {
                continue;
            }

            let mut bodycollision = false;
            for other in &self.snakes {
                if snake.id != other.id && Snake::snake_body_collision(snake, other) {
                    bodycollision = true;
                    break;
                }
            }

            if bodycollision {
                continue;
            }

            let mut headcollision = false;

            for other in &self.snakes {
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
        self.snakes = new_snakes;
    }

    /// Eliminates snakes if they are out of bounds
    /// Does not account for wrapped-ness, so run the .wrap() function BEFORE this if you need that behaviour
    pub fn out_of_bounds_elims(&mut self) {
        let mut new_snakes = vec![];
        for snake in &self.snakes {
            if snake.out_of_bounds(self.width, self.height) {
                continue;
            }
            new_snakes.push(snake.clone());
        }
        self.snakes = new_snakes;
    }

    /// Eliminates snakes if they are out of health
    /// This function does account for health being less than zero, (due to hazards or other)
    pub fn out_of_health_elims(&mut self) {
        let mut new_snakes = vec![];
        for snake in &self.snakes {
            if snake.health <= 0 {
                continue;
            }
            new_snakes.push(snake.clone());
        }
        self.snakes = new_snakes;
    }
}

#[derive(Clone)]
pub struct Snake {
    pub id: usize,
    pub health: i32,
    pub body: Vec<Coordinate>,
}

impl Snake {
    /// Feeds this snake by duplicating its tail and
    pub fn feed(&mut self) {
        self.duplicate_tail();
        self.health = 100;
    }
    /// duplicates its tail ontop of itself
    pub fn duplicate_tail(&mut self) {
        self.body.push(*self.body.last().unwrap());
    }

    /// Applys a given move direction to itself.
    /// This function does remove the tail.
    pub fn apply_move(&mut self, direction: Direction) {
        self.body.pop();
        let new_head = self.body[0] + direction;
        self.body.insert(0, new_head);
        self.health -= 1;
    }

    /// Returns whether or not this snake is out of bounds.
    pub fn out_of_bounds(&self, width: u32, height: u32) -> bool {
        self.body[0].x < 0
            || self.body[0].x >= width as i32
            || self.body[0].y < 0
            || self.body[0].y >= height as i32
    }

    /// Is snake1's head in snake2's body? ("body" is used here to describe the body vector ignoring the head (body[1..]))
    pub fn snake_body_collision(snake1: &Snake, snake2: &Snake) -> bool {
        snake2.body[1..].contains(&snake1.body[0])
    }

    /// Returns true if snake1 and snake2 are in a head to head collision and snake1 lost the encounter (snake1.length <= snake2.length)
    pub fn snake_lost_head_collision(snake1: &Snake, snake2: &Snake) -> bool {
        snake1.body[0] == snake2.body[0] && snake1.body.len() <= snake2.body.len()
    }

    /// Get all possible moves for this snake
    pub fn get_moves(&self) -> Vec<SnakeMove> {
        vec![
            SnakeMove::new(Direction::Up, self.id),
            SnakeMove::new(Direction::Right, self.id),
            SnakeMove::new(Direction::Left, self.id),
            SnakeMove::new(Direction::Down, self.id),
        ]
    }

    fn head(&self) -> Coordinate {
        self.body[0]
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        todo!()
    }
}
impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        todo!()
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct SnakeMove {
    pub id: usize,
    pub direction: Direction,
}

impl SnakeMove {
    pub fn new(dir: Direction, id: usize) -> SnakeMove {
        SnakeMove { direction: dir, id }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
