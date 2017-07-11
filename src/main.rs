
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

// !FIX! need to set up timer(s) to 
// TIM2 is 32-bit, but can only be clocked by the APB1 bus clock @ 8MHz
// TIM1 and TIM8 can be fed by the PLL (on my 303xC), up to 144MHz
fn time() -> u64 {
    // let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // unsafe {
    //     libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
    // }
    // (ts.tv_sec * 1_000_000_000 + ts.tv_nsec) as u64
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
