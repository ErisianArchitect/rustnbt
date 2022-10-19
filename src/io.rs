// https://wiki.vg/NBT
// https://minecraft.fandom.com/wiki/NBT_format

use crate::{
    Map,
    ThisError,
    tag::{
        Tag,
        TagID,
        ListTag,
        NamedTag,
    },
    family::{
        Not,
        Byte,
        NonByte,
        Primitive,
        NonBytePrimitive,
    },
    tag_info_table
};
use std::{
    io::{
        BufReader,
        BufWriter,
        Cursor,
        Error,
        Read,
        Seek,
        SeekFrom,
        Write,
    },
    ops::Mul,
};

/// This is the Error type returned from NbtRead and NbtWrite operations that fail.
#[derive(ThisError, Debug)]
pub enum NbtError {
    #[error("io error.")]
    IO(#[from] std::io::Error),
    #[error("Failed to read UTF-8 string.")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Unsupported Tag ID.")]
    Unsupported,
}

/// A const function that returns the number of bytes that size kibibytes would be.
const fn kibibytes(size: usize) -> usize {
    size << 10
}

/// A const function that returns the number of bytes that size mebibytes would be.
const fn mebibytes(size: usize) -> usize {
    size << 20
}

/// A const function that returns the number of bytes that size gibibytes would be.
const fn gibibytes(size: usize) -> usize {
    size << 30
}

/// This function converts a Vec<u8> into a Vec<i8> safely using compiler magic.
fn safe_vec_u8_to_vec_i8(v: Vec<u8>) -> Vec<i8> {
    v.into_iter().map(|x| x as i8).collect()
}

/// Reads an exact number of bytes from a reader, returning them as a [Vec].
fn read_bytes<R: Read>(reader: &mut R, length: usize) -> Result<Vec<u8>, NbtError> {
    let mut buf: Vec<u8> = vec![0u8; length];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

/// Writes a byte slice to a writer, returning the number of bytes that were written.
fn write_bytes<W: Write>(writer: &mut W, data: &[u8]) -> Result<usize, NbtError> {
    Ok(writer.write_all(data).map(|_| data.len())?)
}

/// Reads a certain number of elements from a reader.
fn read_array<R, T>(reader: &mut R, length: usize) -> Result<Vec<T>, NbtError>
where
    R: Read,
    T: NbtRead,
{
    (0..length).map(|_| T::nbt_read(reader)).collect()
}

/// Writes elements to a writer, returning the total number of bytes written.
fn write_array<W, T>(writer: &mut W, data: &[T]) -> Result<usize, NbtError>
where
    W: Write,
    T: NbtWrite,
{
    data.iter().map(|item| item.nbt_write(writer)).sum()
}

/// Trait that gives the serialization size of various values.
pub trait NbtSize {
    /// Returns the serialization size of this data.
    fn nbt_size(&self) -> usize;
}

impl<T: Primitive + Sized> NbtSize for T {
    fn nbt_size(&self) -> usize {
        std::mem::size_of::<T>()
    }
}

impl<T: Primitive + Sized> NbtSize for Vec<T> {
    fn nbt_size(&self) -> usize {
        std::mem::size_of::<T>() * self.len() + 4usize
    }
}

impl NbtSize for String {
    fn nbt_size(&self) -> usize {
        /*2 bytes for the length*/ 2usize + self.len()
    }
}

impl NbtSize for Vec<String> {
    /// Returns the size that this would be written as NBT.
    /// It will add 4 to the sum size of the elements, marking
    /// the number of bytes reserved for the length, which is
    /// a requirement to write this to memory.
    fn nbt_size(&self) -> usize {
        self.iter()
            .map(|value| value.nbt_size())
            .sum::<usize>()
            + 4usize
    }
}

impl NbtSize for Map {
    fn nbt_size(&self) -> usize {
        self.iter()
            .map(|(name, tag)| name.nbt_size() + tag.nbt_size() + 1)
            .sum::<usize>()
            + 1
    }
}

impl NbtSize for Vec<Map> {
    fn nbt_size(&self) -> usize {
        self.iter()
            .map(|value| value.nbt_size())
            .sum::<usize>()
            + 4
    }
}

impl NbtSize for Vec<ListTag> {
    fn nbt_size(&self) -> usize {
        self.iter()
            .map(|value| value.nbt_size())
            .sum::<usize>()
            + 4
    }
}

/// A trait for reading values from readers.
pub trait NbtRead
where
    Self: Sized,
{
    /// Attempt to read a value from a reader.
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError>;
}

impl<T: NbtRead + Not<Byte>> NbtRead for Vec<T> {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
        let length = u32::nbt_read(reader)?;
        read_array(reader, length as usize)
    }
}

impl NbtRead for Vec<i8> {
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
        let length = u32::nbt_read(reader)?;
        let bytes = read_bytes(reader, length as usize)?;
        Ok(safe_vec_u8_to_vec_i8(bytes))
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

/// A trait for writing values to writers.
pub trait NbtWrite {
    /// Write a value to a writer.
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

// This is a special implementation for writing Vectors that
// are not u8 or i8.
impl<T: NbtWrite + Not<Byte>> NbtWrite for Vec<T> {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        (self.len() as u32).nbt_write(writer)?;
        write_array(writer, self.as_slice()).map(|size| size + 4)
    }
}

// This is a special implementation
impl NbtWrite for Vec<i8> {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        (self.len() as u32).nbt_write(writer)?;
        // self: Vec<i8>
        let u8slice: &[u8] = unsafe {
            std::slice::from_raw_parts(self.as_slice().as_ptr() as *const u8, self.len())
        };
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
            tag.nbt_write_named(writer, key)
                .map(|written| written + size)
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
    ($($id:literal $title:ident $type_:path $([$($impl:path),*])?)+) => {
        impl NbtSize for Tag {
            fn nbt_size(&self) -> usize {
                match self {
                    $(Tag::$title(tag) => tag.nbt_size(),)+
                }
            }
        }

        impl NbtSize for ListTag {
            fn nbt_size(&self) -> usize {
                match self {
                    $(ListTag::$title(list) => list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5,)+
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
                Ok(Tag::nbt_read_named(reader)?.into())
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
