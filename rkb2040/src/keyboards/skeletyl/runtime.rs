use core::ops::Div;
use embedded_hal::digital::InputPin;
// Required by rkb2040_lib init macros.
use cortex_m::prelude::_embedded_hal_timer_CountDown;
use rp2040_hal::{Clock, fugit::ExtU32};

use crate::keyboards::skeletyl::{
    bsp,
    keyboard::{
        PrimaryKB, SecondaryKB, TrrsRx, TrrsTx, UartRx, UartTx, init_primary_pins,
        init_secondary_pins,
    },
    keymap::KEYMAP,
};
use rkb2040_lib::{
    bootloader::bootloader_double_tap_reset, hardware_init, keymap::KeymapSM, run_primary_keyboard,
    run_secondary_keyboard, usb_init,
};

const SCAN_FREQ_MILLIS: u32 = 1;
const PRODUCT_NAME: &str = "sweep2";
const SERIAL: &str = "swp2.1";

pub fn run() -> ! {
    hardware_init!((bsp, SCAN_FREQ_MILLIS) -> pac, clocks, timer, pins, scan_cd);
    // Enter bootloader mode if the board is reset twice withing 500 ms and turn on led connected
    // to gpio17 to indicate it.
    bootloader_double_tap_reset(&mut timer, 500, 1 << 17);
    // Primary board detection.
    if pins.vbus_detect.into_pull_up_input().is_high().unwrap() {
        usb_init!((bsp, pac, clocks, timer, PRODUCT_NAME, SERIAL) -> hid_dev, usb_dev, tick_cd);
        let sm = KeymapSM::new(KEYMAP);
        let rx = TrrsRx::new(UartRx::new(
            pins.gpio1,
            clocks.system_clock.freq().div(16),
            clocks.system_clock.freq(),
            pac.PIO0,
            &mut pac.RESETS,
        ));
        let (matrix, keys) = init_primary_pins!(pins);
        let mut kb = PrimaryKB::new(matrix, keys, rx, sm);
        run_primary_keyboard!(kb, hid_dev, usb_dev, timer, scan_cd, tick_cd);
    } else {
        let tx = TrrsTx::new(UartTx::new(
            pins.gpio1,
            clocks.system_clock.freq().div(16),
            clocks.system_clock.freq(),
            pac.PIO0,
            &mut pac.RESETS,
        ));
        let (matrix, keys) = init_secondary_pins!(pins);
        let mut kb = SecondaryKB::new(matrix, keys, tx);
        run_secondary_keyboard!(kb, timer, scan_cd);
    }
}
