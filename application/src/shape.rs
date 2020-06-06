use crate::behavior::Movement;
use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shape {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub dx: i32,
    pub dy: i32,
    pub behavior: Option<Movement>,
}

impl Shape {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        dx: i32,
        dy: i32,
        behavior: Option<Movement>,
    ) -> Self {
        Shape {
            x,
            y,
            width,
            height,
            dx,
            dy,
            behavior,
        }
    }

    pub fn velocity(self: &Self, new_dx: i32, new_dy: i32) -> Self {
        Shape::new(
            self.x,
            self.y,
            self.width,
            self.height,
            new_dx,
            new_dy,
            self.behavior,
        )
    }

    pub fn move_to(self: &Self, new_x: i32, new_y: i32) -> Self {
        Shape::new(
            new_x,
            new_y,
            self.width,
            self.height,
            self.dx,
            self.dy,
            self.behavior,
        )
    }

    pub fn move_step(self: &Self) -> Self {
        Shape::new(
            self.x + self.dx,
            self.y + self.dy,
            self.width,
            self.height,
            self.dx,
            self.dy,
            self.behavior,
        )
    }
}

pub(crate) fn is_intersection(s1: &Shape, s2: &Shape) -> bool {
    cmp::max(s1.x - s2.x, s2.x - s1.x) < (s1.width / 2 + s2.width / 2) as i32
        && cmp::max(s1.y - s2.y, s2.y - s1.y) < (s1.height / 2 + s2.height / 2) as i32
}
