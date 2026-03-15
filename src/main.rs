#![no_std]
#![no_main]

mod button;
mod channel;
mod led;
mod scheduler;

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use rp2040_hal::{
    Sio, Timer, Watchdog,
    clocks::init_clocks_and_plls,
    gpio::{
        DynPinId, FunctionNull, FunctionSio, FunctionSioOutput, Pin, PinId, Pins, PullDown,
        PullNone, SioOutput, ValidFunction,
    },
    pac::Peripherals,
};

use crate::{
    button::{ButtonDirection, ButtonTask},
    channel::Channel,
    led::LedTask,
};

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;
/// The frequency of the low-power oscillator.
const XTAL_FREQ_HZ: u32 = 12_000_000;
const N_LEDS: usize = 5;
type DynamicPin<F> = Pin<DynPinId, F, PullNone>;
type Leds = [DynamicPin<FunctionSioOutput>; N_LEDS];

#[entry]
fn main() -> ! {
    // Take ownership of the peripherals singleton.
    let peri = Peripherals::take().unwrap();
    let (pins, timer) = get_pins_and_timer(peri);
    let channel = Channel::<ButtonDirection>::new();
    let (mut led_task, [mut task_left, mut task_right]) =
        get_leds_and_buttons(pins, &timer, &channel);

    loop {
        led_task.poll();
        task_left.poll();
        task_right.poll();
    }
}

fn get_pins_and_timer(mut peri: Peripherals) -> (Pins, Timer) {
    // https://github.com/rp-rs/rp-hal/blob/c09914ea54d44e46c10e117917e1e7837efb66d9/rp2040-hal-examples/src/bin/gpio_irq_example.rs#L109-L110
    let sio = Sio::new(peri.SIO);
    let pins = Pins::new(
        peri.IO_BANK0,
        peri.PADS_BANK0,
        sio.gpio_bank0,
        &mut peri.RESETS,
    );
    // To setup a timer, we need the hardware clocks and watchdog driver.
    // We take ownership of the pac resources, moving them by value.
    let mut watchdog = Watchdog::new(peri.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
        peri.XOSC,
        peri.CLOCKS,
        peri.PLL_SYS,
        peri.PLL_USB,
        &mut peri.RESETS,
        &mut watchdog,
    )
    .unwrap();
    let timer = Timer::new(peri.TIMER, &mut peri.RESETS, &clocks);

    (pins, timer)
}

fn type_erase_output<I>(pin: Pin<I, FunctionNull, PullDown>) -> DynamicPin<FunctionSioOutput>
where
    I: PinId + ValidFunction<FunctionSio<SioOutput>>,
{
    pin.into_push_pull_output().into_pull_type().into_dyn_pin()
}

fn get_leds_and_buttons<'a>(
    pins: Pins,
    timer: &'a Timer,
    channel: &'a Channel<ButtonDirection>,
) -> (LedTask<'a>, [ButtonTask<'a>; 2]) {
    // Every pin is a unique type. To put them in an array, we must do type-erasure.
    // The pins are transformed to dynamically typed objects that carry their pin
    // and bank numbers at runtime rather than at compile-time.
    let leds = [
        type_erase_output(pins.gpio0),
        type_erase_output(pins.gpio2),
        type_erase_output(pins.gpio4),
        type_erase_output(pins.gpio6),
        type_erase_output(pins.gpio8),
    ];
    let led_task = LedTask::new(leds, timer, channel.get_receiver());

    // Buttons are wired to be pulled down to ground
    let left = pins.gpio27.into_floating_input().into_dyn_pin();
    let right = pins.gpio14.into_floating_input().into_dyn_pin();

    // The button and led tasks will query the channel to push and pull requests.
    // TODO: create buttontask, channel, ledtask
    let left_task = ButtonTask::new(left, timer, ButtonDirection::Left, channel.get_sender());
    let right_task = ButtonTask::new(right, timer, ButtonDirection::Right, channel.get_sender());
    (led_task, [left_task, right_task])
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}
