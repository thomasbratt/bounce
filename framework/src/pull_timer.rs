use std::time::{Duration, Instant};

pub struct PullTimer {
    fire_at: Instant,
    interval: Duration,
}

impl PullTimer {
    pub fn new(interval: Duration) -> Self {
        PullTimer {
            fire_at: Instant::now() + interval,
            interval,
        }
    }

    pub fn check_and_reset(self: &mut Self) -> bool {
        let result = self.check();
        if result {
            self.reset();
        }
        result
    }

    pub fn check(self: &Self) -> bool {
        Instant::now() >= self.fire_at
    }

    pub fn reset(self: &mut Self) {
        self.fire_at = Instant::now() + self.interval;
    }
}
