#![no_std]
#![no_main]
#![feature(start)]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// #[no_mangle]
// fn start(_argc: isize, _argv: *const *const u8) -> isize {
//     0
// }

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
