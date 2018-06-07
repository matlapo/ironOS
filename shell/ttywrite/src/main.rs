extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use] extern crate structopt_derive;

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};
use xmodem::{Xmodem, Progress};

mod parsers;

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

 fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader, BufRead};

    let opt = Opt::from_args();
    let mut serial = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    let mut settings = serial.read_settings().expect("could not read settings from TTY");

    settings.set_baud_rate(opt.baud_rate).expect("invalid baud rate value");
    settings.set_char_size(opt.char_width);
    settings.set_flow_control(opt.flow_control);
    settings.set_stop_bits(opt.stop_bits);

    let seconds = Duration::new(opt.timeout, 0);
    serial.set_timeout(seconds);

    serial.write_settings(&settings).expect("unable to update settings");
    
    match opt.raw {
        true =>
            match opt.input {
                Some(path) => {
                    let mut file = File::open(path).expect("find not found");
                    io::copy(&mut file, &mut serial).expect("could not transfer raw data to file");
                }
                None => {
                    let mut file = io::stdin();
                    io::copy(&mut file, &mut serial).expect("could not transfer data to stdin");
                }
            },
        false => 
            match opt.input {
                Some(path) => {
                    let mut file = File::open(path).expect("find not found");
                    match Xmodem::transmit_with_progress(file, &mut serial, progress_fn) {
                        Ok(_) => return,
                        Err(e) => panic!("Error: {:?}", e),
                    }
                },
                None => {
                    let mut file = io::stdin();
                    match Xmodem::transmit_with_progress(file, &mut serial, progress_fn) {
                        Ok(_) => return,
                        Err(e) => panic!("Error: {:?}", e),
                    }
                }
            }
    } 
}
