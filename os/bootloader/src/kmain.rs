#![feature(asm, lang_items)]

extern crate xmodem;
extern crate pi;

pub mod lang_items;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be loaded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br $0" : : "r"(addr as usize));
        loop { asm!("nop" :::: "volatile")  }
    }
}


/// idea 1: load kernel in 128 bytes chunks, write chunk to memory starting at 0x80000
/// idea 2: load everything at once and write everything at once as well
// #[no_mangle]
// pub extern "C" fn kmain() {
//     let uart = pi::uart::MiniUart::new();
//     let mut xmodem = xmodem::Xmodem::new(uart);
//     let mut buf = [0u8; 128];
//     let byte_pointer = BINARY_START;
//     loop {
//         // let result = xmodem.read_packet(&mut buf);
//         let result = xmodem.receive(&mut buf);
//         match result {
//             Err(_) => { pi::timer::spin_sleep_ms(750); continue; },
//             Ok(0) => { break; },
//             Ok(bytes) => {
//                 unsafe {
//                     lang_items::memcpy(byte_pointer, &buf[0], bytes);
//                     *byte_pointer += bytes as u8;
//                 }
//             }
//         }
//     }
//     jump_to(BINARY_START);
// }


// from_raw_parts_mut == builds a [u8] from start/end of raw memory
#[no_mangle]
pub extern "C" fn kmain() {
    let mut uart = pi::uart::MiniUart::new();
    uart.set_read_timeout(750);
    let mut available_memory: &mut [u8];
    unsafe {
            available_memory = std::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE);
    }
    loop {
        let result = xmodem::Xmodem::receive(&mut uart, &mut available_memory);
        match result {
            Err(_) => { continue; }
            Ok(_) => { break; }
        }
    }

    jump_to(BINARY_START);
}
