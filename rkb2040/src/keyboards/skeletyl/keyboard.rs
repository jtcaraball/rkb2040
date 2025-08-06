use rkb2040_lib::{
    impl_matrix_primary_keyboard, impl_matrix_secondary_keyboard, impl_trrs_rx, impl_trrs_tx,
};

use crate::keyboards::skeletyl::keymap::SkeletylSM;

use {
    rp2040_hal::gpio::{FunctionNull, PullDown, bank0::Gpio1},
    rp2040_hal::pac::{PIO0, RESETS},
    rp2040_hal::pio::PIOExt,
};

const DEBOUNCE_DELAY: u64 = 50_000;

impl_trrs_rx!(PIO0, RESETS, Gpio1, FunctionNull, PullDown);
impl_trrs_tx!(PIO0, RESETS, Gpio1, FunctionNull, PullDown);
impl_matrix_primary_keyboard!(
    DEBOUNCE_DELAY,
    TrrsRx,
    SkeletylSM,
    matrix: (
        cols: (8, 7, 6, 21, 28),
        rows: (26, 5, 4, 9)
    ),
    keys: (
        (4, 0), (3, 0), (2, 0), (1, 0), (0, 0),
        (4, 1), (3, 1), (2, 1), (1, 1), (0, 1),
        (4, 2), (3, 2), (2, 2), (1, 2), (0, 2),
        (2, 3), (1, 3), (4, 3)
    )
);
impl_matrix_secondary_keyboard!(
    DEBOUNCE_DELAY,
    TrrsTx,
    matrix: (
        cols: (8, 7, 6, 21, 28),
        rows: (26, 5, 4, 9)
    ),
    keys: (
        (0, 0), (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1), (1, 1), (2, 1), (3, 1), (4, 1),
        (0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
        (4, 3), (1, 3), (2, 3)
    )
);
