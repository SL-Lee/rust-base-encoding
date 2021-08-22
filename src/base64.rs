use crate::error::{Error, ErrorKind};

const CHARACTERS: [u8; 64] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
    84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
    107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121,
    122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 43, 47,
];

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut bits = input
        .iter()
        .map(|byte| format!("{:0>8b}", byte))
        .collect::<String>();

    while bits.len() % 6 != 0 {
        bits.push('0');
    }

    let mut output = Vec::new();

    for i in 0..bits.len() / 6 {
        output.push(
            CHARACTERS
                [usize::from_str_radix(&bits[i * 6..i * 6 + 6], 2).unwrap()],
        );
    }

    while output.len() % 4 != 0 {
        output.push(61);
    }

    output
}

pub fn decode(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut bits = String::new();

    for &byte in input.iter().filter(|&&byte| byte != 61) {
        match CHARACTERS.iter().position(|&x| x == byte) {
            Some(index) => bits.push_str(&format!("{:0>6b}", index)),
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
    fn base64_encode_test() {
        let input = "one two three 一二三".as_bytes();
        assert_eq!(
            "b25lIHR3byB0aHJlZSDkuIDkuozkuIk=".as_bytes(),
            encode(input)
        );
    }

    #[test]
    fn base64_decode_test() {
        let input = "b25lIHR3byB0aHJlZSDkuIDkuozkuIk=".as_bytes();
        assert_eq!("one two three 一二三".as_bytes(), decode(input).unwrap());
    }
}
