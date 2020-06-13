use crate::shape;

use framework::Action;
use shape::Shape;

// TODO: should be part of model? How will window resize work?
pub const WORLD_WIDTH: i32 = 1600;
pub const WORLD_HEIGHT: i32 = 1200;

const BAT_WIDTH: i32 = 200;
const BAT_HEIGHT: i32 = 40;
const BAT_MOVE_INCREMENT: i32 = 25;

const BALL_RADIUS: i32 = 32;
const BALL_MOVE_INCREMENT: i32 = 8;

#[derive(Debug)]
pub struct Model {
    pub ball: Shape,
    pub bat: Shape,
    pub world: Shape,
    pub score: u32,
}

impl Model {
    pub fn new(ball: Shape, bat: Shape, world: Shape, score: u32) -> Self {
        Model {
            ball,
            bat,
            world,
            score,
        }
    }
}

// TODO: shrink world by 25 in all directions, to avoid bat/ball updates going off screen
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
        0,
    )
}

pub fn update(action: Action, original: &Model) -> Option<Model> {
    let updated_bat = update_bat(action, original);

    match update_ball(&updated_bat, original) {
        BallResult::Hit(updated_ball) => {
            println!("Hit score:{}", original.score + 1);
            Some(Model::new(
                updated_ball,
                updated_bat,
                original.world,
                original.score + 1,
            ))
        }
        BallResult::Move(updated_ball) => Some(Model::new(
            updated_ball,
            updated_bat,
            original.world,
            original.score,
        )),
        BallResult::Miss(_) => {
            println!("Miss score:{}", original.score);
            let initialized = initialize();
            Some(Model::new(
                initialized.ball,
                initialized.bat,
                initialized.world,
                original.score,
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

fn update_ball(updated_bat: &Shape, original: &Model) -> BallResult {
    // Move ball
    let updated_ball: Shape = original.ball.move_step();

    // Ball hits bat?
    let is_hit = shape::is_intersection(&updated_bat, &updated_ball);
    let hit: Shape = if is_hit {
        if updated_bat.dx > 0 {
            updated_ball
                .velocity(BALL_MOVE_INCREMENT, -BALL_MOVE_INCREMENT)
                .move_step()
        } else if updated_bat.dx < 0 {
            updated_ball
                .velocity(-BALL_MOVE_INCREMENT, -BALL_MOVE_INCREMENT)
                .move_step()
        } else {
            updated_ball
                .velocity(updated_ball.dx, -BALL_MOVE_INCREMENT)
                .move_step()
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
        BallResult::Move(
            collided_horizontally
                .velocity(collided_horizontally.dx, BALL_MOVE_INCREMENT)
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
