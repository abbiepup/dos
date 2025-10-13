//! Random value generation.

use core::arch::asm;

pub use core::random::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct DefaultRandomSource;

impl RandomSource for DefaultRandomSource {
    #[inline(never)]
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        unsafe {
            let mut cx: u16;
            let mut dx: u16;
            asm!("int 0x1A", in("ah") 0u8, out("cx") cx, out("dx") dx, options(nomem, nostack));

            let mut pit: u8;
            asm!("in al, dx", in("dx") 0x40u16, out("al") pit, options(nomem, nostack));

            let offset: u16 = bytes.as_ptr() as u16;
            let segment: u16;
            asm!("mov {0:x}, ds", out(reg) segment);

            let phys_addr: u32 = ((segment as u32) << 4) | (offset as u32);

            let mut state: u32 = ((cx as u32) << 16) ^ (dx as u32) ^ (pit as u32) ^ phys_addr;

            for chunk in bytes.chunks_mut(4) {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;

                let rnd = state.to_le_bytes();
                chunk.copy_from_slice(&rnd[..chunk.len()]);
            }
        }
    }
}

/// Generates a random value from a distribution, using the default random source.
///
/// This is a convenience function for `dist.sample(&mut DefaultRandomSource)` and will sample
/// according to the same distribution as the underlying [`Distribution`] trait implementation. See
/// [`DefaultRandomSource`] for more information about how randomness is sourced.
///
/// # Examples
///
/// Generating a [version 4/variant 1 UUID] represented as text:
/// ```
/// #![feature(random)]
///
/// use dos::random::random;
///
/// let bits: u128 = random(..);
/// let g1 = (bits >> 96) as u32;
/// let g2 = (bits >> 80) as u16;
/// let g3 = (0x4000 | (bits >> 64) & 0x0fff) as u16;
/// let g4 = (0x8000 | (bits >> 48) & 0x3fff) as u16;
/// let g5 = (bits & 0xffffffffffff) as u64;
/// let uuid = format!("{g1:08x}-{g2:04x}-{g3:04x}-{g4:04x}-{g5:012x}");
/// println!("{uuid}");
/// ```
///
/// [version 4/variant 1 UUID]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random)
pub fn random<T>(dist: impl Distribution<T>) -> T {
    dist.sample(&mut DefaultRandomSource)
}
