#![allow(clippy::upper_case_acronyms)]
#![feature(asm_const)]
#![feature(const_option)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(unchecked_math)]
#![no_main]
#![no_std]
#![feature(naked_functions)]

use embedded_hal::{digital::v2::OutputPin, serial::Write};

use crate::bitbang::uart::*;
use crate::{
    gpio::pin::{Gpio0, Gpio42, Pin, PinId, PushPullOutput},
    time::spin_for,
};

// Real entrypoint
mod boot;

mod bitbang;
mod bsp;
mod console;
mod driver;
mod gpio;
mod panic_wait;
mod print;
mod synchronization;
mod time;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
pub unsafe fn kernel_init() -> ! {
    // Initialize the BSP driver subsystem.
    if let Err(x) = bsp::init() {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    // Initialize all device drivers.
    driver::driver_manager().init_drivers();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use core::time::Duration;

    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!(
        "Architectural timer resolution: {} ns",
        time::resolution().as_nanos()
    );

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();

    // Test a failing timer case.
    time::spin_for(Duration::from_nanos(1));

    let led_pin: Pin<Gpio42, <Gpio42 as PinId>::Reset> = unsafe { Pin::new() };
    let mut led_pin: Pin<_, PushPullOutput> = led_pin.into_mode();

    let uart_pin: Pin<Gpio0, <Gpio0 as PinId>::Reset> = unsafe { Pin::new() };
    let uart_pin: Pin<_, PushPullOutput> = uart_pin.into_mode();

    let mut uart = SoftUartTransmitter::<Pin<_, PushPullOutput>>::new(
        uart_pin,
        9600,
        StopBitsOption::One,
        ParityMode::Even,
    );

    loop {
        led_pin.set_high().unwrap();
        spin_for(Duration::from_millis(200));

        uart.write(0x31).unwrap();
        uart.write(0x33).unwrap();

        led_pin.set_low().unwrap();
        spin_for(Duration::from_millis(1000));
    }
}
