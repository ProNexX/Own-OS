#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!(" ------------------------------------------------------------------------------ ");
    println!(" |                           Welcome to my first OS                           | ");

    for _x in 0..20 {
        println!(" |                                                                            | ");
    }

    println!(" ------------------------------------------------------------------------------ ");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

mod vga_buffer;