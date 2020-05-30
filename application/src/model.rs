use crate::math;
use crate::shape;

use framework::Action;
use math::clamp;
use shape::Shape;
use std::cmp;

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
    pub ball_dx: i32,
    pub ball_dy: i32,
}

impl Model {
    pub fn new(bat: Shape, ball: Shape, ball_dx: i32, ball_dy: i32) -> Self {
        Model {
            bat,
            ball,
            ball_dx,
            ball_dy,
        }
    }
}

pub fn initialize() -> Model {
    Model::new(
        Shape::new(
            (WORLD_WIDTH + BAT_WIDTH) as i32 / 2,
            (WORLD_HEIGHT - 2 * BAT_HEIGHT) as i32,
            BAT_WIDTH,
            BAT_HEIGHT,
        ),
        Shape::new(
            (WORLD_WIDTH / 2 - BALL_RADIUS) as i32,
            BALL_RADIUS as i32,
            2 * BALL_RADIUS,
            2 * BALL_RADIUS,
        ),
        0,
        BALL_MOVE_INCREMENT,
    )
}

pub fn update(action: Action, original: &Model) -> Option<Model> {
    let new_bat_dx = match action {
        Action::Left => -BAT_MOVE_INCREMENT,
        Action::Right => BAT_MOVE_INCREMENT,
        _ => 0,
    };

    let is_hit = is_intersection(&original.bat, &original.ball);

    let new_ball_dx = match action {
        a if a == Action::Left && is_hit => -BALL_MOVE_INCREMENT,
        a if a == Action::Right && is_hit => BALL_MOVE_INCREMENT,
        _ => original.ball_dx,
    };

    let new_ball_dy = if is_hit {
        -BALL_MOVE_INCREMENT
    } else {
        original.ball_dy
    };

    Some(Model::new(
        move_center_position(&original.bat, new_bat_dx, 0, WORLD_WIDTH, WORLD_HEIGHT),
        move_center_position(
            &original.ball,
            new_ball_dx,
            new_ball_dy,
            WORLD_WIDTH,
            WORLD_HEIGHT,
        ),
        new_ball_dx,
        new_ball_dy,
    ))

    // TODO: check for miss when ball goes off screen
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

fn is_intersection(s1: &Shape, s2: &Shape) -> bool {
    cmp::max(s1.x - s2.x, s2.x - s1.x) < (s1.width / 2 + s2.width / 2) as i32
        && cmp::max(s1.y - s2.y, s2.y - s1.y) < (s1.height / 2 + s2.height / 2) as i32
}
