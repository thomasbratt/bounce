use framework::initialize;
use std::time::Duration;

mod math;
mod model;
mod shape;
mod view;

pub fn main() {
    match initialize(
        "Bouncing Ball",
        model::WORLD_WIDTH,
        model::WORLD_HEIGHT,
        Option::Some(Duration::from_millis(100)),
    ) {
        Ok(mut dispatcher) => {
            dispatcher.run(model::initialize, model::update, model::quit, view::render)
        }
        Err(e) => {
            eprintln!("Error: failed to initialize. {}", e);
            std::process::exit(1);
        }
    };
}
