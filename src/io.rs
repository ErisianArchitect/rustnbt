// https://wiki.vg/NBT
// https://minecraft.fandom.com/wiki/NBT_format

use crate::{
    Map,
    ThisError,
    NbtError,
    mebibytes,
    safe_vec_u8_to_vec_i8,
    tag::{
        Tag,
        TagID,
        ListTag,
        NamedTag,
    },
    family::{
        NonByte,
        Primitive,
        NonBytePrimitive,
    },
    unwrap_block,
    match_origin,
    tag_info_table,
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

/// Trait that gives the serialization size of various values.
pub trait NbtSize {
    /// Returns the serialization size of this data.
    fn nbt_size(&self) -> usize;
}

/// A trait for reading values from readers.
pub trait NbtRead
where
    Self: Sized,
{
    /// Attempt to read a value from a reader.
    fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError>;
}

/// A trait for writing values to writers.
pub trait NbtWrite {
    /// Write a value to a writer.
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError>;
}

/// Blanket implementations for reading and writing primitives (scalar types).
macro_rules! primitive_table {
    ($($primitive:ident $(write = $writer:ident)? $(read = $read:ident)?)+) => {
        $(
            impl NbtRead for $primitive {
                /// Attempts to read primitive from reader. This will read in Big-Endian byte-order.
                fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
                    let mut buf = [0u8; std::mem::size_of::<$primitive>()];
                    reader.read_exact(&mut buf)?;
                    Ok(Self::from_be_bytes(buf))
                }
            }

            impl NbtWrite for $primitive {
                /// Attempts to write primitive to writer. This will write in Big-Endian byte-order.
                fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
                    Ok(writer.write(self.to_be_bytes().as_slice())?)
                }
            }
        )+
    };
}

/// These are the primitive types that will be read and write in Big-Endian order.
primitive_table![
    i8 u8
    i16 u16
    i32 u32 f32
    i64 u64 f64
    i128 u128
];

macro_rules! tag_io {
    ($($id:literal $title:ident $type:path [$subtype:ident] [$origin:ident] [$($impl:path),*] [$($attr:meta),*])+) => {

        pub fn write_named_tag<W: Write>(writer: &mut W, tag: &Tag, name: &str) -> Result<usize, NbtError> {
            match tag {
                $(
                    $(#[$attr])*
                    Tag::$title(tag) => {
                        let id_size = TagID::$title.nbt_write(writer)?;
                        let key_size = name.nbt_write(writer)?;
                        let tag_size = tag.nbt_write(writer)?;
                        Ok(id_size + key_size + tag_size)
                    }
                )+
            }
        }

        pub fn read_named_tag<R: Read>(reader: &mut R) -> Result<(String, Tag), NbtError> {
            let id = TagID::nbt_read(reader)?;
            if matches!(id, TagID::End | TagID::Unsupported) {
                return Err(NbtError::Unsupported);
            }
            let name = String::nbt_read(reader)?;
            let tag = match id {
                $(
                    $(#[$attr])*
                    TagID::$title => {
                        Tag::$title(<$type>::nbt_read(reader)?)
                    }
                )+
                _ => panic!("Impossible state."),
            };
            Ok((name, tag))
        }

        impl NbtSize for Tag {
            fn nbt_size(&self) -> usize {
                match self {
                    $(
                        $(#[$attr])*
                        Tag::$title(tag) => tag.nbt_size(),
                    )+
                }
            }
        }

        impl NbtSize for ListTag {
            fn nbt_size(&self) -> usize {
                match self {
                    $(
                        $(#[$attr])*
                        ListTag::$title(list) => list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5,
                    )+
                    ListTag::Empty => 5,
                }
            }
        }

        impl NbtRead for ListTag {
            fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
                let id = TagID::nbt_read(reader)?;
                Ok(match id {
                    $(
                        $(#[$attr])*
                        TagID::$title => {
                            let length = u32::nbt_read(reader)?;
                            ListTag::$title(
                                read_array(reader, length as usize)?
                            )
                        }
                    )+
                    TagID::End => {
                        u32::nbt_read(reader)?;
                        ListTag::Empty
                    },
                    TagID::Unsupported => return Err(NbtError::Unsupported),
                })
            }
        }

        impl NbtWrite for ListTag {
            fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize,NbtError> {
                match self {
                    $(
                        $(#[$attr])*
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
                            $(#[$attr])*
                            TagID::$title => Tag::$title(<$type>::nbt_read(reader)?),
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

        impl NbtWrite for NamedTag {
            fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
                write_named_tag(writer, &self.tag, &self.name)
            }
        }

        impl NbtRead for NamedTag {
            fn nbt_read<R: Read>(reader: &mut R) -> Result<NamedTag, NbtError> {
                Ok(read_named_tag(reader)?.into())
            }
        }

        impl NbtWrite for Tag {
            fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
                match self {
                    $(
                        $(#[$attr])*
                        Tag::$title(tag) => tag.nbt_write(writer),
                    )+
                }
            }
        }
    };
}

tag_info_table!(tag_io);

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

impl<T: NbtRead + NonByte> NbtRead for Vec<T> {
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

#[cfg(feature = "extensions")]
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

impl NbtWrite for TagID {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        if *self == TagID::Unsupported {
            return Err(NbtError::Unsupported);
        }
        (self.value() as u8).nbt_write(writer)
    }
}

impl NbtWrite for &str {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        let length: u16 = self.len() as u16;
        length.nbt_write(writer)?;
        Ok(writer.write_all(self.as_bytes()).map(|_| self.len() + 2)?)
    }
}

impl NbtWrite for String {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        self.as_str().nbt_write(writer)
    }
}

// This is a special implementation for writing Vectors of types that
// are not u8 or i8.
impl<T: NbtWrite + NonByte> NbtWrite for Vec<T> {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        (self.len() as u32).nbt_write(writer)?;
        write_array(writer, self.as_slice()).map(|size| size + 4)
    }
}

// This is a special implementation for writing Vec<i8>.
// Profiling showed that this was an improvement, so it's what I'm going with.
impl NbtWrite for Vec<i8> {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        (self.len() as u32).nbt_write(writer)?;
        let u8slice: &[u8] = bytemuck::cast_slice(self.as_slice());
        Ok(write_bytes(writer, u8slice)? + 4)
    }
}

// This is a special implementation for writing Vec<u8>.
// Profiling showed that this was an improvement, so it's what I'm going with.
#[cfg(feature = "extensions")]
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
            write_named_tag(writer, tag, key)
                .map(|written| written + size)
        })?;
        TagID::End.nbt_write(writer).map(|size| write_size + size)
    }
}


impl NbtSize for NamedTag {
    fn nbt_size(&self) -> usize {
        self.name.nbt_size() + self.tag.nbt_size() + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::io::*;
    use crate::tag::*;

    fn test_tag() -> Tag {
        let byte = Tag::Byte(i8::MAX);
        let short = Tag::Short(i16::MAX);
        let int = Tag::Int(69420);
        let long = Tag::Long(i64::MAX);
        let float = Tag::Float(3.14_f32);
        let double = Tag::Double(3.14159265358979_f64);
        let bytearray = Tag::ByteArray(vec![1,2,3,4]);
        let string = Tag::String(String::from("The quick brown fox jumps over the lazy dog🎈🎄"));
        let list = Tag::List(ListTag::from(vec![1i32,2,3,4]));
        let intarray = Tag::IntArray(vec![1,1,2,3,5,8,13,21,34,55,89,144]);
        let longarray = Tag::LongArray(vec![1,3,3,7, 1337, 13,37, 1,3,37,1,337, 133,7, 1,33,7,13,3,7]);
        let mut compound = Map::from([
            ("Byte".to_owned(), byte),
            ("Short".to_owned(), short),
            ("Int".to_owned(), int),
            ("Long".to_owned(), long),
            ("Float".to_owned(), float),
            ("Double".to_owned(), double),
            ("ByteArray".to_owned(), bytearray),
            ("String".to_owned(), string),
            ("List".to_owned(), list),
            ("Empty List".to_owned(), Tag::List(ListTag::Empty)),
            ("IntArray".to_owned(), intarray),
            ("LongArray".to_owned(), longarray),
        ]);
        let mapclone = compound.clone();
        compound.insert("Compound".to_owned(), Tag::Compound(mapclone));
        Tag::Compound(compound)
    }

    #[test]
    fn write_test() -> Result<(),NbtError> {
        let tag = test_tag();
        let named = NamedTag::with_name("The quick brown fox jumps over the lazy dog.", tag);
        let mut writer = BufWriter::new(vec![0u8; named.nbt_size()]);
        println!("Size: {}", named.nbt_size());
        let size = named.nbt_write(&mut writer)?;
        println!("Written: {}", size);
        Ok(())
    }

    #[test]
    fn read_test() -> Result<(), NbtError> {
        let file = include_bytes!("../test_nbt.nbt");
        let mut reader = BufReader::new(file.as_slice());
        let named = NamedTag::nbt_read(&mut reader)?;
        println!("Tag: {:#?}", named);
        Ok(())
    }
}