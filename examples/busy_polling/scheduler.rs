use rp2040_hal::{
    Timer,
    fugit::{Duration, Instant},
};

// Ticks last 1 us and occur at 1 MHz.
type TickInstant = Instant<u64, 1, 1_000_000>;
type TickDuration = Duration<u64, 1, 1_000_000>;

/// A rudimentary task scheduler that calculates the end time of an event.
pub struct Scheduler<'a> {
    end_time: TickInstant,
    timer: &'a Timer,
}

impl<'a> Scheduler<'a> {
    pub fn new(duration: TickDuration, timer: &'a Timer) -> Self {
        Self {
            end_time: timer.get_counter() + duration,
            timer,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.timer.get_counter() >= self.end_time
    }
}
