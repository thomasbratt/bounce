extern crate sdl2;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::video::Window;
use sdl2::render::WindowCanvas;
// use sdl2::video::WindowBuildError;
// use std::time::Duration;
// use std::error::Error;
// use std::fmt;
// use framework;
// use framework::FrameworkError;
use crate::error::FrameworkError;


pub fn initialize() -> Result<WindowCanvas, FrameworkError> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window: sdl2::video::Window = video_subsystem
        .window("Bouncing Ball", 1600, 1200)
        .position_centered()
        .build()?;

    let wc: WindowCanvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()?;

    return Ok(wc);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
