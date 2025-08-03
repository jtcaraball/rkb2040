#[macro_export]
macro_rules! hardware_init {
    (($bsp:ident, $scan_freq:tt) -> $pac:ident, $clocks:ident, $timer:ident, $pins:ident, $scan_cd:ident) => {
        let mut $pac = $bsp::pac::Peripherals::take().unwrap();
        let mut watchdog = $bsp::hal::Watchdog::new($pac.WATCHDOG);
        let $clocks = $bsp::hal::clocks::init_clocks_and_plls(
            bsp::XOSC_CRYSTAL_FREQ,
            $pac.XOSC,
            $pac.CLOCKS,
            $pac.PLL_SYS,
            $pac.PLL_USB,
            &mut $pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let $timer = $bsp::hal::Timer::new($pac.TIMER, &mut $pac.RESETS, &$clocks);
        let mut $scan_cd = $timer.count_down();
        $scan_cd.start($scan_freq.millis());

        let sio = $bsp::hal::Sio::new($pac.SIO);
        let $pins = $bsp::Pins::new(
            $pac.IO_BANK0,
            $pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut $pac.RESETS,
        );
    };
}

#[macro_export]
macro_rules! usb_init {
    (($bsp: ident, $pac:ident, $clocks:ident, $timer:ident, $prod:tt, $serial:tt) -> $hid_dev:ident, $usb_dev:ident, $tick_cd:ident) => {
        let usb_bus = usb_device::bus::UsbBusAllocator::new($bsp::hal::usb::UsbBus::new(
            $pac.USBCTRL_REGS,
            $pac.USBCTRL_DPRAM,
            $clocks.usb_clock,
            true,
            &mut $pac.RESETS,
        ));
        let mut $hid_dev = usbd_human_interface_device::prelude::UsbHidClassBuilder::new()
            .add_device(
                usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig::default(),
            )
            .build(&usb_bus);
        let mut $usb_dev = usb_device::device::UsbDeviceBuilder::new(
            &usb_bus,
            usb_device::device::UsbVidPid(0x1209, 0x0001),
        )
        .strings(&[usb_device::device::StringDescriptors::default()
            .manufacturer("rkb2040")
            .product($prod)
            .serial_number($serial)])
        .unwrap()
        .build();

        let mut $tick_cd = $timer.count_down();
        $tick_cd.start(1.millis());
    };
}

#[macro_export]
macro_rules! run_primary_keyboard {
    ($kb:ident, $hid_dev:ident, $usb_dev:ident, $timer:ident, $scan_cd:ident, $tick_cd:ident) => {
        loop {
            $kb.scan($timer);
            if $scan_cd.wait().is_ok() {
                if let Some(scan) = $kb.sm.get_scan() {
                    match $hid_dev.device().write_report(scan) {
                        Err(usbd_human_interface_device::UsbHidError::WouldBlock) => {}
                        Err(usbd_human_interface_device::UsbHidError::Duplicate) => {}
                        Ok(_) => {
                            $kb.sm.clear_last_scan();
                        }
                        Err(e) => {
                            core::panic!("Failed to write keyboard report: {:?}", e)
                        }
                    }
                }
            }

            if $tick_cd.wait().is_ok() {
                match $hid_dev.tick() {
                    Err(usbd_human_interface_device::UsbHidError::WouldBlock) => {}
                    Ok(_) => {}
                    Err(e) => {
                        core::panic!("Failed to process keyboard tick: {:?}", e)
                    }
                }
            }

            if $usb_dev.poll(&mut [&mut $hid_dev]) {
                match $hid_dev.device().read_report() {
                    Err(usb_device::UsbError::WouldBlock) => {}
                    Err(e) => {
                        core::panic!("Failed to read keyboard report: {:?}", e)
                    }
                    Ok(_) => {}
                }
            }
        }
    };
}

#[macro_export]
macro_rules! run_secondary_keyboard {
    ($kb:ident, $timer:ident, $scan_cd:ident) => {
        loop {
            if $scan_cd.wait().is_ok() {
                $kb.scan($timer);
            }
        }
    };
}
