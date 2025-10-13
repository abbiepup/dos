#![no_std]
#![feature(linkage)]

use core::panic::PanicInfo;

unsafe extern "C" {
    static __bss_start: *mut u8;
    static __bss_size: usize;

    safe fn main();
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rt")]
extern "C" fn _start() -> ! {
    core::arch::naked_asm! {
        // Zero bss.
        "mov di, offset {__bss_start}",
        "mov cx, {__bss_size}",
        "shr cx, 1",
        "xor ax, ax",
        "rep stosw",
        // Call main.
        "call {main}",
        // Exit to DOS.
        "mov ax, 0x4C00",
        "int 0x21",
        __bss_start = sym __bss_start,
        __bss_size = sym __bss_size,
        main = sym main,
    };
}

#[cold]
#[panic_handler]
#[linkage = "weak"]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
