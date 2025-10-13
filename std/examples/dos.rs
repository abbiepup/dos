#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
extern "C" fn main() {
    dos::io::print("Hello, world!");
    dos::io::print_char('!');
    loop {}
}

#[cold]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
