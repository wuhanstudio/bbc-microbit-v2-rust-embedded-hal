#![allow(unused)]

use statig::prelude::*;

use fugit::ExtU64;
use crate::ticker::Ticker;
use crate::timer::Timer;

use rtt_target::rprintln;
use embedded_hal::digital::{InputPin, OutputPin};

// #[derive(Default)]
pub struct Blinky<P: OutputPin, const N: usize>
{
    timer: Timer,
    rows: [P; N],
    cols: [P; N],
}

impl<P: OutputPin, const N: usize> Blinky<P, N> {
    pub fn new(rows: [P; N], cols: [P; N]) -> Self {
        Self {
            timer: Timer::new(1000.millis()),
            rows,
            cols,
        }
    }
}

pub enum Event {
    TimerElapsed
}

#[state_machine(initial = "State::led_on()")]
impl<P: OutputPin, const N: usize> Blinky<P, N> {
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
        for row in self.rows.iter_mut() {
            row.set_high().unwrap();
        }

        for col in self.cols.iter_mut() {
            col.set_high().unwrap();
        }
        self.timer = Timer::new(1000.millis());
    }

    #[action]
    fn exit_led_on(&mut self) {
        rprintln!("Switching to LED Off");
    }

    #[action]
    fn enter_led_off(&mut self) {
        rprintln!("LED OFF");
        for row in self.rows.iter_mut() {
            row.set_high().unwrap();
        }

        for col in self.cols.iter_mut() {
            col.set_low().unwrap();
        }
        self.timer = Timer::new(1000.millis());
    }

    #[action]
    fn exit_led_off(&mut self) {
        rprintln!("Switching to LED On");
    }
}

pub fn blinky_poll<P: OutputPin, const N: usize>(blinky_task: &mut InitializedStateMachine<Blinky<P, N>>) {
    if blinky_task.timer.is_ready() {
        let time = Ticker::now();
        rprintln!("Blinky Event triggered at {} ticks, {} ms", time.ticks(), time.duration_since_epoch().to_millis());
        blinky_task.handle(&Event::TimerElapsed);
    }
}
