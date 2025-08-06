use std::{io, vec};

use bytes::Bytes;

use crate::frame::Frame;

// parse commands
pub(crate) struct Parse {
    parts: vec::IntoIter<Box<Frame>>,
}

pub(crate) enum ParseError {
    EndOfStream,
    UnknownCommand(String),
    Invalid,
}

impl Parse {
    pub(crate) fn new(frame: Frame) -> Result<Parse, ParseError> {
        let array = match frame {
            Frame::Array(array) => array,
            _ => return Err(ParseError::Invalid),
        };
        Ok(Parse {
            parts: array.into_iter(),
        })
    }

    pub(crate) fn next(&mut self) -> Result<Frame, ParseError> {
        self.parts
            .next()
            .map(|frame| *frame)
            .ok_or(ParseError::EndOfStream)
    }

    pub(crate) fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(bytes) => str::from_utf8(&bytes[..])
                .map(|c| c.to_string())
                .map_err(|_| ParseError::Invalid),
            _ => Err(ParseError::Invalid),
        }
    }

    pub(crate) fn next_bytes(&mut self) -> Result<Bytes, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Frame::Bulk(bytes) => Ok(bytes),
            _ => Err(ParseError::Invalid),
        }
    }

    pub(crate) fn next_int(&mut self) -> Result<u64, ParseError> {
        match self.next()? {
            Frame::Integer(int) => Ok(int),
            _ => Err(ParseError::Invalid),
        }
    }

    pub(crate) fn finish(&mut self) -> Result<(), ParseError> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err(ParseError::Invalid)
        }
    }
}

impl From<ParseError> for io::Error {
    fn from(value: ParseError) -> Self {
        use ParseError::*;

        io::Error::new(
            io::ErrorKind::Other,
            match value {
                EndOfStream => "ERROR: end of stream".to_string(),
                Invalid => "ERROR: invalid".to_string(),
                UnknownCommand(cmd) => format!("ERROR: unknown command {cmd}"),
            },
        )
    }
}
