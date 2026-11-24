mod bus;
mod mem_handler;
mod parser;
mod win_bus;
use crate::bus::Bus;
use crate::parser::{ParseError, ParseResult};

use std::io;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mem_width: usize = 4;

    #[cfg(target_os = "windows")]
    let mut bus = win_bus::WinBus { mem_size: 0xFFFF };

    let ptr = bus.alloc();
    if ptr.is_null() {
        panic!("Failed to allocate memory");
    }
    println!("Allocated memory at: {:?}", ptr);

    let mut buffer = String::new();
    println!("\\");

    loop {
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim() == "exit" {
            break;
        }
        match parser::parse(&buffer)? {
            ParseResult::Write { address, bytes } => {
                mem_handler::write(&mut bus, ptr, address, bytes, mem_width)
            }

            ParseResult::Read { address } => mem_handler::read(&mut bus, ptr, address, mem_width),

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
