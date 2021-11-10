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

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    pub fn velocity(&self, dx: i32, dy: i32) -> Self {
        Shape::new(self.x, self.y, self.width, self.height, dx, dy)
    }

    pub fn move_step(&self) -> Self {
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

// TODO: write unit tests
pub fn is_intersection(a: &Shape, b: &Shape) -> bool {
    is_intersection_interval(a.x, a.width, b.x, b.width)
        && is_intersection_interval(a.y, a.height, b.y, b.height)
}

fn is_intersection_interval(a: i32, a_extent: i32, b: i32, b_extent: i32) -> bool {
    if a < b {
        a + a_extent > b
    } else {
        b + b_extent > a
    }
}
