// https://wiki.vg/NBT

#[allow(unused)]
use std::io::{
    Write, Read,
    BufWriter, BufReader,
    Seek, SeekFrom,
    Cursor,Error,
};

use crate::tag::*;


/// Trait that gives the serialization size of various values.
pub trait NBTSize {
    fn size_in_bytes(&self) -> usize;
}

pub trait NBTRead
where Self: Sized {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, Error>;
}

pub trait NBTWrite {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, Error>;
}

impl NBTSize for String {
    fn size_in_bytes(&self) -> usize {
        2usize + self.len()
    }
}

impl NBTSize for Map {
    fn size_in_bytes(&self) -> usize {
        todo!()
    }
}

impl NBTSize for ListTag {
    fn size_in_bytes(&self) -> usize {
        todo!()
    }
}

pub trait NBTPrimitiveSize {
    const SIZE: usize;
    fn primitive_size_in_bytes() -> usize {
        Self::SIZE
    }
}

impl<T> NBTSize for T
where
    T: NBTPrimitiveSize,
{
    fn size_in_bytes(&self) -> usize {
        Self::SIZE
    }
}

macro_rules! primitive_table {
    ($($($primitive:ty)+ = $size:literal)+) => {
        $(
            $(
                impl NBTPrimitiveSize for $primitive {
                    const SIZE: usize = $size;
                }

                impl NBTSize for Vec<$primitive> {
                    fn size_in_bytes(&self) -> usize {
                        4usize + self.len() * <$primitive as NBTPrimitiveSize>::SIZE
                    }
                }

                impl NBTRead for $primitive {
                    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, Error> {
                        let mut buf = [0u8; Self::SIZE];
                        reader.read_exact(buf.as_mut_slice())?;
                        Ok(Self::from_be_bytes(buf))
                    }
                }

                impl NBTWrite for $primitive {
                    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
                        writer.write(self.to_be_bytes().as_slice())
                    }
                }

            )+
        )+
    };
}

primitive_table![
    i8 u8 = 1
    i16 u16 = 2
    i32 u32 f32 = 4
    i64 u64 f64 = 8
    i128 u128 = 16
];