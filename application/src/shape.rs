use crate::movement::Movement;
use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shape {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub dx: i32,
    pub dy: i32,
    pub movement: Option<Movement>,
}

impl Shape {
    pub fn new(
        x: u32,
        y: u32,
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
            movement: behavior,
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
            self.movement,
        )
    }

    pub fn move_to(self: &Self, new_x: u32, new_y: u32) -> Self {
        Shape::new(
            new_x,
            new_y,
            self.width,
            self.height,
            self.dx,
            self.dy,
            self.movement,
        )
    }

    pub fn move_step(self: &Self) -> Self {
        Shape::new(
            (self.x as i32 + self.dx) as u32,
            (self.y as i32 + self.dy) as u32,
            self.width,
            self.height,
            self.dx,
            self.dy,
            self.movement,
        )
    }
}

// TODO: sort out namespace and write unit tests
pub(crate) fn is_intersection(s1: &Shape, s2: &Shape) -> bool {
    cmp::max(s1.x - s2.x, s2.x - s1.x) < (s1.width / 2 + s2.width / 2)
        && cmp::max(s1.y - s2.y, s2.y - s1.y) < (s1.height / 2 + s2.height / 2)
}
