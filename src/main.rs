#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(not(test), no_std)]
// don't use no_main for testing
#![cfg_attr(not(test), no_main)]

mod vga_buffer;

use core::panic::PanicInfo;
// handle panics
// don't compile for testing
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// entry point on Linux:
// don't compile for testing
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World from {}", "C_OS");
    panic!("The kernel paniced");
    loop {}
}
