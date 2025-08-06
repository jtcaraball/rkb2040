#[macro_export]
macro_rules! impl_direct_wire_primary_keyboard {
    ($delay:tt, $rx:ty, $sm:ty, $($id:tt),*) => {
        paste::paste! {
            pub struct PrimaryKB {
                pub keys: (
                    $($crate::pin::DirectKeyPin<rp2040_hal::gpio::bank0::[< Gpio $id >], $delay>,)*
                ),
                pub rx: $rx,
                pub sm: $sm,
            }

            impl PrimaryKB {
                #[must_use]
                pub fn new(
                    keys: (
                        $($crate::pin::DirectKeyPin<rp2040_hal::gpio::bank0::[< Gpio $id >], $delay>,)*
                    ),
                    rx: $rx,
                    sm: $sm,
                ) -> Self {
                    Self { keys, rx, sm }
                }

                pub fn scan(&mut self, timer: rp2040_hal::Timer) {
                    self.sm.begin_scan();
                    rkb2040_proc::direct_pin_rx_check!(self, timer, $($id,)*);
                    self.sm.finish_scan();
                }
            }

            macro_rules! init_primary_pins {
                ($_pins:ident) => {
                    (
                        $($crate::pin::DirectKeyPin::new($_pins.[< gpio $id >].into_pull_up_input()),)*
                    )
                }
            }
            pub(crate) use init_primary_pins;
        }
    }
}

#[macro_export]
macro_rules! impl_direct_wire_secondary_keyboard {
    ($delay:tt, $tx:ty, $($id:tt),*) => {
        paste::paste! {
            pub struct SecondaryKB {
                pub keys: (
                    $($crate::pin::DirectKeyPin<rp2040_hal::gpio::bank0::[< Gpio $id >], $delay>,)*
                ),
                pub tx: $tx,
            }

            impl SecondaryKB {
                #[must_use]
                pub fn new(
                    keys: (
                        $($crate::pin::DirectKeyPin<rp2040_hal::gpio::bank0::[< Gpio $id >], $delay>,)*
                    ),
                    tx: $tx,
                ) -> Self {
                    Self { keys, tx }
                }

                pub fn scan(&mut self, timer: rp2040_hal::Timer) {
                    rkb2040_proc::direct_pin_check!(self, timer, $($id,)*);
                }
            }

            macro_rules! init_secondary_pins {
                ($_pins:ident) => {
                    (
                        $($crate::pin::DirectKeyPin::new($_pins.[< gpio $id >].into_pull_up_input()),)*
                    )
                }
            }

            pub(crate) use init_secondary_pins;
        }
    }
}

#[macro_export]
macro_rules! impl_matrix_primary_keyboard {
    ($delay:tt, $rx:ty, $sm:ty, $(matrix: (cols: ($($cid:tt),*), rows: ($($rid:tt),*))),*) => {
        paste::paste! {
            pub struct PrimaryKB {
                pub matrices: (
                    $((
                        ($(Option<$crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $cid >]>>),*,),
                        ($($crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $rid >]>),*,),
                    ),),*
                ),
                pub keys: rkb2040_proc::matrix_to_states!($crate::pin::PinState<$delay>, $((($($cid),*), ($($rid),*))),*),
                pub rx: $rx,
                pub sm: $sm,
            }

            impl PrimaryKB {
                #[must_use]
                pub fn new(
                    matrices: (
                        $((
                            ($(Option<$crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $cid >]>>),*,),
                            ($($crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $rid >]>),*,),
                        ),),*
                    ),
                    keys: rkb2040_proc::matrix_to_states!($crate::pin::PinState<$delay>, $((($($cid),*), ($($rid),*))),*),
                    rx: $rx,
                    sm: $sm,
                ) -> Self {
                    Self { matrices, keys, rx, sm }
                }

                pub fn scan(&mut self, timer: rp2040_hal::Timer) {
                    self.sm.begin_scan();
                    rkb2040_proc::matrix_pin_rx_check!(self, timer, $((($($cid),*), ($($rid),*))),*);
                    self.sm.finish_scan();
                }
            }

            macro_rules! init_primary_pins {
                ($_pins:ident) => {(
                    (
                        $((
                            ($(Some($_pins.[< gpio $cid >].into_pull_up_input())),*,),
                            ($($_pins.[< gpio $rid >].into_pull_up_input()),*,),
                        ),),*
                    ),
                    rkb2040_proc::matrix_to_states_init!($crate::pin::PinState, $((($($cid),*), ($($rid),*))),*)
                )}
            }
            pub(crate) use init_primary_pins;
        }
    };
}

#[macro_export]
macro_rules! impl_matrix_secondary_keyboard {
    ($delay:tt, $tx:ty, $sm:ty, $(matrix: (cols: ($($cid:tt),*), rows: ($($rid:tt),*))),*) => {
        paste::paste! {
            pub struct PrimaryKB {
                pub matrices: (
                    $((
                        ($(Option<$crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $cid >]>>),*,),
                        ($($crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $rid >]>),*,),
                    ),),*
                ),
                pub keys: rkb2040_proc::matrix_to_states!($crate::pin::PinState<$delay>, $((($($cid),*), ($($rid),*))),*),
                pub tx: $rx,
                pub sm: $sm,
            }

            impl PrimaryKB {
                #[must_use]
                pub fn new(
                    matrices: (
                        $((
                            ($(Option<$crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $cid >]>>),*,),
                            ($($crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $rid >]>),*,),
                        ),),*
                    ),
                    keys: rkb2040_proc::matrix_to_states!($crate::pin::PinState<$delay>, $((($($cid),*), ($($rid),*))),*),
                    tx: $tx,
                    sm: $sm,
                ) -> Self {
                    Self { matrices, keys, tx, sm }
                }

                pub fn scan(&mut self, timer: rp2040_hal::Timer) {
                    self.sm.begin_scan();
                    rkb2040_proc::matrix_pin_check!(self, timer, $((($($cid),*), ($($rid),*))),*);
                    self.sm.finish_scan();
                }
            }

            macro_rules! init_secondary_pins {
                ($_pins:ident) => {(
                    (
                        $((
                            ($(Some($_pins.[< gpio $cid >].into_pull_up_input())),*,),
                            ($($_pins.[< gpio $rid >].into_pull_up_input()),*,),
                        ),),*
                    ),
                    rkb2040_proc::matrix_to_states_init!($crate::pin::PinState, $((($($cid),*), ($($rid),*))),*)
                )}
            }
            pub(crate) use init_primary_pins;
        }
    };
}
