extern crate sdl2;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use crate::action::Action;

pub struct Dispatcher {
    canvas: WindowCanvas,
    event_pump: EventPump,
    interval: Option<Duration>,
}

impl Dispatcher {
    pub fn new(canvas: WindowCanvas, event_pump: EventPump, interval: Option<Duration>) -> Self {
        Dispatcher {
            canvas,
            event_pump,
            interval,
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

        let mut timer_action_at = match self.interval {
            Some(x) => Instant::now() + x,
            None => Instant::now(),
        };

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
            match self.interval {
                Some(x) => {
                    if Instant::now() >= timer_action_at {
                        timer_action_at += x;
                        if let Some(new_model) = update_model(Action::Timer, &model) {
                            model = new_model;
                            render(&mut self.canvas, &model);
                        }
                    }
                }
                None => {}
            };
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
