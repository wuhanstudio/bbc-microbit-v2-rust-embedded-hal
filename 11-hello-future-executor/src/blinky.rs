#![allow(unused)]

use statig::prelude::*;

use fugit::ExtU64;
use crate::ticker::Ticker;
use crate::timer::Timer;

use rtt_target::rprintln;

// #[derive(Default)]
pub struct Blinky
{
    pub timer: Timer,
}

impl Default for Blinky {
    fn default() -> Self {
        Self {
            timer: Timer::new(1000.millis())
        }
    }
}

pub enum Event {
    TimerElapsed
}

#[state_machine(initial = "State::led_on()")]
impl Blinky {
    #[state(entry_action = "enter_led_on", exit_action = "exit_led_on")]
    fn led_on(event: &Event) -> Outcome<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_off()),
            _ => Super
        }
    }

    #[state(entry_action = "enter_led_off", exit_action = "exit_led_off")]
    fn led_off(event: &Event) -> Outcome<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_on()),
            _ => Super
        }
    }

    #[action]
    fn enter_led_on(&mut self) {
        rprintln!("LED ON");
        self.timer = Timer::new(1000.millis());
    }

    #[action]
    fn exit_led_on(&mut self) {
        rprintln!("Switching to LED Off");
    }

    #[action]
    fn enter_led_off(&mut self) {
        rprintln!("LED OFF");
        self.timer = Timer::new(1000.millis());
    }

    #[action]
    fn exit_led_off(&mut self) {
        rprintln!("Switching to LED On");
    }
}

pub fn blinky_poll(blinky_task: &mut InitializedStateMachine<Blinky>) {
    if blinky_task.timer.is_ready() {
        let time = Ticker::now();
        rprintln!("Blinky Event triggered at {} ticks, {} ms", time.ticks(), time.duration_since_epoch().to_millis());
        blinky_task.handle(&Event::TimerElapsed);
    }
}
