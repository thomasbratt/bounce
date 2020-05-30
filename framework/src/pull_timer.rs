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

    pub fn is_elapsed(&self) -> bool {
        Instant::now() >= self.fire_at
    }

    pub fn make_next(&self) -> PullTimer {
        PullTimer {
            fire_at: self.fire_at + self.interval,
            interval: self.interval,
        }
    }
}
