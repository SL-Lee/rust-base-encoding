use crate::error::{Error, ErrorKind};

const CHARACTERS: [u8; 64] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2b, 0x2f,
];

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut bits = input.iter().map(|byte| format!("{byte:0>8b}")).collect::<String>();

    while bits.len() % 6 != 0 {
        bits.push('0');
    }

    let mut output = Vec::new();

    for i in 0..bits.len() / 6 {
        output.push(CHARACTERS[usize::from_str_radix(&bits[i * 6..i * 6 + 6], 2).unwrap()]);
    }

    while output.len() % 4 != 0 {
        output.push(0x3d);
    }

    output
}

pub fn decode(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut bits = String::new();

    for &byte in input.iter().filter(|&&byte| byte != 0x3d) {
        bits.push_str(&format!(
            "{character_bits:0>6b}",
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
    fn base64_encode_test() {
        let input = "one two three 一二三".as_bytes();
        assert_eq!("b25lIHR3byB0aHJlZSDkuIDkuozkuIk=".as_bytes(), encode(input));
    }

    #[test]
    fn base64_decode_test() {
        let input = "b25lIHR3byB0aHJlZSDkuIDkuozkuIk=".as_bytes();
        assert_eq!("one two three 一二三".as_bytes(), decode(input).unwrap());
    }
}
