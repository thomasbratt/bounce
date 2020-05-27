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

        'running: loop {
            match self.determine_next_action() {
                Some(Action::Quit) => {
                    break 'running;
                }
                Some(action) => {
                    if let Some(new_model) = update_model(action, &model) {
                        model = new_model;
                        render(&mut self.canvas, &model);
                    }
                    // Reset the timer *after* update, in case of long running model updates.
                    if action == Action::Timer {
                        if let Some(pt) = &self.pull_timer {
                            pt.reset();
                        }
                    }
                }
                None => {}
            }
        }

        cleanup_model(model);
    }

    fn determine_next_action(self: &mut Self) -> Option<Action> {
        // Pump is required to enable all keyboard notifications (?) and also provides quit notification.
        for event in self.event_pump.wait_timeout_iter(10) {
            if let Event::Quit { .. } = event {
                Some(Action::Quit)
            };
        }

        // Get key press without delay.
        let actions: Vec<Action> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .filter_map(self::extract_action)
            .collect();
        if !actions.is_empty() {
            // TODO: map multiple key press events to single action
            Some(actions.first())
        }

        // Produce timer notifications here as well, for ease of consumption by the caller.
        if self.pull_timer.check() {
            Some(Action::Timer)
        }

        None
    }

    // TODO: make this static
    fn extract_action(self: &Self, keycode: Keycode) -> Action {
        match keycode {
            Keycode::Left | Keycode::A => Action::Left,
            Keycode::Right | Keycode::D => Action::Right,
            Keycode::Escape => Action::Quit,
            _ => Action::None,
        }
    }
}
