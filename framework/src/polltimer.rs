use std::time::{Duration, Instant};

pub struct PollTimer {
    fire_at: Instant,
    interval: Duration,
}

impl PollTimer {
    pub fn new(interval: Duration) -> Self {
        PollTimer {
            fire_at: Instant::now() + interval,
            interval,
        }
    }

    pub fn is_elapsed(&self) -> bool {
        Instant::now() >= self.fire_at
    }

    pub fn remaining(&self) -> Duration {
        let now = Instant::now();
        if now > self.fire_at {
            Duration::from_millis(0)
        } else {
            self.fire_at - now
        }
    }

    pub fn make_next(&self) -> PollTimer {
        PollTimer {
            fire_at: self.fire_at + self.interval,
            interval: self.interval,
        }
    }
}
