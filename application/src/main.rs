extern crate sdl2;

use sdl2::render::WindowCanvas;
use std::time::Duration;
use sdl2::pixels::Color;

use framework::Action;
use framework::initialize;
use framework::FrameworkError;
use framework::Dispatcher;

pub fn main() {

    match initialize() {
        Ok(mut dispatcher) => dispatcher.run(init_model, update_model, render),
        Err(e) => {
            eprintln!("Error: failed to initialize. {}", e);
            std::process::exit(1);
        }
    };

    println!("OK: Finished");
}

pub struct Model{}

pub fn init_model() -> Model {
    Model{}
}

pub fn update_model(_action: Action, _model: &Model) -> Option<Model> {
    Option::None
}

pub fn render(_canvas: &mut WindowCanvas, _model: &Model) {

}