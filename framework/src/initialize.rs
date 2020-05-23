extern crate sdl2;

use sdl2::render::WindowCanvas;
use crate::error::FrameworkError;

pub fn initialize() -> Result<WindowCanvas, FrameworkError> {

    // TODO: more errors
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
