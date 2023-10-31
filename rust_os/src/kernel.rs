#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod common;
use crate::common::memset;
use crate::common::puts;

#[allow(dead_code)]
fn kernel_main() {
    unsafe {
        memset(__bss as *mut u8, 0, (__bss_end - __bss) as usize);
    }

    my_panic("booted!");
    puts("Unreachable");

    loop {
        unsafe { asm!("wfi") }
    }
}

fn my_panic(info: &str) {
    puts("Kernel panicked: ");
    puts(info);
    puts("\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        puts("Kernel panicked: ");
    }
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
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
