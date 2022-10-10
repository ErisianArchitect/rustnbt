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