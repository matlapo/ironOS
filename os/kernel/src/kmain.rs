#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(pointer_methods)] //remove me

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

#[no_mangle]
pub extern "C" fn kmain() {

	//TEST #2
	let mut pin16 = pi::gpio::Gpio::new(16 as u8).into_output();
	loop {
		pin16.set();
		pi::timer::spin_sleep_ms(1000);
		pin16.clear();
		pi::timer::spin_sleep_ms(1000);
	}
}
