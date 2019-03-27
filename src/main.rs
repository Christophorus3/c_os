#![no_std]
#![no_main]

use core::panic::PanicInfo;

// entry point on Linux:
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

// handle panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}