use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Error, ErrorKind, Read, Result, Write};
use uuid::Uuid;
use varint_rs::{VarintReader, VarintWriter};

pub trait Readable: Sized {
    fn read(input: &mut impl Read) -> io::Result<Self>;

    fn read_ext(
        input: &mut impl Read,
        #[allow(unused)] field_name: &str,
        #[allow(unused)] compact: bool,
    ) -> io::Result<Self> {
        Readable::read(input)
    }
}

pub trait Writable {
    fn write(&self, output: &mut impl Write) -> io::Result<()>;

    fn write_ext(
        &self,
        _output: &mut impl Write,
        #[allow(unused)] _field_name: &str,
        #[allow(unused)] _compact: bool,
    ) -> io::Result<()> {
        self.write(_output)
    }
}

impl Readable for bool {
    fn read(input: &mut impl Read) -> io::Result<Self> {
        input.read_i8().map(|v| v != 0)
    }
}

impl Writable for bool {
    #[inline]
    fn write(&self, output: &mut impl Write) -> io::Result<()> {
        if *self {
            output.write_i8(1)
        } else {
            output.write_i8(0)
        }
    }
}

impl Readable for Uuid {
    fn read(input: &mut impl Read) -> io::Result<Self> {
        input.read_u128::<BigEndian>().map(Uuid::from_u128)
    }
}

impl Writable for Uuid {
    fn write(&self, output: &mut impl Write) -> io::Result<()> {
        output.write_u128::<BigEndian>(self.as_u128())
    }
}

impl Writable for String {
    fn write(&self, output: &mut impl Write) -> io::Result<()> {
        unimplemented!()
    }

    fn write_ext(
        &self,
        output: &mut impl Write,
        field_name: &str,
        compact: bool,
    ) -> io::Result<()> {
        let len = self.len();
        if len > i16::MAX as usize {
            Err(Error::new(
                ErrorKind::Other,
                invalid_len_message(field_name)(len as i64),
            ))
        } else {
            write_len_i16(output, invalid_len_message(field_name), len as i16, compact)?;
            output.write(self.as_bytes()).map(|_| ())
        }
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

impl Writable for Option<String> {
    fn write(&self, _output: &mut impl Write) -> io::Result<()> {
        unimplemented!()
    }

    fn write_ext(
        &self,
        output: &mut impl Write,
        field_name: &str,
        compact: bool,
    ) -> io::Result<()> {
        if let Some(string) = self {
            string.write_ext(output, field_name, compact)
        } else {
            write_len_i16(output, invalid_len_message(field_name), -1, compact)
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
fn read_len_i16(
    input: &mut impl Read,
    invalid_len_message: impl FnOnce(i64) -> String,
    compact: bool,
) -> io::Result<i16> {
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
fn read_len_i32(
    input: &mut impl Read,
    invalid_len_message: impl FnOnce(i64) -> String,
    compact: bool,
) -> io::Result<i32> {
    if compact {
        let len = (input.read_u32_varint()? as i64) - 1;
        if len > i32::MAX as i64 {
            Err(Error::new(ErrorKind::Other, invalid_len_message(len)))
        } else {
            Ok(len as i32)
        }
    } else {
        input.read_i32::<BigEndian>()
    }
}

#[inline]
fn write_len_i16(
    output: &mut impl Write,
    invalid_len_message: impl FnOnce(i64) -> String,
    len: i16,
    compact: bool,
) -> io::Result<()> {
    if len < -1 {
        Err(Error::new(
            ErrorKind::Other,
            invalid_len_message(len as i64),
        ))
    } else {
        if compact {
            output.write_u32_varint((len + 1) as u32)
        } else {
            output.write_i16::<BigEndian>(len)
        }
    }
}

#[inline]
fn write_len_i32(
    output: &mut impl Write,
    invalid_len_message: impl FnOnce(i64) -> String,
    len: i32,
    compact: bool,
) -> io::Result<()> {
    if len < -1 {
        Err(Error::new(
            ErrorKind::Other,
            invalid_len_message(len as i64),
        ))
    } else {
        if compact {
            output.write_u32_varint((len + 1) as u32)
        } else {
            output.write_i32::<BigEndian>(len)
        }
    }
}

#[inline]
fn invalid_len_message(field_name: &str) -> impl FnOnce(i64) -> String {
    let field_name_own = field_name.to_string();
    move |len| format!("string field {field_name_own} had invalid length {len}")
}

pub(crate) fn write_nullable_array<T>(
    output: &mut impl Write,
    field_name: &str,
    array: Option<&[T]>,
    compact: bool,
) -> io::Result<()>
where
    T: Writable,
{
    if let Some(array) = array {
        write_len_i32(output, invalid_len_message(field_name), array.len() as i32, compact)?;
        write_array_inner(output, array, field_name, compact)
    } else {
        write_len_i32(output, invalid_len_message(field_name), -1, compact)
    }
}

fn write_array_inner<T>(
    output: &mut impl Write,
    array: &[T],
    field_name: &str,
    compact: bool,
) -> io::Result<()>
where
    T: Writable,
{
    for item in array {
        item.write_ext(output, field_name, compact)?;
    }
    Ok(())
}

#[inline]
pub(crate) fn read_nullable_array<T>(
    input: &mut impl Read,
    field_name: &str,
    compact: bool,
) -> Result<Option<Vec<T>>>
where
    T: Readable,
{
    let len = read_len_i32(input, invalid_len_message(field_name), compact)?;
    if len < 0 {
        Ok(None)
    } else {
        read_array_inner(input, len, field_name, compact).map(Some)
    }
}

#[inline]
fn read_array_inner<T>(
    input: &mut impl Read,
    len: i32,
    field_name: &str,
    compact: bool,
) -> Result<Vec<T>>
where
    T: Readable,
{
    let mut vec: Vec<T> = Vec::with_capacity(len as usize);
    for _ in 0..len {
        vec.push(T::read_ext(input, field_name, compact)?);
    }
    Ok(vec)
}
