#![no_std]
#![no_main]

#[macro_use(entry)]
extern crate cortex_m_rt;
extern crate teensy3;
extern crate mk20d7_hal;
extern crate embedded_hal;
extern crate cortex_m;

use mk20d7_hal::prelude::*;
use mk20d7_hal::mk20d7;

entry!(main);

fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = mk20d7::Peripherals::take().unwrap();

    let watchdog = mk20d7_hal::wdog::Watchdog::new(&*dp.WDOG);
    watchdog.disable();

    let oscillator = mk20d7_hal::osc::Oscillator::new(&*dp.OSC);
    oscillator.enable();
    oscillator.set_capacitance(10);

    let mut sim = mk20d7_hal::sim::SystemIntegrationModule::new(&*dp.SIM);
    sim.set_dividers(1, 2, 3);

    let mut mcg = mk20d7_hal::mcg::MultipurposeClockGenerator::new(&*dp.MCG);
    match mcg.clock_mode() {
        mk20d7_hal::mcg::ClockMode::Fei(fei) => {
            let fbe: mk20d7_hal::mcg::Fbe = fei.into();
            let pbe: mk20d7_hal::mcg::Pbe = fbe.into();
            let _pee: mk20d7_hal::mcg::Pee = pbe.into();
        },
        mk20d7_hal::mcg::ClockMode::Fbe(fbe) => {
            let pbe: mk20d7_hal::mcg::Pbe = fbe.into();
            let _pee: mk20d7_hal::mcg::Pee = pbe.into();
        },
        mk20d7_hal::mcg::ClockMode::Pbe(pbe) => {
            let _pee: mk20d7_hal::mcg::Pee = pbe.into();
        },
        mk20d7_hal::mcg::ClockMode::Pee(_) => {},
        _ => panic!("Clock mode is unsupported!"),
    };

    let mut delay = mk20d7_hal::delay::Delay::new(cp.SYST, &sim);

    let mut gpioc = (dp.PTC, dp.PORTC).split(&dp.SIM.scgc5);
    let mut ptc5 = gpioc.ptc5.into_push_pull_output(&mut gpioc.pcr, &mut gpioc.pddr);

    loop {
        alive(&mut ptc5, &mut delay);
    }
}

/// Blink the light twice to know we're alive
fn alive<P: embedded_hal::digital::OutputPin, D: embedded_hal::blocking::delay::DelayMs<u16>>(pin: &mut P, delay: &mut D) {
    for _ in 0..2 {
        pin.set_low();
        delay.delay_ms(200_u16);
        pin.set_high();
        delay.delay_ms(200_u16);
        pin.set_low();
        delay.delay_ms(200_u16);
    }
}
