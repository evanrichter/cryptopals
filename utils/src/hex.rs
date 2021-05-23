/// Decode a string encoded with hexadecimal bytes into a `u8` container. Returns the number of
/// bytes successfully decoded in both the `Ok` and `Err` case.
#[must_use]
pub fn hex_decode<E: Extend<u8>>(from: &str, into: &mut E) -> Result<usize, usize> {
    // chunk up the input into 2 nibbles per iteration
    let byte_chunks = from.as_bytes().chunks_exact(2);

    // total number of bytes expected to parse
    let chunk_count = byte_chunks.len();

    // number of leftover bytes in input
    let remainder = byte_chunks.remainder().len();

    // parse each nibble, and if there's an error, return early with the number of bytes
    // successfully parsed
    for (n, byte) in byte_chunks.enumerate() {
        let upper = ascii_hex_val(byte[0]).map_err(|_| n)?;
        let lower = ascii_hex_val(byte[1]).map_err(|_| n)?;

        // build the byte value
        let val = upper << 4 | lower;

        // push the value onto the receiver
        into.extend(Some(val));
    }

    // leftover characters at the end produce an Error
    match remainder {
        0 => Ok(chunk_count),
        _ => Err(chunk_count),
    }
}

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        encoded.push(nibble_to_ascii(byte >> 4));
        encoded.push(nibble_to_ascii(byte & 0xf));
    }

    encoded
}

fn ascii_hex_val(hex: u8) -> Result<u8, ()> {
    match hex {
        b'0'..=b'9' => Ok(hex - b'0'),
        b'A'..=b'F' => Ok(hex - b'A' + 10),
        b'a'..=b'f' => Ok(hex - b'a' + 10),
        _ => Err(()),
    }
}

fn nibble_to_ascii(nibble: u8) -> char {
    match nibble {
        0..=9 => (nibble + b'0').into(),
        10..=16 => (nibble + b'a' - 10).into(),
        _ => panic!("incorrect nibble value"),
    }
}
