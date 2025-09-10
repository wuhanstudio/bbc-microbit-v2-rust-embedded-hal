pub mod ticker;

pub struct Timer {
    end_time: ticker::TickInstant,
}

impl Timer {
    pub fn new(duration: ticker::TickDuration) -> Self {
        Self {
            end_time: ticker::Ticker::now() + duration,
        }
    }

    pub fn is_ready(&self) -> bool {
        ticker::Ticker::now() >= self.end_time
    }
}
