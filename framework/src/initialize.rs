extern crate sdl2;

use crate::dispatch::Dispatcher;
use crate::error::FrameworkError;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use std::time::Duration;

pub fn initialize(
    title: &str,
    width: u32,
    height: u32,
    interval: Option<Duration>,
) -> Result<Dispatcher, FrameworkError> {
    // TODO: more errors
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window: sdl2::video::Window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()?;

    let canvas: WindowCanvas = window.into_canvas().accelerated().present_vsync().build()?;

    // TODO: error handling as above ^
    let event_pump: EventPump = sdl_context.event_pump().unwrap();

    Ok(Dispatcher::new(canvas, event_pump, interval))
}
