pub struct Shape {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Shape {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Shape {
            x,
            y,
            width,
            height,
        }
    }

    pub fn make_copy(original: &Shape) -> Self {
        let x = original.x;
        let y = original.y;
        let width = original.width;
        let height = original.height;
        Shape {
            x,
            y,
            width,
            height,
        }
    }
}
