// Import crate for the `is_high` trait on
use embedded_hal::digital::InputPin;
use rp2040_hal::{Timer, fugit::ExtU64, gpio::FunctionSioInput};

use crate::{DynamicPin, channel::Sender, scheduler::Scheduler};

const BUTTON_DEBOUNCE_MS: u64 = 100;

#[derive(Clone, Copy)]
pub enum ButtonDirection {
    Left,
    Right,
}

pub enum ButtonState<'a> {
    /// The button is not pressed.
    Idle,
    /// The button is pressed and will be debounced.
    /// This avoid the typical rapid repetition of pulses.
    Debounce(Scheduler<'a>),
}

/// A task to poll the state of the button, which is sent over a `Channel`.
pub struct ButtonTask<'a> {
    pin: DynamicPin<FunctionSioInput>,
    timer: &'a Timer,
    direction: ButtonDirection,
    state: ButtonState<'a>,
    sender: Sender<'a, ButtonDirection>,
}

impl<'a> ButtonTask<'a> {
    pub fn new(
        pin: DynamicPin<FunctionSioInput>,
        timer: &'a Timer,
        direction: ButtonDirection,
        sender: Sender<'a, ButtonDirection>,
    ) -> Self {
        Self {
            pin,
            timer,
            direction,
            state: ButtonState::Idle,
            sender,
        }
    }

    pub fn poll(&mut self) {
        match &self.state {
            ButtonState::Idle => {
                if self.pin.is_high().unwrap() {
                    self.sender.send(self.direction);
                    self.state = ButtonState::Debounce(Scheduler::new(
                        BUTTON_DEBOUNCE_MS.millis(),
                        self.timer,
                    ));
                }
            }
            ButtonState::Debounce(timer) => {
                if timer.is_ready() && self.pin.is_low().unwrap() {
                    self.state = ButtonState::Idle;
                }
            }
        }
    }
}
