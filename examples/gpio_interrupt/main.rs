#![no_std]
#![no_main]

use core::{cell::RefCell, panic::PanicInfo};
use cortex_m_rt::entry;
use critical_section::Mutex;
use embedded_hal::digital::StatefulOutputPin;
use rp2040_hal::pac::interrupt;
use rp2040_hal::{
    Sio, Watchdog,
    clocks::init_clocks_and_plls,
    gpio::{
        FunctionSioInput, FunctionSioOutput,
        Interrupt::EdgeHigh,
        Pin, Pins, PullDown, PullNone,
        bank0::{Gpio0, Gpio27},
    },
    pac::{Interrupt, NVIC, Peripherals},
};
/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;
const XTAL_FREQ_HZ: u32 = 12_000_000;

type LedPin = Pin<Gpio0, FunctionSioOutput, PullNone>;
type ButtonPin = Pin<Gpio27, FunctionSioInput, PullDown>;
type LedAndButton = (LedPin, ButtonPin);
static GLOBAL_PINS: Mutex<RefCell<Option<LedAndButton>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut pac = Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let _clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    );
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let led: LedPin = pins.gpio0.reconfigure();
    let button: ButtonPin = pins.gpio27.reconfigure();
    button.set_interrupt_enabled(EdgeHigh, true);

    critical_section::with(|cs| GLOBAL_PINS.borrow(cs).replace(Some((led, button))));
    unsafe {
        NVIC::unmask(Interrupt::IO_IRQ_BANK0);
    }

    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn IO_IRQ_BANK0() {
    static mut LED_AND_BUTTON: Option<LedAndButton> = None;

    if LED_AND_BUTTON.is_none() {
        critical_section::with(|cs| {
            *LED_AND_BUTTON = GLOBAL_PINS.borrow(cs).take();
        })
    }

    if let Some(led_and_button) = LED_AND_BUTTON {
        let (led, button) = led_and_button;
        if button.interrupt_status(EdgeHigh) {
            let _ = led.toggle();
            button.clear_interrupt(EdgeHigh);
        }
    }
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}
