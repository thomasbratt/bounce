extern crate sdl2;





// use framework::*;
// use initialize;
// use dispatch;

// use bouncing_ball::*;
// use ball;
// use bat;
// use view;

use sdl2::render::WindowCanvas;
use std::time::Duration;

use sdl2::pixels::Color;

use framework::initialize;
use framework::FrameworkError;
use framework::Context;

pub fn main() {

    let mut context: Context = match initialize() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: failed to initialize. {}", e);
            std::process::exit(1);
        }
    };

    const BACKGROUND_COLOR: Color = Color::GRAY;

    context.canvas.set_draw_color(BACKGROUND_COLOR);
    context.canvas.clear();
    context.canvas.present();

    let _ = context.event_pump.poll_iter();

    ::std::thread::sleep(Duration::from_secs(3));

    // let mut event_pump = sdl_context.event_pump().unwrap();
    // 0

    // let mut is_active = false;
    // 'running: loop {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::KeyDown {
    //                 keycode: Some(Keycode::A),
    //                 ..
    //             }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Left),
    //                 ..
    //             } => {
    //                 // = note: destructuring assignments are not currently supported
    //                 // = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
    //                 let next = move_center_position(
    //                     (x, y),
    //                     (-BAT_MOVE_INCREMENT, 0),
    //                     BAT_SIZE,
    //                     WINDOW_SIZE,
    //                 );
    //                 // println!("left {:?}", next);
    //                 x = next.0;
    //                 y = next.1;
    //             }
    //             Event::KeyDown {
    //                 keycode: Some(Keycode::D),
    //                 ..
    //             }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Right),
    //                 ..
    //             } => {
    //                 let next = move_center_position(
    //                     (x, y),
    //                     (BAT_MOVE_INCREMENT, 0),
    //                     BAT_SIZE,
    //                     WINDOW_SIZE,
    //                 );
    //                 // println!("right {:?}", next);
    //                 x = next.0;
    //                 y = next.1;
    //             }
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //
    //         render(&mut canvas, (x, y), BAT_SIZE);
    //         is_active = true;
    //     }
    //
    //     if is_active {
    //         is_active = false
    //     } else {
    //         ::std::thread::sleep(Duration::from_millis(10));
    //     }
    // }
}

