use crate::ticker;

pub struct Timer<'a> {
    end_time: ticker::TickInstant,
    ticker: &'a ticker::Ticker,
}

impl<'a> Timer<'a> {
    pub fn new(duration: ticker::TickDuration, ticker: &'a ticker::Ticker) -> Self {
        Self {
            end_time: ticker.now() + duration,
            ticker,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ticker.now() >= self.end_time
    }
}
