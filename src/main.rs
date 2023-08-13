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

use crate::bsp::driver::GPIO;

// Real entrypoint
mod boot;

mod bsp;
mod console;
mod driver;
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
    if let Err(x) = bsp::driver::init() {
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
    info!("Booting on: {}", bsp::board_name());

    info!(
        "Architectural timer resolution: {} ns",
        time::resolution().as_nanos()
    );

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();

    // Test a failing timer case.
    time::spin_for(Duration::from_nanos(1));

    GPIO.pin_42_config_output();

    loop {
        info!("Turning ON the LED");
        GPIO.pin_42_set();

        info!("Waiting for 1 second");
        time::spin_for(Duration::from_secs(1));

        info!("Turning OFF the LED");
        GPIO.pin_42_clr();

        info!("Waiting for 1 second");
        time::spin_for(Duration::from_secs(1));
    }
}
