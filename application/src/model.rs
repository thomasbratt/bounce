use crate::math;
use crate::shape;

use framework::Action;
use math::clamp;
use shape::Shape;

// TODO: should be part of model? How will window resize work?
pub const WORLD_WIDTH: u32 = 1600;
pub const WORLD_HEIGHT: u32 = 1200;

const BAT_WIDTH: u32 = 200;
const BAT_HEIGHT: u32 = 40;
const BAT_MOVE_INCREMENT: i32 = 50;

const BALL_RADIUS: u32 = 32;
const BALL_MOVE_INCREMENT: i32 = 8;

#[derive(Debug)]
pub struct Model {
    pub bat: Shape,
    pub ball: Shape,
}

impl Model {
    pub fn new(bat: Shape, ball: Shape) -> Self {
        Model { bat, ball }
    }
}

pub fn initialize() -> Model {
    Model {
        bat: Shape::new(
            (WORLD_WIDTH + BAT_WIDTH) as i32 / 2,
            (WORLD_HEIGHT - 2 * BAT_HEIGHT) as i32,
            BAT_WIDTH,
            BAT_HEIGHT,
        ),
        ball: Shape::new(
            (WORLD_WIDTH / 2 - BALL_RADIUS) as i32,
            BALL_RADIUS as i32,
            2 * BALL_RADIUS,
            2 * BALL_RADIUS,
        ),
    }
}

pub fn update(action: Action, original: &Model) -> Option<Model> {
    match action {
        Action::Left => {
            // println!("Left");
            Some(Model::new(
                move_center_position(
                    &original.bat,
                    -BAT_MOVE_INCREMENT,
                    0,
                    WORLD_WIDTH,
                    WORLD_HEIGHT,
                ),
                Shape::make_copy(&original.ball),
            ))
        }
        Action::Right => {
            // println!("Right");
            Some(Model::new(
                move_center_position(
                    &original.bat,
                    BAT_MOVE_INCREMENT,
                    0,
                    WORLD_WIDTH,
                    WORLD_HEIGHT,
                ),
                Shape::make_copy(&original.ball),
            ))
        }
        Action::Timer => Some(Model::new(
            Shape::make_copy(&original.bat),
            move_center_position(
                &original.ball,
                0,
                BALL_MOVE_INCREMENT,
                WORLD_WIDTH,
                WORLD_HEIGHT,
            ),
        )),
        _ => Option::None,
    }
}

pub fn quit(model: Model) {
    println!("Quit. Save model etc: {:?}", model);
}

fn move_center_position(
    original: &Shape,
    dx: i32,
    dy: i32,
    world_width: u32,
    world_height: u32,
) -> Shape {
    let x_min: i32 = (original.width / 2) as i32;
    let x_max: i32 = (world_width - (original.width / 2)) as i32;

    let y_min: i32 = (original.height / 2) as i32;
    let y_max: i32 = (world_height - (original.height / 2)) as i32;

    Shape::new(
        clamp(original.x + dx, x_min, x_max),
        clamp(original.y + dy, y_min, y_max),
        original.width,
        original.height,
    )
}
