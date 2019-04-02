#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
// handle panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, World!";

// entry point on Linux:
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //cast integer to raw pointer to vga buffer address:
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            //set the letter/byte
            *vga_buffer.offset(i as isize * 2) = byte;
            //set the color (0xb = light cyan)
            *vga_buffer.offset(i as isize * 2 + 1) = 0xf;
        }
    }

    vga_buffer::print_something();

    loop {}
}
