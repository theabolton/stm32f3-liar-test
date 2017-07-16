// stm32f3-liar-test src/main.rs
//
// A test harnesss for the 'liar' crate, which tests running it in no_std
// environment, specifically, on an STM32F3 Discovery development board.
//
// Copyright © 2017 Sean Bolton
//
// Licensed under the MIT/X11 license, see the included file 'LICENSE' for
// details.

// This uses the DWT cycle counter, clocked at 72MHz using the external 8MHz HSE
// clock from the ST-Link, multiplied by 9 the PLL. The CYCCNT register is
// 32-bit, so at 72MHz that means intervals of just over 59 seconds can be timed
// before overflow. Each tick of the counter represent one CPU clock cycle of
// about 13.8 nanoseconds.
// (With the internal 8MHz HSI clock, intervals of almost 9 minutes (536
// seconds) can be timed before overflow, at 125 nanoseconds per count.)

#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f30x;
extern crate liar;

use cortex_m::asm;
use cortex_m::peripheral::DWT;
use stm32f30x::{FLASH, RCC};
use liar::black_box;
use liar::no_std::bencher::Bencher;

// set_sys_clock()
// Set the system clock to 72MHz, using the 8MHz external clock from ST-Link.
// This assumes the clock and PLL are still in their reset state, and turns
// off the HSI clock when no longer needed, but otherwise follows the
// STM32F3-Discovery_FW_V1.1.0 library procedure.
fn set_sys_clock() {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let flash = FLASH.borrow(cs);

        // turn on HSE with bypass
        rcc.cr.modify(|_, w| unsafe { w.hseon().bits(1)
                                       .hsebyp().bits(1) });
        // wait for HSE to become ready
        let mut startup_count = 0x500;
        while rcc.cr.read().hserdy().bits() == 0 {
            startup_count -= 1;
            if startup_count == 0 {
                asm::bkpt();  // HSE did not become ready; halt
            }
        }
        // set flash prefetch and latency
        flash.acr.modify(|_, w| unsafe { w.prftbe().bits(1)
                                          .latency().bits(0b010) });
        // set bus clocks
        rcc.cfgr.modify(|_, w| unsafe {
             w.hpre().bits(0) // HCLK = SYSCLK
             .ppre2().bits(0) // PCLK2 = HCLK
             .ppre1().bits(0b100) // PCLK1 = HCLK / 2
        });
        // set PLL for 9 times HSE input
        rcc.cfgr.modify(|_, w| unsafe {
            w.pllsrc().bits(1) // PLL source HSE/PREDIV
            .pllmul().bits(0b0111) // PLL multiplier 9
        });
        // enable PLL and wait for it to ready
        rcc.cr.modify(|_, w| unsafe { w.pllon().bits(1) });
        while rcc.cr.read().pllrdy().bits() == 0 {}
        // select PLL as system clock
        rcc.cfgr.modify(|_, w| unsafe { w.sw().bits(0b10) });
        // wait until PLL is used
        while rcc.cfgr.read().sws().bits() != 0b10 {}
        // turn off HSI
        rcc.cr.modify(|_, w| unsafe { w.hsion().bits(0) });
    });
}

fn nop(b: &mut Bencher<u64>) {
    b.run(|| { black_box(0); });
}

fn foo(b: &mut Bencher<u64>) {
    b.run(|| {
        let mut a = [0u32; 128];
        for i in 0..128 {
            a[i] = black_box(128 - i as u32);
        }
    })
}

fn main() {
    // set system clock (and the cycle counter) to 72MHz
    set_sys_clock();
    // enable the cycle counter
    let dwt = DWT.get();
    unsafe { (*dwt).enable_cycle_counter(); }

    hprintln!("liar test starting");

    let mut samples = [None, None, None];
    let mut b = Bencher::new(&mut samples, time, diff);

    b.bench("nop", &mut nop);
    b.bench("foo", &mut foo);

    // report results over semihosting
    let ns = 1f32 / 72_000_000f32 * 1e9f32;
    hprint!("test results:\nunits are average count of CPU clocks at ");
    hprintln!("{} nanoseconds each", ns);
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

fn time() -> u64 {
    // read the DWT cycle counter
    let dwt = DWT.get();
    unsafe {
        (*dwt).cyccnt.read() as u64
    }
}

fn diff(start: &u64, end: &u64) -> u64 {
    // The cycle counter wraps without any bookkeeping, so do the subtraction
    // modulo 2^32.
    (*end as u32).wrapping_sub(*start as u32) as u64
}

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
// no (real) interrupt handlers
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
