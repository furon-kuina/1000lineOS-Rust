use rustsbi;
use rustsbi::spec::binary::SbiRet;

use core::arch::asm;

pub fn putchar(c: u8) {
    sbi_call(c as u32, 0, 0, 0, 0, 0, 0, 1);
}

pub fn puts(s: &str) {
    for byte in s.bytes() {
        putchar(byte);
    }
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

pub fn memset(buf: *mut u8, c: u8, n: usize) -> *mut u8 {
    let mut p = buf;
    for _ in 0..n {
        unsafe {
            *p = c;
            p = p.add(1);
        }
    }
    p
}

fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let d = dst;
    let s = src;
    for _ in 0..n {
        unsafe {
            *d = *s;
            let _ = d.add(1);
            let _ = s.add(1);
        }
    }
    return dst;
}
