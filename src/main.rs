// stm32f3-liar-test src/main.rs
//
// A test harnesss for the 'liar' crate, which tests running it in no_std
// environment, specifically, on an STM32F3 Discovery development board.
//
// Copyright © 2017 Sean Bolton
//
// Licensed under the MIT/X11 license, see the included file 'LICENSE' for
// details.

#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate liar;

use cortex_m::asm;
use liar::bencher::no_std::Bencher;

fn nop(b: &mut Bencher<u64>) {
    b.run(|| {});
}

fn main() {
    hprintln!("liar test starting");

    let mut samples = [None, None, None];
    let mut b = Bencher::new(&mut samples, time, diff);

    b.bench("nop", &mut nop);
    // !FIX! do something with the results

    hprintln!("liar test finished");
}

// !FIX! need to set up a real timer
fn time() -> u64 {
    0u64
}

fn diff(start: &u64, end: &u64) -> u64 {
    end - start
}

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
// no (real) interrupt handlers
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
