use crate::math;
use framework::Action;
use math::clamp;

// TODO: should be part of model
pub const WINDOW_SIZE: (u32, u32) = (1600, 1200);
const START_POSITION_X: i32 = (WINDOW_SIZE.0 + BAT_SIZE.0) as i32 / 2;
const START_POSITION_Y: i32 = (WINDOW_SIZE.1 - 2 * BAT_SIZE.1) as i32;
const BAT_SIZE: (u32, u32) = (200, 40);
const BAT_MOVE_INCREMENT: i32 = 50;

pub struct Model {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Model {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Model {
            x,
            y,
            width,
            height,
        }
    }
}

pub fn initialize() -> Model {
    Model {
        x: START_POSITION_X,
        y: START_POSITION_Y,
        width: BAT_SIZE.0,
        height: BAT_SIZE.1,
    }
}

pub fn update(action: Action, original: &Model) -> Option<Model> {
    match action {
        Action::Left => {
            // println!("Left");
            Some(move_center_position(
                original,
                (-BAT_MOVE_INCREMENT, 0),
                BAT_SIZE,
                WINDOW_SIZE,
            ))
        }
        Action::Right => {
            // println!("Right");
            Some(move_center_position(
                original,
                (BAT_MOVE_INCREMENT, 0),
                BAT_SIZE,
                WINDOW_SIZE,
            ))
        }
        _ => Option::None,
    }
}

fn move_center_position(
    from: &Model,
    offset: (i32, i32),
    shape_size: (u32, u32),
    window_size: (u32, u32),
) -> Model {
    let (dx, dy) = offset;

    let x_min: i32 = (shape_size.0 / 2) as i32;
    let x_max: i32 = (window_size.0 - (shape_size.0 / 2)) as i32;

    let y_min: i32 = (shape_size.1 / 2) as i32;
    let y_max: i32 = (window_size.1 - (shape_size.1 / 2)) as i32;

    Model::new(
        clamp(from.x + dx, x_min, x_max),
        clamp(from.y + dy, y_min, y_max),
        BAT_SIZE.0,
        BAT_SIZE.1,
    )
}
