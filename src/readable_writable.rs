use std::io::{self, Read, Write};


pub trait Readable: Sized {
    fn read(input: &mut impl Read) -> io::Result<Self>;
}

pub trait Writable {
    fn write(&self, output: &mut impl Write) -> io::Result<()>;
}
