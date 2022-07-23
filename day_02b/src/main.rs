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

// https://github.com/skoji/laranja-os/blob/osbook_day02b/src/main.rs#L38
fn get_memory_map(system_table: &mut SystemTable<Boot>) {
    let boot_services = system_table.boot_services();
    let mmap_buf = &mut [0; 4096 * 4];
    let (_, memmap_iter) = boot_services.memory_map(mmap_buf).unwrap();
    for (i, m) in memmap_iter.enumerate() {
        write(
            system_table.stdout(),
            format_args!(
                "{}, {:?}, {}, {}, {:?}\n",
                i, m.ty, m.phys_start, m.page_count, m.att
            ),
        )
        .unwrap();
    }
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    write(system_table.stdout(), format_args!("Hello, world!\n")).unwrap();
    get_memory_map(&mut system_table);
    write(system_table.stdout(), format_args!("100\n")).unwrap();

    loop {}
}
