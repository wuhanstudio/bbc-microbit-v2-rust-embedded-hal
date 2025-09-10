use nrf52833_hal::Rtc;
use nrf52833_pac::RTC0;

use fugit::{Instant, Duration};

pub type TickInstant = Instant<u64, 1, 32_768>; // 32.768 kHz clock
pub type TickDuration = Duration<u64, 1, 32_768>; // 32.768 kHz clock

pub struct Ticker {
    rtc: Rtc<RTC0>,
}

impl Ticker {
    pub fn new(rtc0: RTC0) -> Self {
        let rtc = Rtc::new(rtc0, 0).unwrap();
        rtc.enable_counter();
        Ticker { rtc }
    }

    pub fn now(&self) -> TickInstant {
        TickInstant::from_ticks(self.rtc.get_counter() as u64)
    }
}
