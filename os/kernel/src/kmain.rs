#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(exclusive_range_pattern)]
#![feature(i128_type)]
#![feature(never_type)]
#![feature(unique)]
#![feature(pointer_methods)]
#![feature(naked_functions)]
#![feature(fn_must_use)]
#![feature(alloc, allocator_api, global_allocator)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
extern crate pi;
extern crate stack_vec;
extern crate fat32;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod fs;
pub mod traps;
pub mod aarch64;
pub mod process;
pub mod vm;

use pi::uart::MiniUart;
use pi::gpio::Gpio;
use pi::timer::Timer;
use console::Console;
use shell::shell;

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;
use process::GlobalScheduler;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

pub static SCHEDULER: GlobalScheduler = GlobalScheduler::uninitialized();

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
    // ALLOCATOR.initialize();


    let mut uart = MiniUart::new();
    let mut activity_led = Gpio::new(16).into_output();


    // loop {
        // let byte = uart.read_byte();
		// // uart.write_str("<-");
        // uart.write_byte(byte);

        // activity_led.set();
        // pi::timer::spin_sleep_ms(25);
        // activity_led.clear();

    pi::timer::spin_sleep_ms(2000); 
    console::kprintln!("Starting test...");
    shell("> ")
}
