use framework::initialize;
use framework::Action;
use framework::Dispatcher;
use framework::FrameworkError;

mod math;
mod model;
mod view;

pub fn main() {
    match initialize() {
        Ok(mut dispatcher) => {
            dispatcher.run(model::initialize, model::update, view::render)
        }
        Err(e) => {
            eprintln!("Error: failed to initialize. {}", e);
            std::process::exit(1);
        }
    };

    println!("OK: Finished");
}
