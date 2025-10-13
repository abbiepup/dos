use crate::cp437::CP437Char;
use crate::sys::Service;
use core::arch::asm;
use thiserror::Error;

pub fn print(str: &str) {
    let mut buf = [0u8; 256];

    let mut i = 0;
    for ch in str.chars() {
        if let Some(c) = CP437Char::new(ch) {
            buf[i] = c.as_byte();
            i += 1;
        } else {
            buf[i] = b'?';
            i += 1;
        }
    }

    buf[i] = b'$';

    todo!()
}

pub fn print_char(ch: char) {
    let ch = if let Some(c) = CP437Char::new(ch) { c.as_byte() } else { b'?' };
    unsafe { asm!("int 0x21", in("ah") Service::PrintChar as u8, in("dl") ch, options(nostack, preserves_flags)) };
}

/// A specialized [`Result`] type for I/O operations.
///
/// This type is broadly used across [`dos::io`] for any operation which may
/// produce an error.
///
/// [`dos::io`]: crate::io
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
#[error("")]
pub struct Error {}

pub trait Read {
    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Creates a "by reference" adaptor for this instance of `Read`.
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}

pub trait Write {
    /// Writes a buffer into this writer, returning how many bytes were written.
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

/// A handle to the global standard output stream of the current process.
pub struct Stdout {}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let count;
        unsafe {
            asm!("int 0x21", in("ah") 0x40u8, in("bx") 1u16, in("cx") buf.len() as u16, in("dx") buf.as_ptr(), lateout("ax") count, options(nostack))
        }
        Ok(count)
    }
}
