use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::ticker;
use crate::ticker::TickDuration;
use crate::executor::ExtWaker;

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

    // Registration places the deadline & its task_id onto a `BinaryHeap`, and
    // then will attempt to schedule it (via COMPARE0) if it's earlier than
    // the current deadline.
    fn register(&self, task_id: usize) {
        let new_deadline = self.end_time.ticks();
        critical_section::with(|cs| {
            let mut rm_deadlines = ticker::WAKE_DEADLINES.borrow_ref_mut(cs);
            let is_earliest = if let Some((next_deadline, _)) = rm_deadlines.peek() {
                new_deadline < *next_deadline
            } else {
                true
            };
            if rm_deadlines.push((new_deadline, task_id)).is_err() {
                // Dropping a deadline in this system can be Very Bad:
                //  - In the LED task, the LED will stop updating, but may come
                //    back to life on a button press...
                //  - In a button task, it will never wake again
                // `panic` to raise awareness of the issue during development
                panic!("Deadline dropped for task {}!", task_id);
            }
            // schedule now if its the earliest
            if is_earliest {
                ticker::schedule_wakeup(rm_deadlines, ticker::TICKER.rtc.borrow_ref_mut(cs));
            }
        });
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            TimerState::Init => {
                self.register(cx.waker().task_id());
                self.state = TimerState::Wait;
                Poll::Pending
            }
            TimerState::Wait => {
                if ticker::Ticker::now() >= self.end_time {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

pub async fn delay(duration: TickDuration) {
    Timer::new(duration).await;
}
