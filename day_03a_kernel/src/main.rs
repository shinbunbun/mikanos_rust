// https://zenn.dev/flipflap/articles/2c30482a89d5d3
// https://github.com/uchan-nos/os-from-zero/issues/41

#![no_main]
#![no_std]

use core::{arch::asm, panic::PanicInfo};

#[no_mangle]
extern "C" fn kernel_main() {
    unsafe {
        loop {
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
