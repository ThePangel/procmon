mod bus;
mod parser;
mod win_bus;
use crate::bus::Bus;

use std::io;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let mut bus = win_bus::WinBus {
        mem_size: 64 * 1024,
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
        if buffer.trim() == "exit" {
            break;
        }
        parser::parse(&buffer, &mut bus, ptr)?;

        buffer.clear();
    }
    bus.free(ptr);
    Ok(())
}
