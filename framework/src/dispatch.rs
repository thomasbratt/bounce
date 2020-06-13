extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use crate::action::Action;
use crate::polltimer::PollTimer;

pub struct Dispatcher {
    canvas: WindowCanvas,
    event_pump: EventPump,
    pull_timer: PollTimer,
}

impl Dispatcher {
    pub fn new(canvas: WindowCanvas, event_pump: EventPump, interval: Duration) -> Self {
        let pull_timer = PollTimer::new(interval);
        Dispatcher {
            canvas,
            event_pump,
            pull_timer,
        }
    }

    pub fn run<Model>(
        &mut self,
        initialize_model: fn() -> Model,
        update_model: fn(action: Action, model: &mut Model) -> Option<Model>,
        cleanup_model: fn(model: Model),
        render: fn(canvas: &mut WindowCanvas, model: &Model),
    ) {
        let mut model: Model = initialize_model();

        render(&mut self.canvas, &model);

        loop {
            for action in self.next_actions() {
                match action {
                    Action::Quit => {
                        cleanup_model(model);
                        return;
                    }
                    action => {
                        if let Some(new_model) = update_model(action, &mut model) {
                            model = new_model;
                            render(&mut self.canvas, &model);
                        }
                    }
                }
            }
        }
    }

    fn next_actions(self: &mut Self) -> Vec<Action> {
        loop {
            if self.pull_timer.is_elapsed() {
                self.pull_timer = self.pull_timer.reset();

                if let Some(Event::Quit { .. }) = self.event_pump.poll_event() {
                    return vec![Action::Quit];
                }

                // Get key press without delay.
                let actions: Vec<Action> = self
                    .event_pump
                    .keyboard_state()
                    .pressed_scancodes()
                    .filter_map(Keycode::from_scancode)
                    .filter_map(Dispatcher::extract_action)
                    .collect();

                return if actions.is_empty() {
                    vec![Action::Timer]
                } else {
                    actions
                };
            } else {
                ::std::thread::sleep(self.pull_timer.remaining());
            }
        }
    }

    fn extract_action(keycode: Keycode) -> Option<Action> {
        match keycode {
            Keycode::Left | Keycode::A => Some(Action::Left),
            Keycode::Right | Keycode::D => Some(Action::Right),
            Keycode::Escape => Some(Action::Quit),
            _ => None,
        }
    }
}
