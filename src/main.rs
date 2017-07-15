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
use liar::no_std::bencher::Bencher;

fn nop(b: &mut Bencher<u64>) {
    b.run(|| {});
}

fn foo(b: &mut Bencher<u64>) {
    b.run(|| {
        let mut a = [0u32; 128];
        for i in 0..128 {
            a[i] = 128 - i as u32;
        }
    })
}

fn main() {
    hprintln!("liar test starting");

    let mut samples = [None, None, None];
    let mut b = Bencher::new(&mut samples, time, diff);

    b.bench("nop", &mut nop);
    b.bench("foo", &mut foo);

    // report results over semihosting
    for sample in b.samples() {
        if !sample.is_some() {
            continue;
        }
        let sample = sample.as_ref().unwrap();
        hprintln!("{}:", sample.name);
        for s in sample.data.iter() {
            hprintln!("    {}", s);
        }
    }

    hprintln!("liar test finished");
}

// !FIX! need to set up a real timer
static mut FAKE_TIMER: u64 = 0;

fn time() -> u64 {
    unsafe {
        FAKE_TIMER += 10;
        FAKE_TIMER
    }
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
