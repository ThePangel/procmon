pub enum ParseResult {
    Read { address: (u64, u64) },
    Write { address: u64, bytes: Vec<u8> },
    Run { address: u64 },
    ParseError(ParseError),
    Regions,
}

pub enum ParseError {
    EmptyInput,
    NoSplit,
    NotHex,
    NoSpace,
}

fn is_hex(line: &str) -> bool {
    line.chars()
        .filter(|c| !c.is_whitespace())
        .all(|c| c.is_ascii_hexdigit())
}

pub fn parse(input: &str) -> Result<ParseResult, Box<dyn std::error::Error>> {
    let line = input.trim();

    if line.is_empty() {
        return Ok(ParseResult::ParseError(ParseError::EmptyInput));
    }
    if line.matches(":").count() == 1 {
        let Some((str_address, str_bytes)) = line.split_once(":") else {
            return Ok(ParseResult::ParseError(ParseError::NoSplit));
        };
        if !is_hex(str_address) || !is_hex(str_bytes) {
            return Ok(ParseResult::ParseError(ParseError::NotHex));
        }

        let address = u64::from_str_radix(str_address.trim(), 16)?;

        let mut bytes: Vec<u8> = Vec::new();

        for byte in str_bytes.split_whitespace() {
            if byte.len() > 2 {
                return Ok(ParseResult::ParseError(ParseError::NoSpace));
            }
            bytes.push(u8::from_str_radix(byte, 16)?);
        }

        Ok(ParseResult::Write { address, bytes })
    } else if line.matches(".").count() == 1 {
        let Some((str_address_1, str_address_2)) = line.split_once(".") else {
            return Ok(ParseResult::ParseError(ParseError::NoSplit));
        };

        if !is_hex(str_address_1) || !is_hex(str_address_2) {
            return Ok(ParseResult::ParseError(ParseError::NotHex));
        }

        let start_address = u64::from_str_radix(str_address_1, 16)?;
        let end_address = u64::from_str_radix(str_address_2, 16)?;

        Ok(ParseResult::Read {
            address: (start_address, end_address),
        })
    } else if line.eq_ignore_ascii_case("regions") {
        Ok(ParseResult::Regions)
    } else if line.matches("R").count() == 1 {
        let Some(address) = line.strip_suffix("R") else {
            return Ok(ParseResult::ParseError(ParseError::NoSplit));
        };

        if !is_hex(address) {
            return Ok(ParseResult::ParseError(ParseError::NotHex));
        }

        Ok(ParseResult::Run {
            address: u64::from_str_radix(address, 16)?,
        })
    } else {
        if !is_hex(line) {
            return Ok(ParseResult::ParseError(ParseError::NotHex));
        }

        let address = u64::from_str_radix(line, 16)?;
        Ok(ParseResult::Read {
            address: (address, address),
        })
    }
}
