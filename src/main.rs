#![no_std] // No standard library. We can't use this.
#![no_main] // We do have a main, but not in the standard Rust way.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};

// Include assembly file during compilation.
// We need to include some things at the top of
// the text section.
use core::arch::global_asm;
use core::panic::PanicInfo;
use hex_literal::hex;
use risc0_zkvm_platform::syscall::sys_alloc_aligned;
use winter_air::proof::StarkProof;
use winter_crypto::DefaultRandomCoin;
use winter_math::fields::f128::BaseElement;
use winter_math::FieldElement;
use winter_verifier::AcceptableOptions;
global_asm!(include_str!("asm/init.s"));

mod air;
use air::FibAir;
mod env;
mod serde;
mod utils;

pub type Blake3_192 = winter_crypto::hashers::Blake3_192<BaseElement>;

// Since input is passed as u32, we convert it to a vector of bytes.
fn read_input() -> Vec<u8> {
    let mut input = Vec::new();

    let num_bytes: u32 = env::read();
    let num_u32 = num_bytes / 4;

    for i in 0..num_u32 {
        let u: u32 = env::read();
        let b = u.to_le_bytes();
        for i in 0..4 {
            input.push(b[i]);
        }
    }

    let rem = (num_bytes - num_u32 * 4) as usize;
    if rem > 0 {
        let u: u32 = env::read();
        let b = u.to_le_bytes();
        for i in 0..rem {
            input.push(b[i]);
        }
    }

    input
}

#[no_mangle]
pub extern "C" fn runcontract(_: u32) -> u32 {
    let input = read_input();

    let proof = match StarkProof::from_bytes(&input) {
        Ok(p) => p,
        Err(e) => return 1,
    };

    let mut set = Vec::new();
    set.push(proof.options().clone());

    let acceptable_options = AcceptableOptions::OptionSet(set);

    // The result is public information. We are using the 16th fib term.
    let fib_result = BaseElement::new(987);
    let result = winter_verifier::verify::<FibAir, Blake3_192, DefaultRandomCoin<Blake3_192>>(
        proof,
        fib_result,
        &acceptable_options,
    );

    let ret = match result {
        Ok(r) => 0,
        Err(e) => 1,
    };

    // Writing 0 means the proof checks out.
    env::write(&ret);
    return ret;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct BumpPointerAlloc;

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        sys_alloc_aligned(layout.size(), layout.align())
        //sys_alloc_aligned(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc;
