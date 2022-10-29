use super::errors::{Error, Result};

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
enum SingleDigit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[derive(Debug, Eq, PartialEq)]
struct EightDigit {
    pub digit: SingleDigit,
    pub has_point: bool,
}

fn parse_digit(bin: u8) -> Result<EightDigit> {
    let has_point = (bin & 0b10000000) != 0;
    let digit = bin & 0b01111111;
    let digit = match digit {
        0b0111111 => SingleDigit::Zero,
        0b0000110 => SingleDigit::One,
        0b1011011 => SingleDigit::Two,
        0b1001111 => SingleDigit::Three,
        0b1100110 => SingleDigit::Four,
        0b1101101 => SingleDigit::Five,
        0b1111101 => SingleDigit::Six,
        0b0000111 => SingleDigit::Seven,
        0b1111111 => SingleDigit::Eight,
        0b1101111 => SingleDigit::Nine,
        _ => {
            return Err(Error::DigitParseError(format!("invalid packet byte {:X}", bin)));
        }
    };
    Ok(EightDigit { digit, has_point })
}

fn parse_packet(packet: &[u8; 3]) -> Result<[EightDigit; 3]> {
    Ok([parse_digit(packet[0])?,
        parse_digit(packet[1])?,
        parse_digit(packet[2])?
    ])
}

#[cfg(test)]
mod test {
    use crate::digit::packet::{EightDigit, parse_packet, SingleDigit};

    mod parse_digit_test {
        use crate::digit::packet::{EightDigit, parse_digit, SingleDigit};

        #[test]
        fn parse_digit_test_0() {
            let expected = EightDigit { digit: SingleDigit::Zero, has_point: true };
            let actual = parse_digit(0b10111111).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn parse_digit_test_1() {
            let expected = EightDigit { digit: SingleDigit::One, has_point: true };
            let actual = parse_digit(0b10000110).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn parse_digit_test_2() {
            let expected = EightDigit { digit: SingleDigit::Two, has_point: false };
            let actual = parse_digit(0b01011011).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn parse_digit_test_3() {
            let expected = EightDigit { digit: SingleDigit::Three, has_point: false };
            let actual = parse_digit(0b01001111).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn parse_packet_test() {
        let expected = [EightDigit { digit: SingleDigit::Seven, has_point: false },
            EightDigit { digit: SingleDigit::Two, has_point: true },
            EightDigit { digit: SingleDigit::Six, has_point: false }
        ];
        let actual = parse_packet(&[0b00000111, 0b11011011, 0b1111101]).unwrap();
        assert_eq!(expected, actual);
    }
}
