#![no_std]
#![no_main]

extern crate teensy3;
extern crate mk20d7_hal;
extern crate embedded_hal;

use teensy3::bindings;
use teensy3::serial::Serial;
use mk20d7_hal::prelude::*;
use mk20d7_hal::mk20d7;

#[no_mangle]
pub unsafe extern fn main() {
    let dp = mk20d7::Peripherals::take().unwrap();

    // Wait for th oscillator frequency to stabilize
    bindings::delay(200);

    let ser = Serial {};

    // Watchdog Disabling
    let watchdog = mk20d7_hal::wdog::Watchdog::new(&*dp.WDOG);
    if watchdog.is_enabled() {
        send(&ser, "Watchdog is enabled.\n\r").unwrap();

        if watchdog.allow_update() {
            send(&ser, "Watchdog disabling now.\n\r").unwrap();
            watchdog.disable();

            if watchdog.is_enabled() {
                send(&ser, "Watchdog is still enabled.\n\r").unwrap();
            } else {
                send(&ser, "Watchdog is now disabled.\n\r").unwrap();
            }
        } else {
            send(&ser, "Watchdog is not allowed to update.\n\r").unwrap();
        }
    } else {
        send(&ser, "Watchdog is already disabled.\n\r").unwrap();
    }

    let portc = (dp.PTC, dp.PORTC).split(&(&*dp.SIM).scgc5);
    let mut ptc5 = portc.ptc5.into_push_pull_output();

    loop {
        // Show we are alive
        alive(&mut ptc5);

        // If the serial write fails, we will halt (no more alive blinks)
        send(&ser, "Hello Teensy Rusty World!\n\r").unwrap();

        // Don't spam the console
        bindings::delay(1000);
    }
}

/// Blink the light twice to know we're alive
pub unsafe fn alive<P: embedded_hal::digital::OutputPin>(pin: &mut P) {
    for _ in 0..2 {
        pin.set_low();
        bindings::delay(200);
        pin.set_high();
        bindings::delay(200);
        pin.set_low();
        bindings::delay(200);
    }
}

/// Send a message over the USB Serial port
pub fn send(ser: &Serial, msg: &str) -> Result<(),()> {
    ser.write_bytes(msg.as_bytes())
}
