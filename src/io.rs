use std::io::{
    Write, Read,
    BufWriter, BufReader,
    Seek, SeekFrom,
    Cursor,Error,ErrorKind, ReadBuf
};

pub trait NBTRead {
    fn nbt_read(reader: &mut _) -> Result<Self, Error>;
}

pub trait NBTWrite {
    fn nbt_write(&self, writer: &mut _) -> Result<(), Error>;
}

impl<R: Read> NBTRead for i8 {
    fn nbt_read(reader: &mut R) -> Result<Self, Error> {
        let mut buf: &[u8] = &[0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(i8::from(buf[0]))
    }
}

fn testit() {
    i8::nbt_read(reader)
}