use core::arch::asm;
use core::cell::LazyCell;

pub mod env_consts;
pub mod fs;

#[repr(u8)]
pub enum Service {
    Print = 0x09,
    PrintChar = 0x02,
    Terminate = 0x4C,
    Version = 0x30,
}

#[derive(Debug, Clone, Copy)]
pub struct Version {
    major: u8,
    minor: u8,
}

#[unsafe(no_mangle)]
pub fn version() -> Version {
    static mut VERSION: LazyCell<Version> = LazyCell::new(|| {
        let (major, minor);
        unsafe { asm!("int 0x21", inlateout("ah") Service::Version as u8 => major, lateout("al") minor, options(nostack, preserves_flags)) }
        Version { major, minor }
    });
    unsafe { *VERSION }
}
