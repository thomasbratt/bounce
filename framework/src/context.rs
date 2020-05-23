extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

// pub use sdl2::sdl::Sdl as Sdl;

// // let mut event_pump = sdl_context.event_pump().unwrap();

pub struct Context {
    pub canvas: WindowCanvas,
    pub event_pump: EventPump,
    pub sdl_context: Sdl,
}