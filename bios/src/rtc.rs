use core::arch::asm;

#[inline]
pub fn ticks() -> u32 {
    let (hi, lo): (u16, u16);
    unsafe { asm!("int 0x1A", in("ah") 0u8, lateout("cx") hi, lateout("dx") lo, options(nomem, nostack)) };
    (hi as u32) << 16 | lo as u32
}
