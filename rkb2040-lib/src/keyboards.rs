#[macro_export]
macro_rules! impl_direct_wire_primary_keyboard {
    ($rx:ty, $($id:tt),*) => {
        paste::paste! {
            pub struct PrimaryKB {
                pub keys: (
                    $($crate::pins::DirectKeyPin<rp2040_hal::gpio::bank0::[< Gpio $id >]>,)*
                ),
                pub rx: $rx,
                pub cb: fn(u8),
            }

            impl PrimaryKB {
                #[must_use]
                pub fn new(
                    keys: (
                        $($crate::pins::DirectKeyPin<rp2040_hal::gpio::bank0::[< Gpio $id >]>,)*
                    ),
                    rx: $rx,
                    cb: fn(u8),
                ) -> Self {
                    Self { keys, rx, cb }
                }

                pub fn scan(&mut self, timer: rp2040_hal::Timer) {
                    rkb2040_proc::direct_pin_rx_check!(self, timer, $($id,)*);
                }
            }

            macro_rules! init_primary_pins {
                ($_pins:ident) => {
                    (
                        $($crate::pins::DirectKeyPin::new($_pins.[< gpio $id >].into_pull_up_input()),)*
                    )
                }
            }
            pub(crate) use init_primary_pins;
        }
    }
}

#[macro_export]
macro_rules! impl_direct_wire_secondary_keyboard {
    ($tx:ty, $($id:tt),*) => {
        paste::paste! {
            pub struct SecondaryKB {
                pub keys: (
                    $($crate::pins::DirectKeyPin<rp2040_hal::gpio::bank0::[< Gpio $id >]>,)*
                ),
                pub tx: $tx,
            }

            impl SecondaryKB {
                #[must_use]
                pub fn new(
                    keys: (
                        $($crate::pins::DirectKeyPin<rp2040_hal::gpio::bank0::[< Gpio $id >]>,)*
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
                        $($crate::pins::DirectKeyPin::new($_pins.[< gpio $id >].into_pull_up_input()),)*
                    )
                }
            }

            pub(crate) use init_secondary_pins;
        }
    }
}
