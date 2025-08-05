use std::io::Cursor;

use bytes::{Buf, Bytes};

pub(crate) enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Box<Frame>>),
}

pub(crate) enum Error {
    Incomplete,
    Invalid,
}

impl Frame {
    pub(crate) fn array() -> Frame {
        Frame::Array(vec![])
    }

    pub(crate) fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Frame::Array(vec) => 
                vec.push(Box::new(Frame::Bulk(bytes)))
            _ => panic!("Not an Array Frame")
        }
    }

    // check if an entire message can be decoded
    pub(crate) fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {}
}

fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    Ok(src.bytes()[0])
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    Ok(src.get_u8())
}

fn skip(src: &mut Cursor<&[u8]>, cnt: usize) -> Result<(), Error> {
    if src.remaining() < cnt {
        return Err(Error::Incomplete);
    }
    src.advance(cnt);
    Ok(())
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    use atoi::atoi;

    let line = src.get_line()?;


}


