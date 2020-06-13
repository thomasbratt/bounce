use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shape {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub dx: i32,
    pub dy: i32,
}

impl Shape {
    pub fn new(x: i32, y: i32, width: i32, height: i32, dx: i32, dy: i32) -> Self {
        Shape {
            x,
            y,
            width,
            height,
            dx,
            dy,
        }
    }

    pub fn left(self: &Self) -> i32 {
        self.x
    }

    pub fn right(self: &Self) -> i32 {
        self.x + self.width
    }

    pub fn top(self: &Self) -> i32 {
        self.y
    }

    pub fn bottom(self: &Self) -> i32 {
        self.y + self.height
    }

    pub fn velocity(self: &Self, dx: i32, dy: i32) -> Self {
        Shape::new(self.x, self.y, self.width, self.height, dx, dy)
    }

    pub fn move_step(self: &Self) -> Self {
        Shape::new(
            self.x + self.dx,
            self.y + self.dy,
            self.width,
            self.height,
            self.dx,
            self.dy,
        )
    }
}

// TODO: sort out namespace and write unit tests
pub fn is_intersection(s1: &Shape, s2: &Shape) -> bool {
    cmp::max(s1.x - s2.x, s2.x - s1.x) < (s1.width / 2 + s2.width / 2)
        && cmp::max(s1.y - s2.y, s2.y - s1.y) < (s1.height / 2 + s2.height / 2)
}
