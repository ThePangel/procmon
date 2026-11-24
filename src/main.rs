mod bus;
mod mem_handler;
mod parser;
mod win_bus;
use crate::bus::Bus;
use crate::parser::{ParseError, ParseResult};
use clap::Parser;
use std::io;

fn parse_hex(string: &str) -> Result<usize, Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(u64::from_str_radix(string.trim(), 16)? as usize)
}

fn hex_width(n: usize) -> usize {
    if n == 0 { 1 } else { (n.ilog(16) + 1) as usize }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0xFFFF, value_parser = parse_hex)]
    mem_size: usize,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mem_width: usize = hex_width(args.mem_size);

    #[cfg(target_os = "windows")]
    let mut bus = win_bus::WinBus {
        mem_size: args.mem_size,
    };

    let ptr = bus.alloc();
    if ptr.is_null() {
        panic!("Failed to allocate memory");
    }
    println!("Allocated memory at: {:?}", ptr);

    let mut buffer = String::new();
    println!("\\");

    loop {
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim().eq_ignore_ascii_case("EXIT") {
            break;
        }
        match parser::parse(&buffer.to_ascii_uppercase())? {
            ParseResult::Write { address, bytes } => {
                mem_handler::write(&mut bus, ptr, address, bytes, mem_width)
            }

            ParseResult::Read { address } => mem_handler::read(&mut bus, ptr, address, mem_width),

            ParseResult::Run { address } => mem_handler::run(&mut bus, ptr, address),

            ParseResult::ParseError(error) => match error {
                ParseError::EmptyInput => {}

                ParseError::NoSplit => println!("Couldn't split input string"),

                ParseError::NotHex => println!("Input is not pure hex / Invalid command"),

                ParseError::NoSpace => println!("Please add spaces between bytes when writing"),
            },
        }

        buffer.clear();
    }
    bus.free(ptr);
    Ok(())
}
