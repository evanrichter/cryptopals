/// Encode bytes into a base64 string
pub fn base64_encode(bytes: &[u8]) -> String {
    // chunk up the input into 3 bytes per iteration
    let chunks = bytes.chunks_exact(3);

    // number of leftover bytes in input (these will need to be padded)
    let remainder = chunks.remainder();

    let mut encoded = String::new();

    // encode chunks
    for chunk in chunks {
        let char1 = chunk[0] >> 2; // top 6 bits
        let char2 = (chunk[0] & 0b0011) << 4 | (chunk[1] >> 4); // 2 and 4
        let char3 = (chunk[1] & 0b1111) << 2 | (chunk[2] >> 6); // 4 and 2
        let char4 = chunk[2] & 0b111111; // low 6 bits

        encoded.push(BASE64_TABLE[char1 as usize]);
        encoded.push(BASE64_TABLE[char2 as usize]);
        encoded.push(BASE64_TABLE[char3 as usize]);
        encoded.push(BASE64_TABLE[char4 as usize]);
    }

    if remainder.is_empty() {
        return encoded;
    }

    // encode remaining bytes (last three may be padded)
    let rem0 = remainder.get(0).unwrap();
    let rem1 = remainder.get(1).unwrap_or(&0);

    let char1 = rem0 >> 2;
    let char2 = (rem0 & 0b0011) << 4 | (rem1 >> 4);
    let char3 = (rem1 & 0b1111) << 2;

    // push chars for first remainder byte
    encoded.push(BASE64_TABLE[char1 as usize]);
    encoded.push(BASE64_TABLE[char2 as usize]);

    // padding
    if remainder.len() == 1 {
        encoded.push(BASE64_PAD);
        encoded.push(BASE64_PAD);
    } else {
        encoded.push(BASE64_TABLE[char3 as usize]);
        encoded.push(BASE64_PAD);
    }

    encoded
}

/// Decode a string encoded with base64 into a `u8` container. Returns the number of bytes
/// successfully decoded in both the `Ok` and `Err` case.
#[must_use]
pub fn base64_decode<E: Extend<u8>>(from: &str, into: &mut E) -> Result<usize, usize> {
    // remove any whitespace
    let no_whitespace: Vec<u8> = from
        .as_bytes()
        .iter()
        .filter(|b| !b" \t\n\r".contains(b))
        .copied()
        .collect();

    // chunk up the input into 4 characters per iteration
    let chunks = no_whitespace.chunks(4);

    // number of bytes successfully decoded
    let mut num_success = 0;

    // parse each chunk, and if there's an error, return early with the number of bytes
    // successfully parsed
    for chunk in chunks {
        let mut val: u32 = 0;
        let mut chars = 0;
        let mut error = false;
        for character in chunk {
            if *character == BASE64_PAD as u8 {
                // padding, just shift
                val = val << 6;
            } else if let Some(position) = BASE64_TABLE
                .iter()
                .map(|&t| t as u8)
                .position(|t| t == *character)
            {
                // found character in table
                val = val << 6 | position as u32;
                chars += 1;
            } else {
                // non base64 character found
                error = true;
                break;
            }
        }

        // figure out number of underlying bytes we decoded
        let num_bytes = match chars {
            0..=1 => {
                error = true;
                0
            }
            2..=4 => chars - 1,
            _ => unreachable!(),
        };

        // update decode count
        num_success += num_bytes;

        // push the decoded values onto the receiver
        for val in &val.to_be_bytes()[1..1 + num_bytes] {
            into.extend(Some(*val));
        }

        if error {
            return Err(num_success);
        }
    }

    Ok(num_success)
}

const BASE64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const BASE64_PAD: char = '=';
