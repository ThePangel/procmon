use crate::{bus::Bus, proc_bus::ProcBus};
use std::io;

pub fn write<B: Bus>(bus: &mut B, address: u64, bytes: Vec<u8>, mem_width: usize) {
    for (i, byte) in bytes.iter().enumerate() {
        bus.write(address + i as u64, *byte);
    }
    read(
        bus,
        (address, address.wrapping_add(bytes.len() as u64) - 1),
        mem_width,
    );
}

pub fn proc_read<B: ProcBus>(bus: &mut B, address: (u64, u64)) {
    let n_addresses = address.1.wrapping_sub(address.0) + 1;
    let mut block = String::new();
    for i in (0..n_addresses).step_by(16) {
        let base_addrs = address.0.wrapping_add(i);
        for j in 0..16 {
            if !((address.0)..=address.1).contains(&base_addrs.wrapping_add(j)) {
                break;
            }
            let Some(val) = bus.read(base_addrs.wrapping_add(j)) else {
                println!("Address is not part of process address space");

                return;
            };

            if j % 16 == 0 {
                block.push_str(&format!(
                    "\n{:016X}: {:02X}",
                    address.0.wrapping_add(i),
                    val
                ));
            } else {
                block.push_str(&format!(" {:02X}", val));
            }
        }
        for ascii in 0..16 {
            if !(address.0..=address.1).contains(&base_addrs.wrapping_add(ascii)) {
                println!("{block}");
                return;
            }
            let Some(val) = bus.read(base_addrs.wrapping_add(ascii)) else {
                return;
            };

            if val == 0x00 {
                block.push_str(" ··");
            } else {
                block.push_str(&format!(" {}", val as char));
            }
        }
    }
    println!("{block}");
}

pub fn proc_write<B: ProcBus>(bus: &mut B, address: u64, bytes: Vec<u8>) {
    for (i, byte) in bytes.iter().enumerate() {
        bus.write(address + i as u64, *byte);
    }

    proc_read(bus, (address, address.wrapping_add(bytes.len() as u64) - 1));
}

pub fn read<B: Bus>(bus: &mut B, address: (u64, u64), mem_width: usize) {
    let n_addresses = address.1.wrapping_sub(address.0) + 1;
    let mut block = String::new();
    for i in (0..n_addresses).step_by(16) {
        let base_addrs = address.0.wrapping_add(i);
        for j in 0..16 {
            if !((address.0)..=address.1).contains(&base_addrs.wrapping_add(j)) {
                break;
            }
            let Some(val) = bus.read(base_addrs.wrapping_add(j)) else {
                return;
            };

            if j % 16 == 0 {
                block.push_str(&format!(
                    "\n{:0w$X}: {:02X}",
                    address.0.wrapping_add(i),
                    val,
                    w = mem_width
                ));
            } else {
                block.push_str(&format!(" {:02X}", val));
            }
        }
        for ascii in 0..16 {
            if !(address.0..=address.1).contains(&base_addrs.wrapping_add(ascii)) {
                println!("{block}");
                return;
            }
            let Some(val) = bus.read(base_addrs.wrapping_add(ascii)) else {
                return;
            };

            if val == 0x00 {
                block.push_str(" ··");
            } else {
                block.push_str(&format!(" {}", val as char));
            }
        }
    }
    println!("{block}");
}

pub fn run<B: Bus>(bus: &mut B, address: u64) {
    bus.run(address);
}

pub fn proc_region<B: ProcBus>(bus: &mut B) -> io::Result<()> {
    let regions = bus.regions()?;
    for region in regions {
        if region.write || region.read || region.exec {
            println!(
                "{:016X}-{:016X} {}{}{} {}",
                region.start,
                region.end,
                if region.read { "r" } else { "-" },
                if region.write { "w" } else { "-" },
                if region.exec { "x" } else { "-" },
                region.name
            );
        }
    }
    Ok(())
}
