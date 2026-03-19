use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use rp2040_hal::{Timer, fugit::ExtU64};

use crate::{Leds, N_LEDS, button::ButtonDirection, channel::Receiver, scheduler::Scheduler};

const LED_SWITCH_MS: u64 = 500;

enum LedState<'a> {
    Toggle,
    Wait(Scheduler<'a>),
}

/// A task to blink the LED and/or switch to the next one.
pub struct LedTask<'a> {
    leds: Leds,
    idx: usize,
    timer: &'a Timer,
    state: LedState<'a>,
    receiver: Receiver<'a, ButtonDirection>,
}

impl<'a> LedTask<'a> {
    pub fn new(leds: Leds, timer: &'a Timer, receiver: Receiver<'a, ButtonDirection>) -> Self {
        Self {
            leds,
            idx: 0,
            timer,
            state: LedState::Toggle,
            receiver,
        }
    }

    pub fn poll(&mut self) {
        match &self.state {
            LedState::Toggle => {
                self.leds[self.idx].toggle().ok();
                self.state = LedState::Wait(Scheduler::new(LED_SWITCH_MS.millis(), self.timer));
            }
            LedState::Wait(scheduler) => {
                if scheduler.is_ready() {
                    self.state = LedState::Toggle;
                }
                if let Some(direction) = self.receiver.receive() {
                    self.shift(direction);
                }
            }
        }
    }

    fn shift(&mut self, direction: ButtonDirection) {
        self.leds[self.idx].set_low().ok();
        self.idx = match direction {
            ButtonDirection::Left => self.idx.checked_sub(1).unwrap_or(N_LEDS - 1),
            ButtonDirection::Right => (self.idx + 1) % N_LEDS,
        };
        self.leds[self.idx].set_high().ok();
    }
}
