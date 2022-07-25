// https://github.com/skoji/laranja-os/blob/osbook_day03b/bootloader/src/main.rs

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(lang_items)]

#[macro_use]
extern crate alloc;

use core::{fmt::write, panic::PanicInfo};
use uefi::{
    prelude::*,
    proto::media::{
        file::{Directory, File, FileAttribute, FileInfo, FileMode, FileType, RegularFile},
        fs::SimpleFileSystem,
    },
    table::boot::{AllocateType, MemoryDescriptor, MemoryType},
};

// https://github.com/rust-lang/rust/issues/62785/
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

const KERNEL_BASE_ADDRESS: usize = 0x100000;

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

fn open_root_dir(system_table: &mut SystemTable<Boot>) -> Directory {
    let simple_file_system = if let Ok(sfs) = system_table
        .boot_services()
        .locate_protocol::<SimpleFileSystem>()
    {
        unsafe { &mut *sfs.get() }
    } else {
        write(
            system_table.stdout(),
            format_args!("{}", "no symple file system\n"),
        )
        .unwrap();
        panic!("no sfs");
    };
    match simple_file_system.open_volume() {
        Ok(dir) => dir,
        Err(err) => {
            write(
                system_table.stdout(),
                format_args!("open volume error: {:?}", err),
            )
            .unwrap();
            panic!("open volume error");
        }
    }
}

fn get_kernel_file(system_table: &mut SystemTable<Boot>, root: &mut Directory) -> RegularFile {
    let file_name = cstr16!("day_03a_kernel.elf");
    let handle = match root.open(file_name, FileMode::Read, FileAttribute::READ_ONLY) {
        Ok(kernel_file) => kernel_file,
        Err(err) => {
            write(
                system_table.stdout(),
                format_args!("open kernel file error{:?}", err),
            )
            .unwrap();
            panic!("open kernel file error");
        }
    };
    let file_type = match handle.into_type() {
        Ok(file_type) => file_type,
        Err(err) => {
            write(
                system_table.stdout(),
                format_args!("get kernel file type error{:?}", err),
            )
            .unwrap();
            panic!("get kernel file type error");
        }
    };
    match file_type {
        FileType::Regular(regular_file) => regular_file,
        FileType::Dir(_) => {
            write(
                system_table.stdout(),
                format_args!("{}", "not regular file\n"),
            )
            .unwrap();
            panic!("not regular file");
        }
    }
}

fn get_file_size(system_table: &mut SystemTable<Boot>, file: &mut RegularFile) -> usize {
    const BUF_SIZE: usize = 4000;
    let buf = &mut [0u8; BUF_SIZE];
    let info: &mut FileInfo = match file.get_info(buf) {
        Ok(info) => info,
        Err(err) => {
            write(
                system_table.stdout(),
                format_args!("get file info error: {:?}", err),
            )
            .unwrap();
            panic!("get file info error");
        }
    };
    info.file_size() as usize
}

fn allocate_memory_for_kerbel(
    system_table: &mut SystemTable<Boot>,
    kernel_file: &mut RegularFile,
    kernel_file_size: usize,
) {
    let page_pointer = match system_table.boot_services().allocate_pages(
        AllocateType::Address(KERNEL_BASE_ADDRESS),
        MemoryType::LOADER_DATA,
        (kernel_file_size + 0xfff) / 0x1000,
    ) {
        Ok(page_pointer) => page_pointer,
        Err(err) => {
            write(
                system_table.stdout(),
                format_args!("allocate memory error: {:?}", err),
            )
            .unwrap();
            panic!("allocate memory error");
        }
    };
    let page_buf = unsafe { &mut *(page_pointer as *mut [u8; 4096]) };
    if let Err(err) = kernel_file.read(page_buf) {
        write(
            system_table.stdout(),
            format_args!("read kernel file error: {:?}", err),
        )
        .unwrap();
        panic!("read kernel file error");
    };

    write(
        system_table.stdout(),
        format_args!(
            "Kernel: 0x{:x} ({} bytes)",
            KERNEL_BASE_ADDRESS, kernel_file_size
        ),
    )
    .unwrap();
}

fn exit_boot_services(handle: Handle, system_table: SystemTable<Boot>) {
    // exit boot service
    let max_mmap_size = system_table.boot_services().memory_map_size().map_size
        + 8 * core::mem::size_of::<MemoryDescriptor>();
    let mut mmap_storage = vec![0; max_mmap_size].into_boxed_slice();
    system_table
        .exit_boot_services(handle, &mut mmap_storage[..])
        .unwrap();
}

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    write(system_table.stdout(), format_args!("Hello, world!\n")).unwrap();

    let mut root = open_root_dir(&mut system_table);

    let mut kernel_file = get_kernel_file(&mut system_table, &mut root);

    let kernel_file_size = get_file_size(&mut system_table, &mut kernel_file);

    allocate_memory_for_kerbel(&mut system_table, &mut kernel_file, kernel_file_size);

    exit_boot_services(handle, system_table);

    // boot kernel
    unsafe {
        let entry_addr = *((KERNEL_BASE_ADDRESS + 24) as *mut usize);
        let entry_point: extern "C" fn() = core::mem::transmute(entry_addr);
        (entry_point)();
    }
    uefi::Status::SUCCESS
}
