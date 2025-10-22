use core::arch::asm;

#[inline]
pub fn open(file: *const u8) -> u16 {
    let mut handle: u16;
    unsafe { asm!("int 0x21", in("ah") 0x0Fu8, in("dx") file, lateout("ax") handle, options(nostack)) }
    handle
}
