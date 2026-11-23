use crate::bus::Bus;

pub fn parse<B: Bus>(input: &str, bus: &mut B, ptr: *mut u8) {
    if input.trim().len() > 16 {
        println!("INVALID INPUT: Please enter a hexadecimal value within size limits; 1-16 chars");
        return;
    }
    if input.trim().len() == 4 {
        let val = bus.read(ptr, u64::from_str_radix(input.trim(), 16).unwrap());
        println!("\n{}: {:02X}", input.trim().to_ascii_uppercase(), val);
    }
}
