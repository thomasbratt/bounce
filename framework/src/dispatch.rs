extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::Sdl;

use crate::action::Action;

// use self::sdl2::event::Event;

// Elm-like loop:
//
// add frame timer duration to initialize
// event type: frame, key, mouse
//
// init_model -> M
// update_model(event, M) -> M
// render(canvas, M)
//
// Note that model generation process can happen async, so long as it provides an implementation
// of the required interfaces.
//
// initialize should return Result<Dispatcher, FrameworkError>

pub struct Dispatcher {
    canvas: WindowCanvas,
    event_pump: EventPump,
    sdl_context: Sdl,
}

impl Dispatcher {
    pub fn new(canvas: WindowCanvas, event_pump: EventPump, sdl_context: Sdl) -> Self {
        Dispatcher {
            canvas,
            event_pump,
            sdl_context,
        }
    }

    pub fn run<Model>(
        &mut self,
        init_model: fn() -> Model,
        update_model: fn(action: Action, model: &Model) -> Option<Model>,
        render: fn(canvas: &mut WindowCanvas, model: &Model),
    ) {
        let mut model: Model = init_model();

        render(&mut self.canvas, &model);

        'running: loop {
            for event in self.event_pump.wait_timeout_iter(10) {
                match Dispatcher::extract_action(event) {
                    Action::Quit => {
                        break 'running;
                    }
                    action => {
                        if let Some(new_model) = update_model(action, &model) {
                            model = new_model;
                            render(&mut self.canvas, &model);
                        }
                    }
                }
            }
        }
    }

    fn extract_action(event: Event) -> Action {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => Action::Left,
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => Action::Right,
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Action::Quit,
            _ => Action::None,
        }
    }
}
