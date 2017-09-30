use error::{Error, Result};

pub unsafe fn syscall0(mut a: usize) -> Result<usize> {
    asm!("svc #0"
        : "={x0}"(a)
        : "{x0}"(a)
        : "memory"
        : "volatile");

    Error::demux(a)
}

pub unsafe fn syscall1(mut a: usize, b: usize) -> Result<usize> {
    asm!("svc #0"
        : "={x0}"(a)
        : "{x0}"(a), "{x1}"(b)
        : "memory"
        : "volatile");

    Error::demux(a)
}

// Clobbers all registers - special for clone
pub unsafe fn syscall1_clobber(mut a: usize, b: usize) -> Result<usize> {
    asm!("svc #0"
        : "={x0}"(a)
        : "{x0}"(a), "{x1}"(b)
        : "memory", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29", "x30" // TODO: is this even necessary
        : "volatile");

    Error::demux(a)
}

pub unsafe fn syscall2(mut a: usize, b: usize, c: usize) -> Result<usize> {
    asm!("svc #0"
        : "={x0}"(a)
        : "{x0}"(a), "{x1}"(b), "{x2}"(c)
        : "memory"
        : "volatile");

    Error::demux(a)
}

pub unsafe fn syscall3(mut a: usize, b: usize, c: usize, d: usize) -> Result<usize> {
    asm!("svc #0"
        : "={x0}"(a)
        : "{x0}"(a), "{x1}"(b), "{x2}"(c), "{x3}"(d)
        : "memory"
        : "volatile");

    Error::demux(a)
}

pub unsafe fn syscall4(mut a: usize, b: usize, c: usize, d: usize, e: usize) -> Result<usize> {
    asm!("svc #0"
        : "={x0}"(a)
        : "{x0}"(a), "{x1}"(b), "{x2}"(c), "{x3}"(d), "{x4}"(e)
        : "memory"
        : "volatile");

    Error::demux(a)
}

pub unsafe fn syscall5(mut a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> Result<usize> {
    asm!("svc #0"
        : "={x0}"(a)
        : "{x0}"(a), "{x1}"(b), "{x2}"(c), "{x3}"(d), "{x4}"(e), "{x5}"(f)
        : "memory"
        : "volatile");

    Error::demux(a)
}
