#![cfg_attr(not(test), no_std)]
#![no_main]

mod keyboards;

#[cfg(feature = "sweep2")]
use keyboards::sweep2::{bsp::entry};

#[entry]
fn main() -> ! {
    todo!()
}

#[panic_handler]
#[inline(never)]
fn halt(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
