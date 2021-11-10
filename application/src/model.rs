use crate::shape;

use framework::Action;
use shape::Shape;

use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use std::cmp::Ordering;

// TODO: should be part of model? How will window resize work?
pub const WORLD_WIDTH: i32 = 1600;
pub const WORLD_HEIGHT: i32 = 1200;

const BAT_WIDTH: i32 = 200;
const BAT_HEIGHT: i32 = 40;
const BAT_MOVE_INCREMENT: i32 = 25;

const BALL_RADIUS: i32 = 32;
const BALL_MOVE_INCREMENT: i32 = 8;
const BALL_MOVE_JITTER: i32 = 3;

#[derive(Debug)]
pub struct Model {
    pub ball: Shape,
    pub bat: Shape,
    pub world: Shape,
    pub score: u32,
    pub rng: ThreadRng,
    pub normal: Normal<f32>,
}

impl Model {
    pub fn new(
        ball: Shape,
        bat: Shape,
        world: Shape,
        score: u32,
        rng: ThreadRng,
        normal: Normal<f32>,
    ) -> Self {
        Model {
            ball,
            bat,
            world,
            score,
            rng,
            normal,
        }
    }
}

pub fn initialize() -> Model {
    Model::new(
        // Ball
        Shape::new(
            WORLD_WIDTH / 2 - BALL_RADIUS,
            2 * BALL_RADIUS,
            2 * BALL_RADIUS,
            2 * BALL_RADIUS,
            0,
            BALL_MOVE_INCREMENT,
        ),
        // Bat
        Shape::new(
            WORLD_WIDTH / 2 - BAT_WIDTH / 2,
            WORLD_HEIGHT - 2 * BAT_HEIGHT,
            BAT_WIDTH,
            BAT_HEIGHT,
            0,
            0,
        ),
        // World
        Shape::new(0, 0, WORLD_WIDTH, WORLD_HEIGHT, 0, 0),
        // Score
        0,
        rand::thread_rng(),
        Normal::new(0.0, BALL_MOVE_JITTER as f32).unwrap(),
    )
}

pub fn update(action: Action, model: &mut Model) -> Option<Model> {
    let updated_bat = update_bat(action, model);

    match update_ball(&updated_bat, model) {
        BallResult::Hit(updated_ball) => {
            println!("Hit score:{}", model.score + 1);
            Some(Model::new(
                updated_ball,
                updated_bat,
                model.world,
                model.score + 1,
                model.rng,
                model.normal,
            ))
        }
        BallResult::Move(updated_ball) => Some(Model::new(
            updated_ball,
            updated_bat,
            model.world,
            model.score,
            model.rng,
            model.normal,
        )),
        BallResult::Miss(_) => {
            println!("Miss score:{}", model.score);
            let initialized = initialize();
            Some(Model::new(
                initialized.ball,
                initialized.bat,
                initialized.world,
                model.score,
                model.rng,
                model.normal,
            ))
        }
    }
}

fn update_bat(action: Action, original: &Model) -> Shape {
    let updated_bat = match action {
        Action::Left => original.bat.velocity(-BAT_MOVE_INCREMENT, 0).move_step(),
        Action::Right => original.bat.velocity(BAT_MOVE_INCREMENT, 0).move_step(),
        _ => original.bat.velocity(0, 0),
    };

    if updated_bat.left() < original.world.left() {
        updated_bat.velocity(BAT_MOVE_INCREMENT, 0).move_step()
    } else if updated_bat.right() > original.world.right() {
        updated_bat.velocity(-BAT_MOVE_INCREMENT, 0).move_step()
    } else {
        updated_bat
    }
}

enum BallResult {
    Hit(Shape),
    Miss(Shape),
    Move(Shape),
}

fn update_ball(updated_bat: &Shape, original: &mut Model) -> BallResult {
    // Move ball
    let updated_ball: Shape = original.ball.move_step();

    // Ball hits bat?
    let is_hit = shape::is_intersection(updated_bat, &updated_ball);

    // Update ball if hit
    let hit: Shape = if is_hit {
        match updated_bat.dx.cmp(&0) {
            Ordering::Greater => updated_ball
                .velocity(BALL_MOVE_INCREMENT, -BALL_MOVE_INCREMENT)
                .move_step(),
            Ordering::Less => updated_ball
                .velocity(-BALL_MOVE_INCREMENT, -BALL_MOVE_INCREMENT)
                .move_step(),
            Ordering::Equal => updated_ball
                .velocity(updated_ball.dx, -BALL_MOVE_INCREMENT)
                .move_step(),
        }
    } else {
        updated_ball
    };

    // Ball hits horizontal wall?
    let collided_horizontally = if hit.left() < original.world.left() {
        hit.velocity(BALL_MOVE_INCREMENT, hit.dy).move_step()
    } else if hit.right() > original.world.right() {
        hit.velocity(-BALL_MOVE_INCREMENT, hit.dy).move_step()
    } else {
        hit
    };

    // Ball reaches top of world?
    if collided_horizontally.top() < original.world.top() {
        let dx_jitter = Normal::new(0.0, BALL_MOVE_JITTER as f64)
            .unwrap()
            .sample(&mut original.rng) as i32;
        BallResult::Move(
            collided_horizontally
                .velocity(collided_horizontally.dx + dx_jitter, BALL_MOVE_INCREMENT)
                .move_step(),
        )
    } else if collided_horizontally.bottom() > original.world.bottom() {
        BallResult::Miss(collided_horizontally)
    } else if is_hit {
        BallResult::Hit(collided_horizontally)
    } else {
        BallResult::Move(collided_horizontally)
    }
}

pub fn quit(model: Model) {
    println!("Quit. Final model: {:?}", model);
}
