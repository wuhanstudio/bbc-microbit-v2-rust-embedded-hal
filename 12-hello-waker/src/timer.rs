use core::pin::Pin;
use core::task::Poll;
use core::task::Context;

use crate::ticker::Ticker;
use crate::ticker::TickDuration;
use crate::ticker::TickInstant;
use crate::ticker::TimerEntry;
use crate::ticker::TIMERS;

pub struct Timer {
    deadline: TickInstant,
    registered: bool,
}

impl Timer {
    pub fn new(dur: TickDuration) -> Self {
        Self {
            deadline: Ticker::now() + dur,
            registered: false,
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if Ticker::now() >= self.deadline {
            return Poll::Ready(());
        }

        let mut next_deadline = None;
        if !self.registered {
            critical_section::with(|cs| {
                TIMERS.borrow_ref_mut(cs).push(TimerEntry {
                    deadline: self.deadline,
                    waker: cx.waker().clone(),
                })
                .ok();

                // Find earliest deadline
                next_deadline = TIMERS.borrow_ref_mut(cs).iter().map(|t| t.deadline).min();
            });

            if let Some(next) = next_deadline {
                Ticker::set_compare(next);
            }
            self.registered = true;
        }

        Poll::Pending
    }
}

pub async fn delay(duration: TickDuration) {
    Timer::new(duration).await;
}
