// https://zenn.dev/flipflap/articles/2c30482a89d5d3
// https://github.com/uchan-nos/os-from-zero/issues/41

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(lang_items)]

use core::{arch::asm, panic::PanicInfo};

#[no_mangle]
extern "sysv64" fn kernel_main() {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[lang = "eh_personality"]
#[cfg(not(test))]
fn eh_personality() {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
