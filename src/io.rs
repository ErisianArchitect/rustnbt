// https://wiki.vg/NBT
// https://minecraft.fandom.com/wiki/NBT_format

#[allow(unused)]
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Seek, SeekFrom, Write};

use crate::{
    tag::*, 
    tag_info_table,
};

fn read_bytes<R: Read>(mut reader: R, length: usize) -> Result<Vec<u8>,Error> {
    let mut buf: Vec<u8> = Vec::new();
    reader.take(length as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_bytes<W: Write>(mut writer: W, data: &[u8]) -> Result<(), Error> {
    writer.write_all(data)
}

fn read_array<R: Read, T: NBTRead>(mut reader: R, length: usize) -> Result<Vec<T>,Error> {
    (0..length)
        .map(|_| {
            T::nbt_read(&mut reader)
        })
        .collect()
}

fn write_array<W: Write, T: NBTWrite>(mut writer: W, data: &[T]) -> Result<usize, Error> {
    data.iter()
        .map(|item| item.nbt_write(&mut writer))
        .sum()
}

/// Trait that gives the serialization size of various values.
pub trait NBTSize {
    fn size_in_bytes(&self) -> usize;
}

impl NBTSize for String {
    fn size_in_bytes(&self) -> usize {
        2usize + self.len()
    }
}

impl NBTSize for Vec<String> {
    /// Returns the size that this would be written as NBT.
    /// It will add 4 to the sum size of the elements, marking
    /// the number of bytes reserved for the length, which is
    /// a requirement to write this to memory.
    fn size_in_bytes(&self) -> usize {
        self.iter().map(|value| {
            value.size_in_bytes()
        })
        .sum::<usize>() + 4usize
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

pub trait NBTRead
where
    Self: Sized,
{
    fn nbt_read<R: Read>(reader: R) -> Result<Self, Error>;
}

impl<T: NBTRead> NBTRead for Vec<T> {
    fn nbt_read<R: Read>(mut reader: R) -> Result<Self, Error> {
        let length = u32::nbt_read(&mut reader)?;
        read_array(reader, length as usize)
    }
}

impl NBTRead for String {
    fn nbt_read<R: Read>(mut reader: R) -> Result<Self, Error> {
        // 🦆 <-- Frank
        // Frank: How does this function work, eh?
        // Me: Well, you see, to read a string in NBT format, we first
        //     need to read a 16-bit unsigned big endian integer, that
        //     signifies our length. We then read that number of bytes
        //     and interpret those bytes as a utf-8 string.
        let length: u16 = u16::nbt_read(&mut reader)?;
        let strbytes = read_bytes(reader, length as usize)?;
        if let Ok(result) = String::from_utf8(strbytes) {
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, "Failed to convert bytes to UTF-8 string."))
        }
    }
}

impl NBTRead for TagID {
    fn nbt_read<R: Read>(mut reader: R) -> Result<Self, Error> {
        Ok(TagID::from(u8::nbt_read(reader)?))
    }
}

pub trait NBTWrite {
    fn nbt_write<W: Write>(&self, writer: W) -> Result<usize, Error>;
}

impl NBTWrite for TagID {
    fn nbt_write<W: Write>(&self, writer: W) -> Result<usize, Error> {
        (self.value() as u8).nbt_write(writer)
    }
}

impl NBTWrite for String {
    fn nbt_write<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
        let length: u16 = self.len() as u16;
        length.nbt_write(&mut writer)?;
        writer.write(self.as_bytes())
    }
}

impl<T: NBTWrite> NBTWrite for Vec<T> {
    fn nbt_write<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
        (self.len() as u32).nbt_write(&mut writer)?;
        write_array(writer, self.as_slice())
            .map(|size| size + 4)
    }
}

impl NBTWrite for Map {
    fn nbt_write<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
        // Writing goes like this:
        // for each key/value pair, write:
        //     TagID of value
        //     name string
        //     Payload
        // After iteration, write TagID::End (0u8)
        let write_size = self.iter().try_fold(0usize, |size, (key, tag)| {
            tag.nbt_write_named(&mut writer, key)
        })?;
        Tag::End.nbt_write(writer).map(|size| write_size + size)
    }
}

macro_rules! primitive_table {
    ($($primitive:ty)+) => {
        $(
            impl NBTSize for $primitive {
                fn size_in_bytes(&self) -> usize {
                    std::mem::size_of::<$primitive>()
                }
            }

            impl NBTSize for Vec<$primitive> {
                fn size_in_bytes(&self) -> usize {
                    self.len() * std::mem::size_of::<$primitive>() + 4
                }
            }

            impl NBTRead for $primitive {
                fn nbt_read<R: Read>(mut reader: R) -> Result<Self, Error> {
                    let mut buf = [0u8; std::mem::size_of::<$primitive>()];
                    reader.read_exact(&mut buf)?;
                    Ok(Self::from_be_bytes(buf))
                }
            }

            impl NBTWrite for $primitive {
                fn nbt_write<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
                    writer.write(self.to_be_bytes().as_slice())
                }
            }
        )+
    };
}

primitive_table![
    i8 u8
    i16 u16
    i32 u32 f32
    i64 u64 f64
    i128 u128
];

macro_rules! tag_io {
    ($($id:literal $title:ident $type_:ty)+) => {


        impl NBTSize for Tag {
            fn size_in_bytes(&self) -> usize {
                match self {
                    $(Tag::$title(tag) => tag.size_in_bytes(),)+
                    Tag::End => 0,
                }
            }
        }

        impl NBTSize for ListTag {
            fn size_in_bytes(&self) -> usize {
                match self {
                    $(ListTag::$title(list) => list.iter().map(|item| item.size_in_bytes()).sum::<usize>() + 5,)+
                    ListTag::End => 5,
                }
            }
        }

        // Complete!
        impl NBTRead for ListTag {
            fn nbt_read<R: Read>(mut reader: R) -> Result<Self, Error> {
                let id = TagID::nbt_read(&mut reader)?;
                Ok(match id {
                    $(
                        TagID::$title => {
                            let length = u32::nbt_read(&mut reader)?;
                            ListTag::$title(read_array(&mut reader, length as usize)?)
                        }
                    )+
                    TagID::End => ListTag::End,
                    TagID::Unsupported => return Err(Error::new(std::io::ErrorKind::Other, "Unsupported tag id encountered in stream.")),
                })
            }
        }

        // Complete!
        impl NBTWrite for ListTag {
            fn nbt_write<W: Write>(&self, mut writer: W) -> Result<usize,Error> {
                match self {
                    $(
                        ListTag::$title(list) => {
                            TagID::$title.nbt_write(&mut writer)?;
                            let length: u32 = list.len() as u32;
                            length.nbt_write(&mut writer)?;
                            list.nbt_write(writer).map(|size| size + 1)
                        }
                    )+
                    ListTag::End => {
                        TagID::End.nbt_write(&mut writer)?;
                        0u32.nbt_write(writer)?;
                        Ok(5)
                    },
                }
            }
        }

        impl NBTRead for Map {
            fn nbt_read<R: Read>(mut reader: R) -> Result<Self, Error> {
                // Reading goes like this:
                // Read TagID
                // if TagID is not End or Unsupported,
                //     Read string for name
                //     Read tag
                //     read next id
                //     repeat until id is End or Unsupported
                let mut map = Map::new();
                let mut id = TagID::nbt_read(&mut reader)?;
                while id != TagID::End {
                    let name = String::nbt_read(&mut reader)?;
                    let tag = match id {
                        $(
                            TagID::$title => Tag::$title(<$type_>::nbt_read(&mut reader)?),
                        )+
                        TagID::Unsupported => return Err(Error::new(std::io::ErrorKind::Other, "Encountered unsuppored TagID in stream.")),
                        TagID::End => panic!("This would not be a valid state, and should be impossible."),
                    };
                    map.insert(name, tag);
                    id = TagID::nbt_read(&mut reader)?;
                }
                Ok(map)
            }
        }

        impl Tag {
            fn nbt_write_named<W: Write, S: Into<String>>(&self, mut writer: W, name: S) -> Result<usize, Error> {
                match self {
                    $(
                        Tag::$title(tag) => {
                            // Ok(
                            //     TagID::$title.nbt_write(&mut writer)? +
                            //     name.into().nbt_write(&mut writer)? +
                            //     tag.nbt_write(writer)?
                            // )
                            let id_size = TagID::$title.nbt_write(&mut writer)?;
                            let key_size = name.into().nbt_write(&mut writer)?;
                            let tag_size = tag.nbt_write(writer)?;
                            Ok(id_size + key_size + tag_size)
                        }
                    )+
                    Tag::End => {
                        Err(Error::new(std::io::ErrorKind::Other, "Cannot write End named tag."))
                    }
                }
            }
        }

        impl NBTWrite for Tag {
            fn nbt_write<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
                match self {
                    $(
                        Tag::$title(tag) => tag.nbt_write(writer),
                    )+
                    // Tag::End wouldn't exactly be valid to write,
                    // so we'll just return Ok(0usize) since it would be
                    // an empty type anyway.
                    Tag::End => Ok(0usize),
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

    fn make_byte() -> Tag {
        Tag::Byte(43)
    }

    #[test]
    fn size_test() {
        let tag = Tag::List(ListTag::from(vec![vec![1,2,3],vec![1,2,3],vec![1,2,3]]));
        assert_eq!(53, tag.size_in_bytes());
    }
}
