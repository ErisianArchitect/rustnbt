// https://wiki.vg/NBT

#[allow(unused)]
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Seek, SeekFrom, Write};

use crate::{
    tag::*, 
    table_arm_filter,
    tag_info_table,
};

pub fn read_bytes<R: Read>(reader: &mut R, length: usize) -> Result<Vec<u8>,Error> {
    let mut buf: Vec<u8> = vec![0u8; length];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn write_bytes<W: Write>(writer: &mut W, data: &[u8]) -> Result<(), Error> {
    writer.write_all(data)
}

pub fn read_array<R: Read, T: NBTRead>(reader: &mut R, length: usize) -> Result<Vec<T>,Error> {
    (0..length)
        .map(|_| {
            T::nbt_read(reader)
        })
        .collect()
}

pub fn write_array<W: Write, T: NBTWrite>(writer: &mut W, data: &[T]) -> Result<usize, Error> {
    data.iter().try_fold(4usize, |size, item| {
        item.nbt_write(writer)
            .and_then(|write_size| Ok(size + write_size))
    })
}

/// Trait that gives the serialization size of various values.
pub trait NBTSize {
    fn size_in_bytes(&self) -> usize;
}

pub trait NBTRead
where
    Self: Sized,
{
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, Error>;
}

impl NBTRead for String {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let bytes = read_bytes(reader, u16::nbt_read(reader)? as usize)?;
        if let Ok(result) = String::from_utf8(bytes) {
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, "Failed to convert to UTF-8 string."))
        }
    }
}

pub trait NBTWrite {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, Error>;
}

impl NBTWrite for String {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let length: u16 = self.len() as u16;
        length.nbt_write(writer)?;
        todo!()
    }
}

impl NBTSize for String {
    fn size_in_bytes(&self) -> usize {
        2usize + self.len()
    }
}

impl NBTRead for String {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        // 🦆 <-- Frank
        // Frank: How does this function work, eh?
        // Me: Well, you see, to read a string in NBT format, we first
        //     need to read a 16-bit unsigned big endian integer, that
        //     signifies our length. We then read that number of bytes
        //     and interpret those bytes as a utf-8 string.
        let length: u16 = u16::nbt_read(reader)?;
        let strbytes = read_bytes(reader, length as usize)?;
        if let Ok(result) = String::from_utf8(strbytes) {
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, "Failed to convert bytes to UTF-8 string."))
        }
    }
}

impl NBTSize for Vec<String> {
    fn size_in_bytes(&self) -> usize {
        self.iter().map(|value| {
            value.size_in_bytes()
        })
        .sum::<usize>() + 4
    }
}

impl NBTSize for Map {
    fn size_in_bytes(&self) -> usize {
        self.iter().map(|(name, tag)| {
                name.size_in_bytes() + tag.size_in_bytes() + 1
            })
            .sum::<usize>() + 1
    }
}

impl NBTSize for Vec<Map> {
    fn size_in_bytes(&self) -> usize {
        self.iter().map(|value| {
            value.size_in_bytes()
        })
        .sum::<usize>() + 4
    }
}

impl NBTSize for Vec<ListTag> {
    fn size_in_bytes(&self) -> usize {
        self.iter().map(|value| {
            value.size_in_bytes()
        })
        .sum::<usize>() + 4
    }
}

macro_rules! primitive_table {
    ($($($primitive:ty)+ = $size:literal)+) => {
        $(
            $(

                impl NBTSize for $primitive {
                    fn size_in_bytes(&self) -> usize {
                        $size
                    }
                }

                impl NBTSize for Vec<$primitive> {
                    fn size_in_bytes(&self) -> usize {
                        self.len() * $size + 4
                    }
                }

                impl NBTRead for $primitive {
                    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, Error> {
                        let mut buf = [0u8; $size];
                        reader.read_exact(&mut buf)?;
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

macro_rules! tag_io {
    ($($id:literal $title:ident $($type_:ty)?)+) => {

        impl NBTSize for Tag {
            fn size_in_bytes(&self) -> usize {
                match self {
                    $(table_arm_filter!( $($type_)? : { Tag::$title(item)    } else { Tag::$title } )
                    =>table_arm_filter!( $($type_)? : { item.size_in_bytes() } else { 0           } ),)+
                }
            }
        }

        impl NBTSize for ListTag {
            fn size_in_bytes(&self) -> usize {
                match self {
                    $(table_arm_filter!($($type_)? : { ListTag::$title(arr)                                           } else { ListTag::$title } )
                    =>table_arm_filter!($($type_)? : { arr.iter().map(|item| item.size_in_bytes()).sum::<usize>() + 5 } else { 5               } ),)+
                }
            }
        }
    };
}

tag_info_table!(tag_io);
//include!("table.rs");
#[cfg(test)]
mod tests {
    #![allow(unused)]

    use crate::tag::*;
    use super::*;
    #[test]
    fn size_test() {
        let tag = Tag::List(ListTag::from(vec![vec![1,2,3],vec![1,2,3],vec![1,2,3]]));
        assert_eq!(53, tag.size_in_bytes());
    }
}
