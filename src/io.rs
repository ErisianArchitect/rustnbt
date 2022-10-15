// https://wiki.vg/NBT
// https://minecraft.fandom.com/wiki/NBT_format

#[allow(unused)]
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Seek, SeekFrom, Write};
use std::ops::Mul;
use crate::family::*;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum NbtError {
    #[error("io error.")]
    IO(#[from] std::io::Error),
    #[error("Failed to read UTF-8 string.")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Unsupported Tag ID.")]
    Unsupported,
}

use crate::{
    tag::*, 
    tag_info_table,
};


/// gets Kibibytes
const fn kibibytes(size: usize) -> usize {
    size * 1024
}

const fn mebibytes(size: usize) -> usize {
    kibibytes(kibibytes(1)) * size
}

const fn gibibytes(size: usize) -> usize {
    mebibytes(kibibytes(1)) * size
}

fn vec_u8_to_vec_i8(v: Vec<u8>) -> Vec<i8> {
    let mut v = std::mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}

fn vec_i8_to_vec_u8(v: Vec<i8>) -> Vec<u8> {
    let mut v = std::mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
}

fn read_bytes<R: Read>(reader: &mut R, length: usize) -> Result<Vec<u8>,NbtError> {
    let mut buf: Vec<u8> = Vec::new();
    // let mut buf: Vec<u8> = Vec::with_capacity(length);
    reader.take(length as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_bytes<W: Write>(writer: &mut W, data: &[u8]) -> Result<usize, NbtError> {
    Ok(writer.write_all(data).map(|_| data.len())?)
}


fn read_array<R, T>(reader: &mut R, length: usize) -> Result<Vec<T>,NbtError>
where
    R: Read,
    T: NbtRead {
    (0..length)
        .map(|_| {
            T::nbt_read(reader)
        })
        .collect()
}

fn write_array<W, T>(writer: &mut W, data: &[T]) -> Result<usize, NbtError>
where
    W: Write,
    T: NbtWrite {
        data.iter()
            .map(|item| item.nbt_write(writer))
            .sum()
}

/// Trait that gives the serialization size of various values.
pub trait NbtSize {
    fn size_in_bytes(&self) -> usize;
}

impl<T: Primitive + Sized> NbtSize for T {
    fn size_in_bytes(&self) -> usize {
        std::mem::size_of::<T>()
    }
}

impl<T: Primitive + Sized> NbtSize for Vec<T> {
    fn size_in_bytes(&self) -> usize {
        std::mem::size_of::<T>() * self.len() + 4usize
    }
}

impl NbtSize for String {
    fn size_in_bytes(&self) -> usize {
        2usize + self.len()
    }
}

impl NbtSize for Vec<String> {
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

impl NbtSize for Map {
    fn size_in_bytes(&self) -> usize {
        self.iter().map(|(name, tag)| {
                name.size_in_bytes() + tag.size_in_bytes() + 1
            })
            .sum::<usize>() + 1
    }
}

impl NbtSize for Vec<Map> {
    fn size_in_bytes(&self) -> usize {
        self.iter().map(|value| {
            value.size_in_bytes()
        })
        .sum::<usize>() + 4
    }
}

impl NbtSize for Vec<ListTag> {
    fn size_in_bytes(&self) -> usize {
        self.iter().map(|value| {
            value.size_in_bytes()
        })
        .sum::<usize>() + 4
    }
}

pub trait NbtRead
where
    Self: Sized,
{
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError>;
}

impl<T: NbtRead + Not<byte>> NbtRead for Vec<T> {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
        let length = u32::nbt_read(reader)?;
        read_array(reader, length as usize)
    }
}

impl NbtRead for Vec<i8> {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
        let length = u32::nbt_read(reader)?;
        let bytes = read_bytes(reader, length as usize)?;
        Ok(vec_u8_to_vec_i8(bytes))
    }
}

impl NbtRead for Vec<u8> {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
        let length = u32::nbt_read(reader)?;
        read_bytes(reader, length as usize)
    }
}

impl NbtRead for String {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
        // 🦆 <-- Frank
        // Frank: How does this function work, eh?
        // Me: Well, you see, to read a string in NBT format, we first
        //     need to read a 16-bit unsigned big endian integer, that
        //     signifies our length. We then read that number of bytes
        //     and interpret those bytes as a utf-8 string.
        let length: u16 = u16::nbt_read(reader)?;
        let strbytes = read_bytes(reader, length as usize)?;
        Ok(String::from_utf8(strbytes)?)
    }
}

impl NbtRead for TagID {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
        Ok(TagID::from(u8::nbt_read(reader)?))
    }
}

pub trait NbtWrite {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError>;
}

impl NbtWrite for TagID {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        if *self == TagID::Unsupported {
            return Err(NbtError::Unsupported);
        }
        (self.value() as u8).nbt_write(writer)
    }
}

impl NbtWrite for String {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        let length: u16 = self.len() as u16;
        length.nbt_write(writer)?;
        Ok(writer.write_all(self.as_bytes()).map(|_| self.len() + 2)?)
    }
}

impl<T: NbtWrite + Not<byte>> NbtWrite for Vec<T> {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        (self.len() as u32).nbt_write(writer)?;
        write_array(writer, self.as_slice())
            .map(|size| size + 4)
    }
}

impl NbtWrite for Vec<i8> {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        (self.len() as u32).nbt_write(writer)?;
        let u8slice: &[u8] = unsafe { std::slice::from_raw_parts(self.as_slice().as_ptr() as *const u8, self.len()) };
        Ok(write_bytes(writer, u8slice)? + 4)
    }
}

impl NbtWrite for Vec<u8> {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        (self.len() as u32).nbt_write(writer)?;
        Ok(write_bytes(writer, &self)? + 4)
    }
}

impl NbtWrite for Map {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        // Writing goes like this:
        // for each key/value pair, write:
        //     TagID of value
        //     name string
        //     Payload
        // After iteration, write TagID::End (0u8)
        let write_size = self.iter().try_fold(0usize, |size, (key, tag)| {
            tag.nbt_write_named(writer, key).map(|written| written + size)
        })?;
        TagID::End.nbt_write(writer).map(|size| write_size + size)
    }
}

macro_rules! primitive_table {
    ($($primitive:ident $(write = $writer:ident)? $(read = $read:ident)?)+) => {
        $(

            impl NbtRead for $primitive {
                fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
                    let mut buf = [0u8; std::mem::size_of::<$primitive>()];
                    reader.read_exact(&mut buf)?;
                    Ok(Self::from_be_bytes(buf))
                }
            }

            impl NbtWrite for $primitive {
                fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
                    Ok(writer.write(self.to_be_bytes().as_slice())?)
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
    ($($id:literal $title:ident $type_:ty $([$($impl:path),*])?)+) => {


        impl NbtSize for Tag {
            fn size_in_bytes(&self) -> usize {
                match self {
                    $(Tag::$title(tag) => tag.size_in_bytes(),)+
                }
            }
        }

        impl NbtSize for ListTag {
            fn size_in_bytes(&self) -> usize {
                match self {
                    $(ListTag::$title(list) => list.iter().map(|item| item.size_in_bytes()).sum::<usize>() + 5,)+
                    ListTag::Empty => 5,
                }
            }
        }

        // Complete!
        impl NbtRead for ListTag {
            fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
                let id = TagID::nbt_read(reader)?;
                Ok(match id {
                    $(
                        TagID::$title => {
                            let length = u32::nbt_read(reader)?;
                            ListTag::$title(
                                read_array(reader, length as usize)?
                            )
                        }
                    )+
                    TagID::End => ListTag::Empty,
                    TagID::Unsupported => return Err(NbtError::Unsupported),
                })
            }
        }

        // Complete!
        impl NbtWrite for ListTag {
            fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize,NbtError> {
                match self {
                    $(
                        ListTag::$title(list) => {
                            TagID::$title.nbt_write(writer)?;
                            list.nbt_write(writer).map(|size| size + 1)
                        }
                    )+
                    ListTag::Empty => {
                        TagID::End.nbt_write(writer)?;
                        0u32.nbt_write(writer)?;
                        Ok(5)
                    },
                }
            }
        }

        impl NbtRead for Map {
            fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
                // Reading goes like this:
                // Read TagID
                // if TagID is not End or Unsupported,
                //     Read string for name
                //     Read tag
                //     read next id
                //     repeat until id is End or Unsupported
                let mut map = Map::new();
                let mut id = TagID::nbt_read(reader)?;
                while id != TagID::End {
                    let name = String::nbt_read(reader)?;
                    let tag = match id {
                        $(
                            TagID::$title => Tag::$title(<$type_>::nbt_read(reader)?),
                        )+
                        TagID::Unsupported => return Err(NbtError::Unsupported),
                        TagID::End => panic!("This would not be a valid state, and should be impossible."),
                    };
                    map.insert(name, tag);
                    id = TagID::nbt_read(reader)?;
                }
                Ok(map)
            }
        }

        impl Tag {
            fn nbt_write_named<W: Write>(&self, writer: &mut W, name: &String) -> Result<usize, NbtError> {
                match self {
                    $(
                        Tag::$title(tag) => {
                            let id_size = TagID::$title.nbt_write(writer)?;
                            let key_size = name.nbt_write(writer)?;
                            let tag_size = tag.nbt_write(writer)?;
                            Ok(id_size + key_size + tag_size)
                        }
                    )+
                }
            }

            fn nbt_read_named<R: Read>(reader: &mut R) -> Result<(String, Tag), NbtError> {
                let id = TagID::nbt_read(reader)?;
                if matches!(id, TagID::End | TagID::Unsupported) {
                    return Err(NbtError::Unsupported);
                }
                let name = String::nbt_read(reader)?;
                let tag = match id {
                    $(
                        TagID::$title => {
                            Tag::$title(<$type_>::nbt_read(reader)?)
                        }
                    )+
                    _ => unreachable!("Impossible state."),
                };
                Ok((name, tag))
            }
        }

        impl NbtWrite for NamedTag {
            fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
                Ok(self.tag.nbt_write_named(writer, &self.name)?)
            }
        }

        impl NbtRead for NamedTag {
            fn nbt_read<R: Read>(reader: &mut R) -> Result<NamedTag, NbtError> {
                let id = TagID::nbt_read(reader)?;
                if matches!(id, TagID::End | TagID::Unsupported) {
                    return Err(NbtError::Unsupported);
                }
                let name = String::nbt_read(reader)?;
                let tag = match id {
                    $(
                        TagID::$title => {
                            Tag::$title(NbtRead::nbt_read(reader)?)
                        }
                    )+
                    _ => unreachable!("Impossible state."),
                };
                Ok(
                    NamedTag {
                        name,
                        tag,
                    }
                )
            }
        }

        impl NbtWrite for Tag {
            fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
                match self {
                    $(
                        Tag::$title(tag) => tag.nbt_write(writer),
                    )+
                }
            }
        }

    };
}

tag_info_table!(tag_io);
