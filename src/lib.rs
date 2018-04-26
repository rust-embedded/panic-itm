//! Log panic messages using the ITM (Instrumentation Trace Macrocell)
//!
//! This crate contains an implementation of `panic_fmt` that logs panic messages to the ITM
//! stimulus port 0.
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_itm;
//!
//! fn main() {
//!     panic!("FOO")
//! }
//! ```
//!
//! ``` text
//! (gdb) monitor tpiu config external uart off 8000000 2000000
//! (gdb) monitor itm port 0 on
//! (gdb) continue
//! (..)
//! ```
//!
//! ``` text
//! $ itmdump -f /dev/ttyUSB0
//! panicked at 'FOO', src/main.rs:6:5
//! ```

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(lang_items)]
#![no_std]

extern crate aligned;
#[macro_use]
extern crate cortex_m;

use aligned::Aligned;
use cortex_m::peripheral::ITM;
use cortex_m::{interrupt, itm};

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(
    args: core::fmt::Arguments,
    file: &'static str,
    line: u32,
    col: u32,
) -> ! {
    interrupt::disable();

    let itm = &mut *ITM::ptr();
    let stim = &mut itm.stim[0];

    itm::write_aligned(stim, &Aligned(*b"panicked at '"));
    itm::write_fmt(stim, args);
    itm::write_aligned(stim, &Aligned(*b"', "));
    itm::write_str(stim, file);
    iprintln!(stim, ":{}:{}", line, col);

    loop {}
}
