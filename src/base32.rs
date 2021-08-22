use crate::error::{Error, ErrorKind};

const CHARACTERS: [u8; 32] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
    84, 85, 86, 87, 88, 89, 90, 50, 51, 52, 53, 54, 55,
];

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut bits = input
        .iter()
        .map(|byte| format!("{:0>8b}", byte))
        .collect::<String>();

    while bits.len() % 5 != 0 {
        bits.push('0');
    }

    let mut output = Vec::new();

    for i in 0..bits.len() / 5 {
        output.push(
            CHARACTERS
                [usize::from_str_radix(&bits[i * 5..i * 5 + 5], 2).unwrap()],
        );
    }

    while output.len() % 8 != 0 {
        output.push(61);
    }

    output
}

pub fn decode(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut bits = String::new();

    for &byte in input.iter().filter(|&&byte| byte != 61) {
        match CHARACTERS.iter().position(|&x| x == byte) {
            Some(index) => bits.push_str(&format!("{:0>5b}", index)),
            None => return Err(Error::new(ErrorKind::MalformedInput)),
        }
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
        assert_eq!(
            "N5XGKIDUO5XSA5DIOJSWKIHEXCAOJOUM4S4IS===".as_bytes(),
            encode(input)
        );
    }

    #[test]
    fn base32_decode_test() {
        let input = "N5XGKIDUO5XSA5DIOJSWKIHEXCAOJOUM4S4IS===".as_bytes();
        assert_eq!("one two three 一二三".as_bytes(), decode(input).unwrap());
    }
}
