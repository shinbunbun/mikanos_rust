// https://github.com/rust-osdev/uefi-rs/blob/main/template/src/main.rs

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(lang_items)]

use core::{
    fmt::{write, Debug},
    panic::PanicInfo,
    str::from_utf8,
};
use uefi::{
    prelude::*,
    proto::media::{
        file::{Directory, File, FileAttribute, FileHandle, FileMode, FileType, RegularFile},
        fs::SimpleFileSystem,
    },
    table::boot::{self, MemoryDescriptor, MemoryMapKey},
    CString16,
};

// https://github.com/rust-lang/rust/issues/62785/
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

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

fn open_file(system_table: &mut SystemTable<Boot>, file_path: &str) -> FileType {
    write(system_table.stdout(), format_args!("1\n")).unwrap();
    let sfs = if let Ok(sfs) = system_table
        .boot_services()
        .locate_protocol::<SimpleFileSystem>()
    {
        unsafe { &mut *sfs.get() }
    } else {
        write(
            system_table.stdout(),
            format_args!("no simple filesystem protocol\n"),
        )
        .unwrap();
        panic!("no sfs");
    };
    write(system_table.stdout(), format_args!("2\n")).unwrap();

    let mut directory = match sfs.open_volume() {
        Ok(x) => x,
        Err(err) => {
            write(
                system_table.stdout(),
                format_args!("open volume failed: {:?}\n", err),
            )
            .unwrap();
            panic!("open volume failed");
        }
    };
    write(system_table.stdout(), format_args!("3\n")).unwrap();

    let filename_buf: &mut [u16] = &mut [0; 256];
    write(system_table.stdout(), format_args!("4\n")).unwrap();

    let file_name = match uefi::CStr16::from_str_with_buf(file_path, filename_buf) {
        Ok(file_name) => file_name,
        Err(err) => {
            write(
                system_table.stderr(),
                format_args!("Convert CStr16 error: {:?}\n", err),
            )
            .unwrap();
            panic!("Convert CStr16 error");
        }
    };
    write(system_table.stdout(), format_args!("5\n")).unwrap();

    let file = match directory.open(file_name, FileMode::CreateReadWrite, FileAttribute::empty()) {
        Ok(file) => file,
        Err(err) => {
            write(
                system_table.stderr(),
                format_args!("open file failed: {:?}\n", err),
            )
            .unwrap();
            panic!("open file failed");
        }
    };
    write(system_table.stdout(), format_args!("6\n")).unwrap();

    file.into_type().unwrap()
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    if let Err(err) = uefi_services::init(&mut system_table) {
        write(system_table.stdout(), format_args!("Init Err: {:?}\n", err)).unwrap();
    };

    write(system_table.stdout(), format_args!("Hello, world!\n")).unwrap();

    get_memory_map(&mut system_table);

    write(system_table.stdout(), format_args!("100\n")).unwrap();

    open_file(&mut system_table, "memmap");

    write(system_table.stdout(), format_args!("101\n")).unwrap();

    /* let mmap_buf: &mut [u8] = &mut [0; 4096];
    if let Err(err) = system_table.boot_services().memory_map(mmap_buf) {
        write(
            system_table.stdout(),
            format_args!("Get memory map error: {:?}\n", err),
        )
        .unwrap();
    } */
    loop {}
    /* uefi_services::init(&mut system_table).unwrap();

    Status::SUCCESS */
}
