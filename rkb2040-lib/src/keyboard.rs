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
