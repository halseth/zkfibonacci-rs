#![no_std] // No standard library. We can't use this.
#![no_main] // We do have a main, but not in the standard Rust way.

// Include assembly file during compilation.
// We need to include some things at the top of
// the text section.
use core::arch::global_asm;
use core::panic::PanicInfo;
global_asm!(include_str!("asm/init.s"));

#[no_mangle]
pub extern "C" fn runcontract(x: u32) -> u32 {
    let mut y = x;
    let mut i = 0;
    while i < 8 {
        y = y + y;
        i = i + 1;
    }

    y
}

// Unlike C, Rust panics sometimes. This can be very
// helpful when you don't have a lot of debugging
// visibility.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
