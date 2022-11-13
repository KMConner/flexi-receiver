use std::io::Read;
use super::{Error, Result};

#[derive(Clone, Debug)]
enum ParserState {
    WaitStart,
    WaitLength,
    WaitBody(usize, Vec<u8>),
    WaitChecksum(Vec<u8>, usize),
    WaitEnd(Vec<u8>),
    End(Vec<u8>),
}

pub struct FlexiConnection<R: Read> {
    connection: R,
    parser_state: ParserState,
}

const START_BYTE: u8 = 0x9B;
const END_BYTE: u8 = 0x9D;
const CHECKSUM_LENGTH: usize = 2;

impl<R: Read> FlexiConnection<R> {
    pub fn new(read: R) -> Self {
        FlexiConnection {
            connection: read,
            parser_state: ParserState::WaitStart,
        }
    }

    pub fn read_packet(&mut self) -> Result<Vec<u8>> {
        loop {
            let mut byte: [u8; 1] = [0];
            let result = self.connection.read(&mut byte);
            match result {
                Ok(0) => {
                    continue;
                }
                Ok(1) => {}
                Ok(s) => {
                    return Err(Error::UnknownError(format!("invalid read length: {}", s)));
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        continue;
                    }
                    return Err(Error::IoError(e));
                }
            }

            let byte = byte[0];

            let state = Self::update_state(self.parser_state.clone(), byte)?;
            if let ParserState::End(body) = state {
                self.parser_state = ParserState::WaitStart;
                return Ok(body);
            }
            self.parser_state = state;
        }
    }

    fn update_state(current: ParserState, byte: u8) -> Result<ParserState> {
        match (current, byte) {
            (ParserState::WaitStart, START_BYTE) => {
                Ok(ParserState::WaitLength)
            }
            (ParserState::WaitStart, b) => {
                Err(Error::MalformedPacketError(format!("failed to get first byte. got {:X}", b)))
            }
            (ParserState::WaitLength, l) => {
                Ok(ParserState::WaitBody(l as usize, Vec::with_capacity(l as usize)))
            }
            (ParserState::WaitBody(len, buf), byte) => {
                let mut buf = buf;
                buf.push(byte);
                if len <= buf.len() + 3 {
                    Ok(ParserState::WaitChecksum(buf, 0))
                } else {
                    Ok(ParserState::WaitBody(len, buf))
                }
            }
            (ParserState::WaitChecksum(body, len), _) => {
                let len = len + 1;
                if CHECKSUM_LENGTH <= len {
                    Ok(ParserState::WaitEnd(body))
                } else {
                    Ok(ParserState::WaitChecksum(body, len))
                }
            }
            (ParserState::WaitEnd(buf), END_BYTE) => {
                Ok(ParserState::End(buf))
            }
            (s, _) => {
                Err(Error::MalformedPacketError(format!("unexpected packet {:X}, state: {:?}", byte, s)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::serial::connection::FlexiConnection;

    #[test]
    fn test_up() {
        let bin: Vec<u8> = vec![0x9b, 0x06, 0x02, 0x00, 0x00, 0x6c, 0xa1, 0x9d];
        let mut conn: FlexiConnection<&[u8]> = FlexiConnection::new(bin.as_ref());
        let body = conn.read_packet().unwrap();
        assert_eq!(vec![0x02, 0x00, 0x00], body)
    }

    #[test]
    fn test_info() {
        let bin: Vec<u8> = vec![0x9B, 0x07, 0x12, 0x07, 0xCF, 0x66, 0xA3, 0xF0, 0x9D];
        let mut conn: FlexiConnection<&[u8]> = FlexiConnection::new(bin.as_ref());
        let body = conn.read_packet().unwrap();
        assert_eq!(vec![0x12, 0x07, 0xCF, 0x66], body)
    }
}
