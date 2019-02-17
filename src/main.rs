#![feature(global_asm, asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

global_asm!(include_str!("boot.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {asm!("" :::: "volatile")}
    }
}
