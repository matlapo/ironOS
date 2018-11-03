use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};
use std::str;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        unimplemented!()
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    // let mut uart = MiniUart::new();
    loop {
        let mut storage = [0u8; 512];
        let mut input = StackVec::new(&mut storage);
        kprint!("{}", prefix);
        loop {
            let byte = CONSOLE.lock().read_byte();
            // if this byte is the end of the input
            if byte == b'\n' || byte == b'\r' {
                let mut arguments: [&str; 64] = [""; 64];
                let result = Command::parse(str::from_utf8(input.into_slice()).unwrap(), &mut arguments);

                // check if errors

            } else {
                // let result = input.push(byte);
            }
        }

    }
}
