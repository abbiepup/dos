use crate::Error;
use core::arch::asm;
use core::ops::{BitOr, BitOrAssign};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[repr(transparent)]
pub struct File(u16);

impl File {
    pub fn open(path: *const u8) -> Result<Self> {
        let Self(handle);
        unsafe { asm!("int 0x21", in("ah") 0x0Fu8, in("dx") path, lateout("ax") handle, options(nostack)) }

        if !cf() {
            Ok(Self(handle))
        } else {
            // We assume the DOS implementation is correct.
            Err(unsafe { Error::new_unchecked((handle & 0xFF) as u8) })
        }
    }
}

impl Drop for File {
    #[inline]
    fn drop(&mut self) {
        unsafe { asm!("int 0x21", in("ah") 0x3Eu8, in("bx") self.0, options(nostack)) };
    }
}

#[inline]
pub fn rename_file(from: *const u8, to: *const u8) {
    unsafe { asm!("int 0x21", in("ah") 0x56u8, in("dx") from, in("di") to, options(nostack)) }
}

#[inline]
pub fn remove_file(path: *const u8) {
    unsafe { asm!("int 0x21", in("ah") 0x41u8, in("dx") path, options(nostack)) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(u8);

impl Flags {
    /// Read-only.
    pub const R: Self = Self(1 << 0);
    /// Hidden.
    pub const H: Self = Self(1 << 1);
    /// System.
    pub const S: Self = Self(1 << 2);
    /// Volume.
    pub const V: Self = Self(1 << 3);
    /// Directory.
    pub const D: Self = Self(1 << 4);
    /// Archive.
    pub const A: Self = Self(1 << 5);
}

impl BitOr for Flags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Flags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[inline]
pub fn flags(path: *const u8) -> Flags {
    let Flags(attributes);
    unsafe { asm!("int 0x21", in("ah") 0x43u8, in("al") 0x00u8, in("dx") path, lateout("al") attributes, options(nostack)) };
    Flags(attributes)
}

#[inline(always)]
fn cf() -> bool {
    let flags: u16;
    unsafe { asm!("pushf", "pop {0:x}", out(reg) flags) }
    (flags & 0x1) != 0
}
