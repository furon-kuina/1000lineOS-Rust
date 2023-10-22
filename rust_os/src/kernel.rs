#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rustsbi;
use rustsbi::spec::binary::SbiRet;

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

#[allow(dead_code)]
fn kernel_main() {
    let s = "\n\nHello, world!\n";
    for ch in s.chars() {
        putchar(ch as u8)
    }

    loop {
        unsafe { asm!("wfi") }
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

#[inline(always)]
fn sbi_call(
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    fid: u32,
    eid: u32,
) -> SbiRet {
    let (error, value);
    unsafe {
        asm!(
            "ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            lateout("a0") error,
            lateout("a1") value,
            options(nostack, nomem)
        );
    }
    SbiRet { error, value }
}

fn putchar(ch: u8) {
    sbi_call(ch as u32, 0, 0, 0, 0, 0, 0, 1);
}
