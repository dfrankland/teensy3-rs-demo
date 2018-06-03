#![no_std]
#![no_main]

extern crate teensy3;
extern crate mk20d7_hal;
extern crate embedded_hal;
extern crate cortex_m;

use teensy3::bindings;
use teensy3::serial::Serial;
use mk20d7_hal::prelude::*;
use mk20d7_hal::mk20d7;

#[no_mangle]
pub unsafe extern fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = mk20d7::Peripherals::take().unwrap();

    let watchdog = mk20d7_hal::wdog::Watchdog::new(&*dp.WDOG);
    watchdog.disable();
    let oscillator = mk20d7_hal::osc::Oscillator::new(&*dp.OSC);
    oscillator.enable();
    oscillator.set_capacitance(10);
    let mut sim = mk20d7_hal::sim::SystemIntegrationModule::new(&*dp.SIM);
    sim.set_dividers(1, 2, 3);
    let mut delay = mk20d7_hal::delay::Delay::new(cp.SYST, &sim);

    // Wait for th oscillator frequency to stabilize
    bindings::delay(200);
    delay.delay_ms(200_u16);

    let ser = Serial {};

    // Watchdog Disabling
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

    // Oscillator enabling to 10 pF
    if oscillator.is_enabled() {
        send(&ser, "Oscillator is already enabled.\n\r").unwrap();
    } else {
        send(&ser, "Oscillator is disabled; enabling it now.\n\r").unwrap();
        oscillator.enable();
    }
    if oscillator.get_capacitance() == 10 {
        send(&ser, "Oscillator is already set to 10.\n\r").unwrap();
    } else {
        send(&ser, "Oscillator is not set to 10, setting to 10 now.\n\r").unwrap();
        oscillator.set_capacitance(10);
        if oscillator.get_capacitance() == 10 {
            send(&ser, "Oscillator is now set to 10.\n\r").unwrap();
        } else {
            send(&ser, "Oscillator is still not set to 10!\n\r").unwrap();
        }
    }

    // System Integration Module
    if sim.get_dividers() == (1, 2, 3) {
        send(&ser, "System Integration Module is already set to core 1, bus 2, and flash 3.\n\r").unwrap();
    } else {
        send(&ser, "System Integration Module is not set to core 1, bus 2, and flash 3; setting them now.\n\r").unwrap();
        // sim.set_dividers(1, 2, 3);
        if sim.get_dividers() == (1, 2, 3) {
            send(&ser, "System Integration Module is now set to core 1, bus 2, and flash 3.\n\r").unwrap();
        } else {
            send(&ser, "System Integration Module is still not set to core 1, bus 2, and flash 3.\n\r").unwrap();
        }
    }

    let portc = (dp.PTC, dp.PORTC).split(&dp.SIM.scgc5);
    let mut ptc5 = portc.ptc5.into_push_pull_output();

    loop {
        // Show we are alive
        alive(&mut ptc5, &mut delay);

        // If the serial write fails, we will halt (no more alive blinks)
        send(&ser, "Hello Teensy Rusty World!\n\r").unwrap();

        // Don't spam the console
        delay.delay_ms(200_u16);
    }
}

/// Blink the light twice to know we're alive
pub fn alive<P: embedded_hal::digital::OutputPin, D: embedded_hal::blocking::delay::DelayMs<u16>>(pin: &mut P, delay: &mut D) {
    for _ in 0..2 {
        pin.set_low();
        delay.delay_ms(200_u16);
        pin.set_high();
        delay.delay_ms(200_u16);
        pin.set_low();
        delay.delay_ms(200_u16);
    }
}

/// Send a message over the USB Serial port
pub fn send(ser: &Serial, msg: &str) -> Result<(),()> {
    ser.write_bytes(msg.as_bytes())
}
