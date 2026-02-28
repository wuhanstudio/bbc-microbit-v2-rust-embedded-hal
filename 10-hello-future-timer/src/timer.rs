use core::pin::Pin;
use core::task::Poll;
use core::task::Context;

use crate::ticker;
use crate::ticker::TickDuration;

enum TimerState {
    Init,
    Wait,
}

pub struct Timer {
    end_time: ticker::TickInstant,
    state: TimerState
}

impl Timer {
    pub fn new(duration: ticker::TickDuration) -> Self {
        Self {
            end_time: ticker::Ticker::now() + duration,
            state: TimerState::Init,
        }
    }

    pub fn is_ready(&self) -> bool {
        ticker::Ticker::now() >= self.end_time
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            TimerState::Init => {
                self.state = TimerState::Wait;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            TimerState::Wait => {
                if ticker::Ticker::now() >= self.end_time {
                    Poll::Ready(())
                } else {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
    }
}

pub async fn delay(duration: TickDuration) {
    Timer::new(duration).await;
}
