#![crate_type = "cdylib"]
#![no_std]
#![feature(alloc_error_handler, linkage)]

#[macro_use]
extern crate alloc;
extern crate libc;

mod extern_alloc;
mod init;
mod nx;
mod settings;
mod util;

use core::panic;

#[derive(PartialEq, Eq, Hash)]
pub enum LoadKind {
    Kip,
    AsRtld,
    Module,
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    //unsafe { exl_exception_entry() }
}
