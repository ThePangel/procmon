use crate::bus::Bus;

pub fn write<B: Bus>(bus: &mut B, ptr: *mut u8, address: u64, bytes: Vec<u8>, mem_width: usize) {
    for (i, byte) in bytes.iter().enumerate() {
        bus.write(ptr, address + i as u64, *byte);
    }

    println!(
        "\n{:0w$X}: {}",
        address,
        bytes
            .iter()
            .map(|b| format!("{:02X} ", b))
            .collect::<String>(),
        w = mem_width,
    );
}

pub fn read<B: Bus>(bus: &mut B, ptr: *mut u8, address: (u64, u64), mem_width: usize) {
    let n_addresses = address.1.wrapping_sub(address.0) + 1;
    for i in 0..n_addresses {
        let Some(val) = bus.read(ptr, address.0.wrapping_add(i)) else {
            return;
        };

        if i % 16 == 0 {
            print!(
                "\n{:0w$X}: {:02X}",
                address.0.wrapping_add(i),
                val,
                w = mem_width
            )
        } else {
            print!(" {:02X}", val);
        }
    }
    println!()
}

pub fn run<B: Bus>(bus: &mut B, ptr: *mut u8, address: u64) {
    bus.run(ptr, address);
}
