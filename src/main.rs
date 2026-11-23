mod bus;
mod parser;
mod win_bus;
use crate::bus::Bus;

use std::io;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let mut bus = win_bus::WinBus;

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
        } else if !buffer.trim().chars().all(|c| c.is_ascii_hexdigit()) {
            println!("INVALID INPUT: Please enter a valid hexadecimal value");
            buffer.clear();

            continue;
        }
        parser::parse(&buffer, &mut bus, ptr);
        //bus.write(ptr, u64::from_str_radix(buffer.trim(), 16).unwrap(), 0x67);

        buffer.clear();
    }
    bus.free(ptr);
    Ok(())
}
