#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
// handle panics
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// entry point on Linux:
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World from {}", "C_OS");
    panic!("The kernel paniced");
    loop {}
}
