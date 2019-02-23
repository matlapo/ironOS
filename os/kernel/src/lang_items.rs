use console::kprintln;

#[no_mangle]
#[cfg(not(test))]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: ::std::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {

    let pi = r#"            (
       (      )     )
         )   (    (
        (          `
    .-""^"""^""^"""^""-.
  (//\\//\\//\\//\\//\\//)
   ~\^^^^^^^^^^^^^^^^^^/~
     `================`

    The pi is overdone.

---------- PANIC ----------"#;

    kprintln!("{}", pi);

    kprintln!("FILE: {}", file);
    kprintln!("LINE: {}", line);
    kprintln!("COL: {}", col);
    kprintln!("");
    
    kprintln!("{}", fmt);
    

    loop { unsafe { asm!("wfe") } }
}

#[cfg(not(test))] #[lang = "eh_personality"] pub extern fn eh_personality() {}
