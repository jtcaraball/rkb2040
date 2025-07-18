#![cfg_attr(not(test), no_std)]
#![no_main]
// Use sea_picro as source for board support package.
#![cfg(feature = "sea-picro")]
use sea_picro as bsp;

use bsp::entry;

#[entry]
fn main() -> ! {
    loop {}
}

#[panic_handler]
#[inline(never)]
fn halt(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
