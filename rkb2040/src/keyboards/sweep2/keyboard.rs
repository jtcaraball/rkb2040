use crate::keyboards::sweep2::keymap::Sweep2SM;
use rkb2040_lib::{
    impl_direct_wire_primary_keyboard, impl_direct_wire_secondary_keyboard, impl_trrs_rx,
    impl_trrs_tx,
};

use {
    rp2040_hal::gpio::{FunctionNull, PullDown, bank0::Gpio1},
    rp2040_hal::pac::{PIO0, RESETS},
    rp2040_hal::pio::PIOExt,
};

const DEBOUNCE_DELAY: u64 = 50_000;

impl_trrs_tx!(PIO0, RESETS, Gpio1, FunctionNull, PullDown);
impl_trrs_rx!(PIO0, RESETS, Gpio1, FunctionNull, PullDown);
impl_direct_wire_primary_keyboard!(
    DEBOUNCE_DELAY,
    TrrsRx,   // Usb serial reciever.
    Sweep2SM, // Keymaps.
    7, 26, 27, 28, 29, // Top row pins.
    22, 20, 23, 21, 0, // Middle row pins.
    2, 3, 4, 5, 6, // Bottom row pins.
    8, 9 // Thumb cluster pins.
);
impl_direct_wire_secondary_keyboard!(
    DEBOUNCE_DELAY,
    TrrsTx, // Usb serial transmiter.
    29, 28, 27, 26, 7, // Top row pins.
    0, 21, 23, 20, 22, // Middle row pins.
    6, 5, 4, 3, 2, // Bottom row pins.
    9, 8 // Thumb cluster pins.
);
