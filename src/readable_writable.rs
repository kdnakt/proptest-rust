use std::io::{self, Read, Write};

pub trait Readable: Sized {
    fn read(input: &mut impl Read) -> io::Result<Self>;
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
