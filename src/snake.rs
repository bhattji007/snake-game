
// Import necessary libraries and modules
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

// Import function from the draw module
use crate::draw::draw_block;

// Constants for snake colors
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
const HEAD_COLOR: Color = [0.00, 0.40, 0.30, 1.0];

// Enumeration for snake direction
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Implementation of methods for Direction enum
impl Direction {
    // Method to get opposite direction
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }
}

// Struct representing a single block of the snake
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

// Struct representing the Snake
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

// Implementation of methods for Snake struct
impl Snake {
    // Method to create a new snake with initial position
    fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // Method to draw the snake on the screen
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let mut count = 0;
        for block in &self.body {
            count = count + 1;
            if count == 1 {
                draw_block(SNAKE_COLOR, block.x, block.y, con, g);
            } else {
                draw_block(SNAKE_COLOR, block.x, block.y, con, g);
            }
        }
    }

    // Method to get the position of the snake's head
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    // Method to move the snake forward in a direction
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }
        let (last_x, last_y): (i32, i32) = self.head_position();
        let new_block = match self.direction {
            Direction::Up => Block { x: last_x, y: last_y - 1 },
            Direction::Down => Block { x: last_x, y: last_y + 1 },
            Direction::Left => Block { x: last_x - 1, y: last_y },
            Direction::Right => Block { x: last_x + 1, y: last_y },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    // Method to get the direction of the snake's head
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // Method to get the next position of the snake's head
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();
        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }
        match moving_dir {
            Direction::Down => (head_x, head_y + 1),
            Direction::Up => (head_x, head_y - 1),
            Direction::Right => (head_x + 1, head_y),
            Direction::Left => (head_x - 1, head_y),
        }
    }

    // Method to restore the tail of the snake
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    // Method to check if the snake's head overlaps with its tail
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}
