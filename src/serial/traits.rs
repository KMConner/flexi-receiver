use std::io::Read;
use crate::serial::Error;

enum ParserState {
    WaitStart,
    WaitLength,
    WaitBody(usize, Vec<u8>),
    WaitChecksum(Vec<u8>),
    WaitEnd,
}

pub struct FlexiConnection<R: Read> {
    connection: R,
    parser_state: ParserState,
}

impl<R: Read> FlexiConnection<R> {
    fn read_packet(&mut self) -> Result<Vec<u8>> {
        loop {
            let mut byte: [u8; 1] = [0];
            let result = self.connection.read(&mut byte);
            let byte = byte[0]
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
                    return Err(e);
                }
            }

            match (&self.parser_state, byte) {
                (ParserState::WaitStart, 0x9B) => {
                    self.parser_state = ParserState::WaitLength
                }
                (ParserState::WaitStart, _) => {
                    return Err(Error::MalformedPacketError("failed to get first byte".to_string()));
                }
                (ParserState::WaitLength, l) => {
                    self.parser_state = ParserState::WaitBody(l as usize, Vec::new());
                }
                (ParserState::WaitBody(len, buf), byte) => {

                }
            }
        }
    }
}
