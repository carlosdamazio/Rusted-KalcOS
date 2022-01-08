#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello, World!\nThis is KalcOS!\n";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buf: *mut u8 = 0xB8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xB;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
