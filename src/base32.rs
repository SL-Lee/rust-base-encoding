use crate::error::{Error, ErrorKind};

const CHARACTERS: [u8; 32] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
];

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut bits = input.iter().map(|byte| format!("{byte:0>8b}")).collect::<String>();

    while bits.len() % 5 != 0 {
        bits.push('0');
    }

    let mut output = Vec::new();

    for i in 0..bits.len() / 5 {
        output.push(CHARACTERS[usize::from_str_radix(&bits[i * 5..i * 5 + 5], 2).unwrap()]);
    }

    while output.len() % 8 != 0 {
        output.push(0x3d);
    }

    output
}

pub fn decode(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut bits = String::new();

    for &byte in input.iter().filter(|&&byte| byte != 0x3d) {
        bits.push_str(&format!(
            "{character_bits:0>5b}",
            character_bits = CHARACTERS
                .iter()
                .position(|&x| x == byte)
                .ok_or_else(|| Error::new(ErrorKind::MalformedInput))?
        ));
    }

    while bits.len() % 8 != 0 {
        bits.pop();
    }

    let mut output = Vec::new();

    for i in 0..bits.len() / 8 {
        output.push(u8::from_str_radix(&bits[i * 8..i * 8 + 8], 2).unwrap());
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base32_encode_test() {
        let input = "one two three 一二三".as_bytes();
        assert_eq!("N5XGKIDUO5XSA5DIOJSWKIHEXCAOJOUM4S4IS===".as_bytes(), encode(input));
    }

    #[test]
    fn base32_decode_test() {
        let input = "N5XGKIDUO5XSA5DIOJSWKIHEXCAOJOUM4S4IS===".as_bytes();
        assert_eq!("one two three 一二三".as_bytes(), decode(input).unwrap());
    }
}
