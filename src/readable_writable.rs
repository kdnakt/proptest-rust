use std::io::{self, Error, ErrorKind, Read, Write};
use byteorder::{BigEndian, ReadBytesExt};
use varint_rs::VarintReader;
use uuid::Uuid;

pub trait Readable: Sized {
    fn read(input: &mut impl Read) -> io::Result<Self>;

    fn read_ext(input: &mut impl Read,
            #[allow(unused)] field_name: &str,
            #[allow(unused)] compact: bool) -> io::Result<Self> {
        Readable::read(input)
    }
}

pub trait Writable {
    fn write(&self, output: &mut impl Write) -> io::Result<()>;
}

impl Readable for bool {
    fn read(input: &mut impl Read) -> io::Result<Self> {
        let mut buffer = [0u8; 1];
        input.read_exact(&mut buffer)?;
        Ok(buffer[0] != 0)
    }
}

impl Readable for Uuid {
    fn read(input: &mut impl Read) -> io::Result<Self> {
        let mut bytes = [0u8; 16];
        input.read_exact(&mut bytes)?;
        Ok(Uuid::from_bytes(bytes))
    }
}

impl Readable for Option<String> {
    fn read(input: &mut impl Read) -> io::Result<Self> {
        unimplemented!()
    }

    fn read_ext(input: &mut impl Read, field_name: &str, compact: bool) -> io::Result<Self> {
        let len = read_len_i16(input, invalid_len_message(field_name), compact)?;
        if len < 0 {
            Err(Error::new(
                ErrorKind::Other,
                format!("non-nullable field {field_name} was serialized as null"),
            ))
        } else {
            read_string(input, len).map(Some)
        }
    }
}

#[inline]
fn read_string(input: &mut impl Read, str_len: i16) -> io::Result<String> {
    let mut buffer = vec![0u8; str_len as usize];
    input.read_exact(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

#[inline]
fn read_len_i16(input: &mut impl Read, invalid_len_message: impl FnOnce(i64) -> String, compact: bool) -> io::Result<i16> {
    if compact {
        let len = (input.read_u32_varint()? as i64) - 1;
        if len > i16::MAX as i64 {
            Err(Error::new(ErrorKind::Other, invalid_len_message(len)))
        } else {
            Ok(len as i16)
        }
    } else {
        input.read_i16::<BigEndian>()
    }
}

#[inline]
fn invalid_len_message(field_name: &str) -> impl FnOnce(i64) -> String {
    let field_name_own = field_name.to_string();
    move |len| {
        format!("string field {field_name_own} had invalid length {len}")
    }
}
