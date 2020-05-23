extern crate sdl2;

use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use crate::context::Context;
use crate::error::FrameworkError;



pub fn initialize() -> Result<Context, FrameworkError> {

    // TODO: more errors
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window: sdl2::video::Window = video_subsystem
        .window("Bouncing Ball", 1600, 1200)
        .position_centered()
        .build()?;

    let canvas: WindowCanvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()?;

    // TODO: error handling as above ^
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();

    // // let mut event_pump = sdl_context.event_pump().unwrap();
    Ok(Context{canvas, event_pump, sdl_context})
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
