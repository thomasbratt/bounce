extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use crate::action::Action;
use crate::pull_timer::PullTimer;

pub struct Dispatcher {
    canvas: WindowCanvas,
    event_pump: EventPump,
    pull_timer: Option<PullTimer>,
}

impl Dispatcher {
    pub fn new(canvas: WindowCanvas, event_pump: EventPump, interval: Option<Duration>) -> Self {
        let pull_timer = match interval {
            Some(i) => Some(PullTimer::new(i)),
            None => None,
        };
        Dispatcher {
            canvas,
            event_pump,
            pull_timer,
        }
    }

    pub fn run<Model>(
        &mut self,
        initialize_model: fn() -> Model,
        update_model: fn(action: Action, model: &Model) -> Option<Model>,
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
                    Action::None => {}
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

    fn next_actions(self: &mut Self) -> Vec<Action> {
        // Pump is required to enable all keyboard notifications (?) and also provides quit notification.
        for event in self.event_pump.wait_timeout_iter(1) {
            if let Event::Quit { .. } = event {
                return vec![Action::Quit];
            }
        }

        if let Some(pt) = &self.pull_timer {
            if pt.is_elapsed() {
                self.pull_timer = Some(pt.make_next());

                // Get key press without delay.
                let actions: Vec<Action> = self
                    .event_pump
                    .keyboard_state()
                    .pressed_scancodes()
                    .filter_map(Keycode::from_scancode)
                    .filter_map(Dispatcher::extract_action)
                    .collect();

                if actions.is_empty() {
                    return vec![Action::Timer];
                }

                return actions;
            }
        }

        return vec![Action::None];
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
