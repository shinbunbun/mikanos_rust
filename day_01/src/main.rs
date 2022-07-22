// https://github.com/rust-osdev/uefi-rs/blob/main/template/src/main.rs

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(lang_items)]

use core::{fmt::write, panic::PanicInfo};
use uefi::prelude::*;

// https://github.com/rust-lang/rust/issues/62785/
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

// https://github.com/rust-lang/rust/issues/51540#issue-332112999
#[lang = "oom"]
// https://github.com/rust-lang/rust-analyzer/issues/4490#issuecomment-1074922003
#[cfg(not(test))]
extern "C" fn oom(_: core::alloc::Layout) -> ! {
    loop {}
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    write(system_table.stdout(), format_args!("Hello, world!\n")).unwrap();
    loop {}
    /* uefi_services::init(&mut system_table).unwrap();

    Status::SUCCESS */
}
