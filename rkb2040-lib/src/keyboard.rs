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
    ($delay:tt, $rx:ty, $sm:ty, matrix: (cols: ($($cid:tt),*), rows: ($($rid:tt),*)), keys: ($(($c:tt, $r:tt)),*)) => {
        paste::paste! {
            pub struct PrimaryKB {
                pub matrix: (
                    ($(Option<$crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $cid >]>>),*,),
                    ($($crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $rid >]>),*,),
                ),
                pub keys: rkb2040_proc::keys_to_states!($crate::pin::PinState<$delay>, $(($c, $r)),*),
                pub rx: $rx,
                pub sm: $sm,
            }

            impl PrimaryKB {
                #[must_use]
                pub fn new(
                    matrix: (
                        ($(Option<$crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $cid >]>>),*,),
                        ($($crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $rid >]>),*,),
                    ),
                    keys: rkb2040_proc::keys_to_states!($crate::pin::PinState<$delay>, $(($c, $r)),*),
                    rx: $rx,
                    sm: $sm,
                ) -> Self {
                    Self { matrix, keys, rx, sm }
                }

                pub fn scan(&mut self, timer: rp2040_hal::Timer) {
                    self.sm.begin_scan();
                    rkb2040_proc::matrix_pin_rx_check!(self, timer, (($($cid),*), ($($rid),*)), $(($c, $r)),*);
                    self.sm.finish_scan();
                }
            }

            macro_rules! init_primary_pins {
                ($_pins:ident) => {(
                    (
                        ($(Some($_pins.[< gpio $cid >].into_pull_up_input())),*,),
                        ($($_pins.[< gpio $rid >].into_pull_up_input()),*,),
                    ),
                    rkb2040_proc::keys_to_states_init!($crate::pin::PinState, $(($c, $r)),*)
                )}
            }
            pub(crate) use init_primary_pins;
        }
    };
}

#[macro_export]
macro_rules! impl_matrix_secondary_keyboard {
    ($delay:tt, $tx:ty, matrix: (cols: ($($cid:tt),*), rows: ($($rid:tt),*)), keys: ($(($c:tt, $r:tt)),*)) => {
        paste::paste! {
            pub struct SecondaryKB {
                pub matrix: (
                    ($(Option<$crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $cid >]>>),*,),
                    ($($crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $rid >]>),*,),
                ),
                pub keys: rkb2040_proc::keys_to_states!($crate::pin::PinState<$delay>, $(($c, $r)),*),
                pub tx: $tx,
            }

            impl SecondaryKB {
                #[must_use]
                pub fn new(
                    matrix: (
                        ($(Option<$crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $cid >]>>),*,),
                        ($($crate::pin::KeyPin<rp2040_hal::gpio::bank0::[< Gpio $rid >]>),*,),
                    ),
                    keys: rkb2040_proc::keys_to_states!($crate::pin::PinState<$delay>, $(($c, $r)),*),
                    tx: $tx,
                ) -> Self {
                    Self { matrix, keys, tx }
                }

                pub fn scan(&mut self, timer: rp2040_hal::Timer) {
                    rkb2040_proc::matrix_pin_check!(self, timer, (($($cid),*), ($($rid),*)), $(($c, $r)),*);
                }
            }

            macro_rules! init_secondary_pins {
                ($_pins:ident) => {(
                    (
                        ($(Some($_pins.[< gpio $cid >].into_pull_up_input())),*,),
                        ($($_pins.[< gpio $rid >].into_pull_up_input()),*,),
                    ),
                    rkb2040_proc::keys_to_states_init!($crate::pin::PinState, $(($c, $r)),*)
                )}
            }
            pub(crate) use init_secondary_pins;
        }
    };
}
