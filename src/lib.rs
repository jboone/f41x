//! Start of an STM32F4 crate

#![cfg_attr(all(target_arch = "arm",
                feature = "default-exception-handler"),
            feature(core_intrinsics))]
// #![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

#[cfg(all(target_arch = "arm", feature = "alloc"))]
extern crate alloc_cortex_m;
#[macro_use]
#[macro_reexport(bkpt)]
extern crate cortex_m;
extern crate compiler_builtins_snapshot;
#[cfg(feature = "static-ram")]
extern crate r0;
extern crate ref_slice;

pub extern crate stm32f41x_memory_map as peripheral;

#[macro_use]
mod macros;

#[cfg(target_arch = "arm")]
mod lang_items;

pub mod exception;
pub mod fpu;
pub mod interrupt;

// Default initialization routine
#[cfg(feature = "default-init")]
#[doc(hidden)]
pub unsafe fn _init() {
    #[cfg(all(target_arch = "arm", feature = "alloc"))]
    extern "C" {
        static mut _heap_end: usize;
        static mut _heap_start: usize;
    }

    match () {
        #[cfg(all(target_arch = "arm", feature = "alloc"))]
        () => {
            alloc_cortex_m::init(&mut _heap_start, &mut _heap_end);
        }
        #[cfg(not(all(target_arch = "arm", feature = "alloc")))]
        () => {}
    }
    // delay::init();
    // fpu::init();
    // itm::init();
    // l3gd20::init();
    // led::init();
    // lsm303dlhc::init();
    // serial::init();
    // time::init();
}
