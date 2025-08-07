use core::{arch::asm, mem::MaybeUninit};

use embedded_hal::delay::DelayNs;
use rp2040_hal::{Timer, rom_data::reset_to_usb_boot};

const RESET_FLAG: u32 = 0x007A_8EFB;

macro_rules! read {
    ($src:expr, $dest:expr) => {
        unsafe {
            asm!(
                "ldr {val}, [{addr}]",
                addr = in(reg) $src,
                val = out(reg) $dest,
            );
        }
    };
}

macro_rules! store {
    ($src:expr, $dest:expr) => {
        unsafe {
            asm!(
                "str {val}, [{addr}]",
                val = in(reg) $src,
                addr = in(reg) $dest,
            );
        }
    };
}

#[inline]
#[expect(static_mut_refs)]
#[expect(clippy::empty_loop)]
pub fn bootloader_double_tap_reset(timer: &mut Timer, delay_ms: u32, led_pin_mask: u32) {
    #[unsafe(link_section = ".uninit")]
    static mut RESET_ADDR: MaybeUninit<u32> = MaybeUninit::uninit();

    let mut value: u32;
    read!(RESET_ADDR.as_ptr(), value);

    if value == RESET_FLAG {
        value = 0;
        store!(value, RESET_ADDR.as_ptr());
        timer.delay_ms(delay_ms);
        reset_to_usb_boot(led_pin_mask, 0);
        loop {}
    }

    value = RESET_FLAG;
    store!(value, RESET_ADDR.as_ptr());

    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    timer.delay_ms(delay_ms);

    value = 0;
    store!(value, RESET_ADDR.as_ptr());
}
