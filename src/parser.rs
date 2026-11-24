use crate::bus::Bus;

pub fn parse<B: Bus>(
    input: &str,
    bus: &mut B,
    ptr: *mut u8,
) -> Result<(), Box<dyn std::error::Error>> {
    if input.trim().len() > 16 {
        println!("INVALID INPUT: Please enter a hexadecimal value within size limits; 1-16 chars");
        return Ok(());
    }
    if !input.trim().contains(char::is_whitespace)
        && !input.trim().contains(".")
        && !input.trim().contains(":")
    {
        let Some(val) = bus.read(ptr, u64::from_str_radix(input.trim(), 16)?) else {
            return Ok(());
        };
        println!(
            "\n{:016X}: {:02X}",
            u64::from_str_radix(input.trim(), 16).unwrap(),
            val
        );
        return Ok(());
    } else if input.trim().contains(":") {
        let Some((str_address, str_bytes)) = input.trim().split_once(":") else {
            return Ok(());
        };

        let adrress = u64::from_str_radix(str_address.trim(), 16)?;

        let bytes = str_bytes
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16))
            .collect::<Result<Vec<u8>, _>>()?;

        for (i, byte) in bytes.iter().enumerate() {
            bus.write(ptr, adrress + i as u64, *byte);
        }

        println!("\n{:016X}:{}", adrress, str_bytes);
        return Ok(());
    }
    Ok(())
}
