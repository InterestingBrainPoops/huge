use std::ops::Add;

#[derive(Clone)]
/// The board container.
pub struct Board {
    /// Height of the board
    pub height: u32,
    /// Width of the board
    pub width: u32,
    /// The hazards
    pub hazards: Vec<Coordinate>,
    /// The hazard damage that gets applied per hazard piece.
    /// its not unsigned because health can be increased by hazards (See healing pools)
    pub hazard_damage: i32,
    /// A vector holding all of the food that is on the board
    pub food: Vec<Coordinate>,
    /// The snakes that are currently alive
    pub snakes: Vec<Snake>,
}

impl Board {
    /// Wrap any snakes that are out of bounds, run this BEFORE [Self::out_of_bounds_elims] if you want wrapped behaviour.
    pub fn maybe_wrap_snakes(&mut self) {
        for snake in &mut self.snakes {
            snake.body[0].x %= self.width as i32 - 1;
            snake.body[0].y %= self.height as i32 - 1;
        }
    }

    /// Apply hazard damage
    pub fn apply_hazard_damage(&mut self) {
        for hazard in &self.hazards {
            for snake in &mut self.snakes {
                if snake.head() == *hazard {
                    snake.health -= self.hazard_damage;
                    snake.health = snake.health.min(100);
                }
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
    /// Does not account for wrapped-ness, so run the [Self::maybe_wrap_snakes] function BEFORE this if you need that behaviour
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
    /// Get the head of the snake
    fn head(&self) -> Coordinate {
        self.body[0]
    }
}

/// Base coordinate type, this is with the coordinate system of the bottom left being (0,0)
/// Negative for convienience, especially with negative out of bounds checking
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        let rhs = match rhs {
            Direction::Up => Coordinate::new(0, 1),
            Direction::Down => Coordinate::new(0, -1),
            Direction::Left => Coordinate::new(-1, 0),
            Direction::Right => Coordinate::new(1, 0),
        };

        rhs + self
    }
}

/// A holder for a move made by a snake
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct SnakeMove {
    /// ID of the snake
    pub id: usize,
    /// Direction the snake is moving.
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
