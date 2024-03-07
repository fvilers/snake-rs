use crate::{direction::Direction, point::Point};
use std::collections::{vec_deque::Iter, VecDeque};

#[derive(PartialEq)]
enum GameState {
    Playing,
    Over,
}

pub struct Game {
    columns: u16,
    direction: Direction,
    food: Option<Point>,
    rows: u16,
    score: u32,
    snake: VecDeque<Point>,
    state: GameState,
}

const FOOD_SCORE: u32 = 50;

impl Game {
    pub fn new(columns: u16, rows: u16) -> Self {
        Self {
            columns,
            direction: Direction::default(),
            food: None,
            rows,
            score: 0,
            snake: VecDeque::from([Point::default()]),
            state: GameState::Playing,
        }
    }

    pub fn snake(&self) -> Iter<'_, Point> {
        self.snake.iter()
    }

    pub const fn food(&self) -> &Option<Point> {
        &self.food
    }

    pub const fn score(&self) -> u32 {
        self.score
    }

    pub fn up(&mut self) {
        if self.snake.len() == 1 || self.direction != Direction::Down {
            self.direction = Direction::Up;
        }
    }

    pub fn right(&mut self) {
        if self.snake.len() == 1 || self.direction != Direction::Left {
            self.direction = Direction::Right;
        }
    }

    pub fn down(&mut self) {
        if self.snake.len() == 1 || self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
    }

    pub fn left(&mut self) {
        if self.snake.len() == 1 || self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
    }

    pub fn is_over(&self) -> bool {
        self.state == GameState::Over
    }

    pub fn tick(&mut self) {
        let mut new_head = *self
            .snake
            .front()
            .expect("The snake vector should never be empty");

        match self.direction {
            Direction::Up => {
                new_head.y = new_head.y.checked_sub(1).unwrap_or(self.rows - 1);
            }
            Direction::Right => {
                new_head.x = (new_head.x + 1) % self.columns;
            }
            Direction::Down => {
                new_head.y = (new_head.y + 1) % self.rows;
            }
            Direction::Left => {
                new_head.x = new_head.x.checked_sub(1).unwrap_or(self.columns - 1);
            }
        };

        if self.snake.contains(&new_head) {
            self.state = GameState::Over;
            return;
        }

        if let Some(food) = &self.food {
            if new_head == *food {
                self.score += FOOD_SCORE;
                self.food = None;
            } else {
                self.snake.pop_back();
            }
        } else {
            self.snake.pop_back();
        }

        self.snake.push_front(new_head);

        if self.food.is_none() {
            let mut food = Point::random(self.columns, self.rows);

            // TODO: improve this as it could lead to an infinite loop when the snake will take the whole space
            while self.snake.contains(&food) {
                food = Point::random(self.columns, self.rows);
            }

            self.food = Some(food);
        }
    }
}
