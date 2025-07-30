#[macro_export]
macro_rules! impl_trrs_rx {
    ($pio:ident, $reset:ident, $gpio:ident, $gpio_func:ident, $gpio_pull:ident) => {
        pub struct UartRx {
            pub(crate) inner:
                pio_uart::PioUartRx<$gpio, $pio, rp2040_hal::pio::SM0, rp2040_hal::pio::Running>,
            _prog: pio_uart::RxProgram<$pio>,
            _sm1: rp2040_hal::pio::UninitStateMachine<($pio, rp2040_hal::pio::SM1)>,
            _sm2: rp2040_hal::pio::UninitStateMachine<($pio, rp2040_hal::pio::SM2)>,
            _sm3: rp2040_hal::pio::UninitStateMachine<($pio, rp2040_hal::pio::SM3)>,
        }

        impl UartRx {
            pub fn new(
                uart_pin: rp2040_hal::gpio::Pin<$gpio, $gpio_func, $gpio_pull>,
                baud: rp2040_hal::fugit::HertzU32,
                system_freq: rp2040_hal::fugit::HertzU32,
                pio: $pio,
                resets: &mut $reset,
            ) -> Self {
                let rx_pin = uart_pin.reconfigure();
                let (mut pio, sm0, sm1, sm2, sm3) = pio.split(resets);
                let mut rx_program = pio_uart::install_rx_program(&mut pio).ok().unwrap();
                let rx = pio_uart::PioUartRx::new(rx_pin, sm0, &mut rx_program, baud, system_freq)
                    .enable();
                Self {
                    inner: rx,
                    _prog: rx_program,
                    _sm1: sm1,
                    _sm2: sm2,
                    _sm3: sm3,
                }
            }
        }

        pub(crate) struct TrrsRx {
            pub uart: UartRx,
        }

        impl TrrsRx {
            pub fn new(uart: UartRx) -> Self {
                Self { uart }
            }

            #[inline]
            pub fn receive_byte(&mut self) -> Option<u8> {
                self.uart.inner.read_one()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_trrs_tx {
    ($pio:ident, $reset:ident, $gpio:ident, $gpio_func:ident, $gpio_pull:ident) => {
        pub struct UartTx {
            pub(crate) inner:
                pio_uart::PioUartTx<$gpio, $pio, rp2040_hal::pio::SM0, rp2040_hal::pio::Running>,
            _prog: pio_uart::TxProgram<$pio>,
            _sm1: rp2040_hal::pio::UninitStateMachine<($pio, rp2040_hal::pio::SM1)>,
            _sm2: rp2040_hal::pio::UninitStateMachine<($pio, rp2040_hal::pio::SM2)>,
            _sm3: rp2040_hal::pio::UninitStateMachine<($pio, rp2040_hal::pio::SM3)>,
        }

        impl UartTx {
            pub fn new(
                uart_pin: rp2040_hal::gpio::Pin<$gpio, $gpio_func, $gpio_pull>,
                baud: rp2040_hal::fugit::HertzU32,
                system_freq: rp2040_hal::fugit::HertzU32,
                pio: $pio,
                resets: &mut $reset,
            ) -> Self {
                let rx_pin = uart_pin.reconfigure();
                let (mut inner_pio, sm0, sm1, sm2, sm3) = pio.split(resets);
                let mut tx_program = pio_uart::install_tx_program(&mut inner_pio).ok().unwrap();
                let rx = pio_uart::PioUartTx::new(rx_pin, sm0, &mut tx_program, baud, system_freq)
                    .enable();
                Self {
                    inner: rx,
                    _prog: tx_program,
                    _sm1: sm1,
                    _sm2: sm2,
                    _sm3: sm3,
                }
            }
        }

        pub(crate) struct TrrsTx {
            pub uart: UartTx,
        }

        impl TrrsTx {
            pub fn new(uart: UartTx) -> Self {
                Self { uart }
            }

            #[inline]
            pub(crate) fn send_byte(&mut self, msg: u8) {
                self.uart.inner.blocking_write_byte(msg);
            }
        }
    };
}
