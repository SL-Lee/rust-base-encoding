use std::convert::TryInto;

use crate::error::{Error, ErrorKind};

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut input = input.to_vec();
    let mut padding_count = 0;

    while input.len() % 4 != 0 {
        input.push(0);
        padding_count += 1;
    }

    let mut output = input
        .chunks_exact(4)
        .flat_map(|chunk| {
            let mut number = u32::from_be_bytes(chunk.try_into().unwrap());
            let mut place_values = Vec::with_capacity(5);

            while number >= 85 {
                place_values.push((number % 85) as u8);
                number /= 85;
            }

            place_values.push(number as u8);
            place_values.reverse();
            place_values
        })
        .map(|place_value| place_value + 33)
        .collect::<Vec<u8>>();

    output.truncate(output.len() - padding_count);
    output
}

pub fn decode(input: &[u8]) -> Result<Vec<u8>, Error> {
    if input
        .iter()
        .any(|&character| !(33..=117).contains(&character))
    {
        Err(Error::new(ErrorKind::MalformedInput))
    } else {
        let mut input = input.to_vec();
        let mut padding_count = 0;

        while input.len() % 5 != 0 {
            input.push(117);
            padding_count += 1;
        }

        let groups = input.chunks_exact(5);
        let mut output = Vec::with_capacity(groups.len());

        for group in groups {
            let mut number = 0;

            for (i, &character) in group.iter().enumerate() {
                if let Some(n) = ((character - 33) as u32)
                    .checked_mul(85u32.pow((5 - i - 1) as u32))
                {
                    number += n;
                } else {
                    return Err(Error::new(ErrorKind::MalformedInput));
                }
            }

            output.extend(&number.to_be_bytes());
        }

        output.truncate(output.len() - padding_count);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base85_encode_test() {
        let input = "one two three 一二三".as_bytes();
        assert_eq!("Df0,/FE_XGFD,]+AK^'DJE4;#jLC$".as_bytes(), encode(input));
    }

    #[test]
    fn base85_decode_test() {
        let input = "Df0,/FE_XGFD,]+AK^'DJE4;#jLC$".as_bytes();
        assert_eq!("one two three 一二三".as_bytes(), decode(input).unwrap());
    }
}