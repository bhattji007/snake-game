use std::collections::LinkedList;
use piston_window::{Context,G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR:Color=[0.00,0.80,0.00,1.0];

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction{
    pub fn opposite(&self)->Direction{     //so that snake ignore the command of going exact opposite direction
        match *self{
            Direction::Down=>Direction::Up,
            Direction::Left=>Direction::Right,
            Direction::Right=>Direction::Left,
            Direction::Up=>Direction::Down
        }
    }
}

struct Block{
    x:i32,
    y:i32
}
