use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/web/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Playing,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    snake_food_cell: Option<usize>,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        let snake = Snake::new(snake_idx, 3);
        let size = width * width;

        World {
            width,
            size,
            snake_food_cell: World::generate_snake_food_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None,
        }
    }

    fn generate_snake_food_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut snake_food_cell;

        loop {
            snake_food_cell = random(max);
            if !snake_body.contains(&SnakeCell(snake_food_cell)) {
                break;
            }
        }

        Some(snake_food_cell)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_food_cell(&self) -> Option<usize> {
        self.snake_food_cell
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Playing);
    }

    pub fn pause_game(&mut self) {
        self.status = None;
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You have won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Playing) => String::from("Playing"),
            None => String::from("Paused"),
        }
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Playing) => {
                let temp = self.snake.body.clone();

                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    }
                    None => {
                        self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
                    }
                }

                let len = self.snake_length();

                for i in 1..len {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..len].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost);
                }

                if self.snake_food_cell == Some(self.snake_head_idx()) {
                    if len < self.size {
                        self.snake_food_cell =
                            World::generate_snake_food_cell(self.size, &self.snake.body)
                    } else {
                        self.snake_food_cell = None;
                        self.status = Some(GameStatus::Won)
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            _ => {}
        }
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
            Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
            Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
            Direction::Up => {
                if row == 0 {
                    SnakeCell((self.width * (self.width - 1)) + snake_idx)
                } else {
                    SnakeCell((snake_idx - self.width) % self.size)
                }
            }
        };
    }
}

// wasm-pack build --target web
