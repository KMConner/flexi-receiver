use super::errors::{Error, Result};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

fn digits_to_f64(digits: [EightDigit; 3]) -> Result<f64> {
    if digits[2].has_point || digits[0].has_point {
        return Err(Error::InvalidDecimalPointError);
    }
    let base = digits[0].digit as u16 * 100 + digits[1].digit as u16 * 10 + digits[2].digit as u16;
    let mut base = base as f64;
    if digits[1].has_point {
        base /= 10.0;
    }
    Ok(base)
}

pub fn parse(bin: &[u8; 3]) -> Result<f64> {
    digits_to_f64(parse_packet(bin)?)
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

    mod digits_to_f64_test {
        use crate::digit::errors::Error;
        use crate::digit::packet::{digits_to_f64, EightDigit, SingleDigit};

        #[test]
        fn success_with_decimal_point() {
            let expected = 72.5;
            let actual = digits_to_f64([EightDigit { digit: SingleDigit::Seven, has_point: false },
                EightDigit { digit: SingleDigit::Two, has_point: true },
                EightDigit { digit: SingleDigit::Five, has_point: false }]).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn success_without_decimal_point() {
            let expected = 123f64;
            let actual = digits_to_f64([EightDigit { digit: SingleDigit::One, has_point: false },
                EightDigit { digit: SingleDigit::Two, has_point: false },
                EightDigit { digit: SingleDigit::Three, has_point: false }]).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn fail_on_invalid_decimal_point() {
            let result = digits_to_f64([EightDigit { digit: SingleDigit::One, has_point: true },
                EightDigit { digit: SingleDigit::Two, has_point: false },
                EightDigit { digit: SingleDigit::Three, has_point: false }]);
            assert_eq!(result.unwrap_err(), Error::InvalidDecimalPointError);
        }
    }

    mod parse_test {
        use crate::digit::packet::parse;

        #[test]
        fn parse_test() {
            let actual = parse(&[0b00000111, 0b11011011, 0b01101101]).unwrap();
            assert_eq!(72.5f64, actual);
        }
    }
}
