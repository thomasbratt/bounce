use framework::initialize;
use std::time::Duration;

mod model;
mod shape;
mod view;

pub fn main() {
    match initialize(
        "Bouncing Ball",
        model::WORLD_WIDTH as u32,
        model::WORLD_HEIGHT as u32,
        Duration::from_millis(30),
        // Duration::from_millis(10),
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
