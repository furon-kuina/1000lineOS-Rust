#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod common;
use crate::common::puts;

#[allow(dead_code)]
fn kernel_main() {
    puts("Hello, world!\n");

    loop {
        unsafe { asm!("wfi") }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

fn memset(buf: *mut u8, c: u8, n: usize) {
    let mut p = buf;
    for _ in 0..n {
        unsafe {
            *p = c;
            p = p.add(1);
        }
    }
}

#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn boot() -> ! {
    asm!(
        "mv sp, {stack_top}\n
        j {kernel_main}\n",
        stack_top = in(reg) &__stack_top,
        kernel_main = sym kernel_main,
    );
    loop {}
}
