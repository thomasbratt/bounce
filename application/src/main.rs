use framework::initialize;
use std::time::Duration;

mod math;
mod model;
mod view;

pub fn main() {
    match initialize(
        "Bouncing Ball",
        model::WINDOW_SIZE.0,
        model::WINDOW_SIZE.1,
        Option::Some(Duration::from_secs(3)),
    ) {
        Ok(mut dispatcher) => dispatcher.run(model::initialize, model::update, view::render),
        Err(e) => {
            eprintln!("Error: failed to initialize. {}", e);
            std::process::exit(1);
        }
    };
}
