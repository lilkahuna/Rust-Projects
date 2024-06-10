#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

// no_mange ensures the compiler doesn't change the name of the function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}", "Hello");
    println!("{}", "Hello");
    print!("Hello");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
