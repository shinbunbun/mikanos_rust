// https://github.com/rust-osdev/uefi-rs/blob/main/template/src/main.rs

#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;

// https://github.com/rust-lang/rust/issues/62785/
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    Status::SUCCESS
}
