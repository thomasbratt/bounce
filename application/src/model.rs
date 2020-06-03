use crate::math;
use crate::shape;

use framework::Action;
use math::clamp;
use shape::is_intersection;
use shape::Shape;

// TODO: should be part of model? How will window resize work?
pub const WORLD_WIDTH: u32 = 1600;
pub const WORLD_HEIGHT: u32 = 1200;

const BAT_WIDTH: u32 = 200;
const BAT_HEIGHT: u32 = 40;
const BAT_MOVE_INCREMENT: i32 = 25;

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
    Model::new(
        Shape::new(
            (WORLD_WIDTH + BAT_WIDTH) as i32 / 2,
            (WORLD_HEIGHT - 2 * BAT_HEIGHT) as i32,
            BAT_WIDTH,
            BAT_HEIGHT,
            0,
            0,
        ),
        Shape::new(
            (WORLD_WIDTH / 2 - BALL_RADIUS) as i32,
            BALL_RADIUS as i32,
            2 * BALL_RADIUS,
            2 * BALL_RADIUS,
            0,
            BALL_MOVE_INCREMENT,
        ),
    )
}

pub fn update(action: Action, original: &Model) -> Option<Model> {
    let new_bat = match action {
        Action::Left => original.bat.velocity(-BAT_MOVE_INCREMENT, 0).move_step(),
        Action::Right => original.bat.velocity(BAT_MOVE_INCREMENT, 0).move_step(),
        _ => original.bat,
    };

    let is_hit = is_intersection(&original.bat, &original.ball);

    let (new_ball_dx, new_ball_dy) = match action {
        a if a == Action::Left && is_hit => (-BALL_MOVE_INCREMENT, -BALL_MOVE_INCREMENT),
        a if a == Action::Right && is_hit => (BALL_MOVE_INCREMENT, -BALL_MOVE_INCREMENT),
        _ => (original.ball.dx, original.ball.dy),
    };
    let new_ball = original.ball.velocity(new_ball_dx, new_ball_dy).move_step();

    Some(Model::new(new_bat, new_ball))

    // TODO: check for miss when ball goes off screen
}

pub fn quit(model: Model) {
    println!("Quit. Save model etc: {:?}", model);
}

// fn move_center_position(
//     original: &Shape,
//     dx: i32,
//     dy: i32,
//     world_width: u32,
//     world_height: u32,
// ) -> Shape {
//     let x_min: i32 = (original.width / 2) as i32;
//     let x_max: i32 = (world_width - (original.width / 2)) as i32;
//
//     let y_min: i32 = (original.height / 2) as i32;
//     let y_max: i32 = (world_height - (original.height / 2)) as i32;
//
//     Shape::new(
//         clamp(original.x + dx, x_min, x_max),
//         clamp(original.y + dy, y_min, y_max),
//         original.width,
//         original.height,
//         original.dx,
//         original.dy
//     )
// }
