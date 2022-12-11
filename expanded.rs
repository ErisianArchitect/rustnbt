#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod family {
    /// Marks a type as neither i8 or u8.
    pub trait NonByte {}
    /// Marks a type as a primitive (scalar types such as integers or floating point numbers)
    pub trait Primitive {}
    /// Marks a type as both NonByte and Primitive.
    pub trait NonBytePrimitive {}
    impl<T: NonBytePrimitive> NonByte for T {}
    impl<T: NonBytePrimitive> Primitive for T {}
}
pub mod io {
    use crate::{
        Map, NbtError, tag::{Tag, TagID, ListTag, NamedTag},
        family::*, tag_info_table,
    };
    use std::io::{Read, Write};
    /// Trait that gives the serialization size in bytes of various values.
    /// This size may include a 2 or 4 byte length, or a single byte end marker in addition to the payload.
    pub trait NbtSize {
        /// Returns the serialization size of this data.
        fn nbt_size(&self) -> usize;
    }
    /// Trait applied to all readers for NBT extensions.
    pub trait ReadNbt: Read {
        /// Read NBT (anything that implements NbtRead).
        fn read_nbt<T: NbtRead>(&mut self) -> Result<T, NbtError>;
    }
    impl<Reader: Read> ReadNbt for Reader {
        /// Read NBT (anything that implements NbtRead).
        fn read_nbt<T: NbtRead>(&mut self) -> Result<T, NbtError> {
            T::nbt_read(self)
        }
    }
    /// Trait applied to all writers for NBT extensions.
    pub trait WriteNbt: Write {
        /// Write NBT (anything that implements NbtWrite).
        fn write_nbt<T: NbtWrite>(&mut self, value: &T) -> Result<usize, NbtError>;
    }
    impl<Writer: Write> WriteNbt for Writer {
        /// Write NBT (anything that implements NbtWrite).
        fn write_nbt<T: NbtWrite>(&mut self, value: &T) -> Result<usize, NbtError> {
            value.nbt_write(self)
        }
    }
    /// A trait for reading values from readers.
    /// Minecraft's NBT format demands that values are read in Big-Endian byteorder, so
    /// that means that it is pertinent to implement custom readers for those types.
    /// By applying [NbtRead] to all the types that can be represented with NBT, we
    /// are able to deserialize NBT data with greater ease.
    /// Although this trait is public, it is not intended for public API usage.
    pub trait NbtRead: Sized {
        /// Attempt to read a value from a reader.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError>;
    }
    /// A trait for writing values to writers.
    /// Minecraft's NBT format demands that values are read in Big-Endian byteorder, so
    /// that means that it is pertinent to implement custom writers for those types.
    /// By applying [NbtWrite] to all types that can be represented with NBT, we
    /// are able to deserialize NBT data with greater ease.
    /// Although this trait is public, is is not intended for public API usage.
    pub trait NbtWrite {
        /// Write a value to a writer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError>;
    }
    impl NbtRead for i8 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<i8>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for i8 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for u8 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<u8>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for u8 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for i16 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<i16>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for i16 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for u16 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<u16>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for u16 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for i32 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<i32>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for i32 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for u32 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<u32>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for u32 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for f32 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<f32>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for f32 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for i64 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<i64>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for i64 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for u64 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<u64>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for u64 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for f64 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<f64>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for f64 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for i128 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<i128>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for i128 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    impl NbtRead for u128 {
        ///Attempts to read primitive from reader. This will read in Big-Endian byte-order.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut buf = [0u8; std::mem::size_of::<u128>()];
            reader.read_exact(&mut buf)?;
            Ok(Self::from_be_bytes(buf))
        }
    }
    impl NbtWrite for u128 {
        ///Attempts to write primitive to writer. This will write in Big-Endian byte-order.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            Ok(writer.write(self.to_be_bytes().as_slice())?)
        }
    }
    /**
		This function is the bread and butter of serialization of NBT data.<br>
		This function will write the [Tag]'s ID, the provided [Tag] Name, and then the tag itself.
		This is necessary for writing Compound (HashMap) tags.
		This is also how the root tag of an NBT file is written.
		*/
    pub fn write_named_tag<W: Write, S: AsRef<str>>(
        writer: &mut W,
        tag: &Tag,
        name: S,
    ) -> Result<usize, NbtError> {
        let id = tag.id();
        id.nbt_write(writer)?;
        let key_size = name.as_ref().nbt_write(writer)?;
        match tag {
            Tag::Byte(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::Short(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::Int(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::Long(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::Float(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::Double(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::ByteArray(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::String(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::List(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::Compound(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::IntArray(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
            Tag::LongArray(data) => {
                let tag_size = data.nbt_write(writer)?;
                Ok(key_size + tag_size + 1)
            }
        }
    }
    /**
		Like [write_named_tag], this function is crucial to deserialization of NBT data.
		This function will first read a byte representing the [Tag] ID.
		It will then verify that the [Tag] ID is valid (can't be 0, and must match one of the Tag IDs).
		After verifying that the [Tag] ID is valid, it will read the name of the tag.
		After reading the name, it will read the tag itself, using the [Tag] ID that was read to
		determine which [Tag] type to read. Typically this will be a Compound tag (ID: 10), or a List tag (ID: 9).
		There is no restriction on what type this tag can be, though.
		*/
    pub fn read_named_tag<R: Read>(reader: &mut R) -> Result<(String, Tag), NbtError> {
        let id = TagID::nbt_read(reader)?;
        let name = String::nbt_read(reader)?;
        let tag = match id {
            TagID::Byte => Tag::Byte(<i8>::nbt_read(reader)?),
            TagID::Short => Tag::Short(<i16>::nbt_read(reader)?),
            TagID::Int => Tag::Int(<i32>::nbt_read(reader)?),
            TagID::Long => Tag::Long(<i64>::nbt_read(reader)?),
            TagID::Float => Tag::Float(<f32>::nbt_read(reader)?),
            TagID::Double => Tag::Double(<f64>::nbt_read(reader)?),
            TagID::ByteArray => Tag::ByteArray(<std::vec::Vec<i8>>::nbt_read(reader)?),
            TagID::String => Tag::String(<std::string::String>::nbt_read(reader)?),
            TagID::List => Tag::List(<crate::tag::ListTag>::nbt_read(reader)?),
            TagID::Compound => Tag::Compound(<crate::Map>::nbt_read(reader)?),
            TagID::IntArray => Tag::IntArray(<std::vec::Vec<i32>>::nbt_read(reader)?),
            TagID::LongArray => Tag::LongArray(<std::vec::Vec<i64>>::nbt_read(reader)?),
        };
        Ok((name, tag))
    }
    impl NbtSize for Tag {
        ///Get the number of bytes that this data will serialize to.
        fn nbt_size(&self) -> usize {
            match self {
                Tag::Byte(data) => data.nbt_size(),
                Tag::Short(data) => data.nbt_size(),
                Tag::Int(data) => data.nbt_size(),
                Tag::Long(data) => data.nbt_size(),
                Tag::Float(data) => data.nbt_size(),
                Tag::Double(data) => data.nbt_size(),
                Tag::ByteArray(data) => data.nbt_size(),
                Tag::String(data) => data.nbt_size(),
                Tag::List(data) => data.nbt_size(),
                Tag::Compound(data) => data.nbt_size(),
                Tag::IntArray(data) => data.nbt_size(),
                Tag::LongArray(data) => data.nbt_size(),
            }
        }
    }
    impl NbtSize for ListTag {
        ///Get the number of bytes that this data will serialize to.
        fn nbt_size(&self) -> usize {
            match self {
                ListTag::Byte(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::Short(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::Int(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::Long(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::Float(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::Double(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::ByteArray(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::String(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::List(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::Compound(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::IntArray(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::LongArray(list) => {
                    list.iter().map(|item| item.nbt_size()).sum::<usize>() + 5
                }
                ListTag::Empty => 5,
            }
        }
    }
    impl NbtRead for ListTag {
        ///Attempt to read a [ListTag] from a reader.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let id = TagID::nbt_read(reader);
            if match id {
                Err(crate::NbtError::End) => true,
                _ => false,
            } {
                u32::nbt_read(reader)?;
                return Ok(ListTag::Empty);
            }
            match id {
                Ok(TagID::Byte) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::Byte(read_array(reader, length as usize)?))
                }
                Ok(TagID::Short) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::Short(read_array(reader, length as usize)?))
                }
                Ok(TagID::Int) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::Int(read_array(reader, length as usize)?))
                }
                Ok(TagID::Long) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::Long(read_array(reader, length as usize)?))
                }
                Ok(TagID::Float) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::Float(read_array(reader, length as usize)?))
                }
                Ok(TagID::Double) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::Double(read_array(reader, length as usize)?))
                }
                Ok(TagID::ByteArray) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::ByteArray(read_array(reader, length as usize)?))
                }
                Ok(TagID::String) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::String(read_array(reader, length as usize)?))
                }
                Ok(TagID::List) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::List(read_array(reader, length as usize)?))
                }
                Ok(TagID::Compound) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::Compound(read_array(reader, length as usize)?))
                }
                Ok(TagID::IntArray) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::IntArray(read_array(reader, length as usize)?))
                }
                Ok(TagID::LongArray) => {
                    let length = u32::nbt_read(reader)?;
                    Ok(ListTag::LongArray(read_array(reader, length as usize)?))
                }
                Err(crate::NbtError::End) => {
                    u32::nbt_read(reader)?;
                    Ok(ListTag::Empty)
                }
                Err(err) => Err(err),
            }
        }
    }
    impl NbtWrite for ListTag {
        ///Attmept to write a [ListTag] to a writer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            match self {
                ListTag::Byte(list) => {
                    TagID::Byte.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::Short(list) => {
                    TagID::Short.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::Int(list) => {
                    TagID::Int.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::Long(list) => {
                    TagID::Long.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::Float(list) => {
                    TagID::Float.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::Double(list) => {
                    TagID::Double.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::ByteArray(list) => {
                    TagID::ByteArray.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::String(list) => {
                    TagID::String.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::List(list) => {
                    TagID::List.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::Compound(list) => {
                    TagID::Compound.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::IntArray(list) => {
                    TagID::IntArray.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::LongArray(list) => {
                    TagID::LongArray.nbt_write(writer)?;
                    list.nbt_write(writer).map(|size| size + 1)
                }
                ListTag::Empty => {
                    0u8.nbt_write(writer)?;
                    0u32.nbt_write(writer)?;
                    Ok(5)
                }
            }
        }
    }
    impl NbtRead for Map {
        ///Attempt to read a [Map] from a reader.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let mut map = Map::new();
            let mut id = TagID::nbt_read(reader);
            while !match id {
                Err(crate::NbtError::End) => true,
                _ => false,
            } {
                let name = String::nbt_read(reader)?;
                let tag = match id {
                    Ok(TagID::Byte) => Tag::Byte(<i8>::nbt_read(reader)?),
                    Ok(TagID::Short) => Tag::Short(<i16>::nbt_read(reader)?),
                    Ok(TagID::Int) => Tag::Int(<i32>::nbt_read(reader)?),
                    Ok(TagID::Long) => Tag::Long(<i64>::nbt_read(reader)?),
                    Ok(TagID::Float) => Tag::Float(<f32>::nbt_read(reader)?),
                    Ok(TagID::Double) => Tag::Double(<f64>::nbt_read(reader)?),
                    Ok(TagID::ByteArray) => {
                        Tag::ByteArray(<std::vec::Vec<i8>>::nbt_read(reader)?)
                    }
                    Ok(TagID::String) => {
                        Tag::String(<std::string::String>::nbt_read(reader)?)
                    }
                    Ok(TagID::List) => {
                        Tag::List(<crate::tag::ListTag>::nbt_read(reader)?)
                    }
                    Ok(TagID::Compound) => Tag::Compound(<crate::Map>::nbt_read(reader)?),
                    Ok(TagID::IntArray) => {
                        Tag::IntArray(<std::vec::Vec<i32>>::nbt_read(reader)?)
                    }
                    Ok(TagID::LongArray) => {
                        Tag::LongArray(<std::vec::Vec<i64>>::nbt_read(reader)?)
                    }
                    Err(err) => return Err(err),
                };
                map.insert(name, tag);
                id = TagID::nbt_read(reader);
            }
            Ok(map)
        }
    }
    impl NbtWrite for Tag {
        ///Attempt to write a [Tag]
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            match self {
                Tag::Byte(tag) => tag.nbt_write(writer),
                Tag::Short(tag) => tag.nbt_write(writer),
                Tag::Int(tag) => tag.nbt_write(writer),
                Tag::Long(tag) => tag.nbt_write(writer),
                Tag::Float(tag) => tag.nbt_write(writer),
                Tag::Double(tag) => tag.nbt_write(writer),
                Tag::ByteArray(tag) => tag.nbt_write(writer),
                Tag::String(tag) => tag.nbt_write(writer),
                Tag::List(tag) => tag.nbt_write(writer),
                Tag::Compound(tag) => tag.nbt_write(writer),
                Tag::IntArray(tag) => tag.nbt_write(writer),
                Tag::LongArray(tag) => tag.nbt_write(writer),
            }
        }
    }
    /// Reads an exact number of bytes from a reader, returning them as a [Vec].
    fn read_bytes<R: Read>(reader: &mut R, length: usize) -> Result<Vec<u8>, NbtError> {
        let mut buf: Vec<u8> = ::alloc::vec::from_elem(0u8, length);
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
        /// Get the number of bytes that this data will serialize to.
        fn nbt_size(&self) -> usize {
            std::mem::size_of::<T>()
        }
    }
    impl<T: Primitive + Sized> NbtSize for Vec<T> {
        /// Get the number of bytes that this data will serialize to.
        fn nbt_size(&self) -> usize {
            std::mem::size_of::<T>() * self.len() + 4usize
        }
    }
    impl NbtSize for String {
        /// Get the number of bytes that this data will serialize to.
        fn nbt_size(&self) -> usize {
            2usize + self.len()
        }
    }
    impl NbtSize for Vec<String> {
        /// Returns the size that this would be written as NBT.
        /// It will add 4 to the sum size of the elements, marking
        /// the number of bytes reserved for the length, which is
        /// a requirement to write this to memory.
        fn nbt_size(&self) -> usize {
            self.iter().map(|value| value.nbt_size()).sum::<usize>() + 4
        }
    }
    impl NbtSize for Map {
        /// Get the serialization size in bytes.
        /// This will determine the total serialization size of this data when written to a writer.
        fn nbt_size(&self) -> usize {
            self
                .iter()
                .map(|(name, tag)| name.nbt_size() + tag.nbt_size() + 1)
                .sum::<usize>() + 1
        }
    }
    impl NbtSize for Vec<Map> {
        /// Get the serialization size in bytes.
        /// The length of the [Vec] is part of serialization, which adds 4 bytes to the total size.
        fn nbt_size(&self) -> usize {
            self.iter().map(|value| value.nbt_size()).sum::<usize>() + 4
        }
    }
    impl NbtSize for Vec<ListTag> {
        /// Get the serialization size in bytes.
        /// The length of the [ListTag] is part of serialization, which adds 4 bytes to the total size.
        fn nbt_size(&self) -> usize {
            self.iter().map(|value| value.nbt_size()).sum::<usize>() + 4
        }
    }
    impl<S: From<String>, T: From<Tag>> NbtRead for (S, T) {
        /// For reading a named tag straight into a Tuple.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let (name, tag) = read_named_tag(reader)?;
            Ok((S::from(name), T::from(tag)))
        }
    }
    impl<T: NbtRead + NonByte> NbtRead for Vec<T> {
        /// Read a [Vec] from a reader.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let length = u32::nbt_read(reader)?;
            read_array(reader, length as usize)
        }
    }
    impl NbtRead for Vec<i8> {
        /// Read a bytearray from a reader.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let length = u32::nbt_read(reader)?;
            let bytes = read_bytes(reader, length as usize)?;
            Ok(bytes.into_iter().map(|x| x as i8).collect())
        }
    }
    impl NbtRead for String {
        /// Read a String from a reader.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            let length: u16 = u16::nbt_read(reader)?;
            let strbytes = read_bytes(reader, length as usize)?;
            Ok(String::from_utf8(strbytes)?)
        }
    }
    impl NbtRead for TagID {
        /// Read a TagID from a reader. If `0` is encountered, this will return `Err(NbtError::End)`.
        fn nbt_read<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
            TagID::try_from(u8::nbt_read(reader)?)
        }
    }
    impl<S: AsRef<str>> NbtWrite for (S, Tag) {
        /// Write a Tuple as a named tag to a writer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            write_named_tag(writer, &self.1, self.0.as_ref())
        }
    }
    impl NbtWrite for TagID {
        /// Write a TagID to a writer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            (self.value() as u8).nbt_write(writer)
        }
    }
    impl NbtRead for NamedTag {
        ///Attempt to read a [NamedTag] from a reader. This is a wrapper around `read_named_tag(reader)
        fn nbt_read<R: Read>(reader: &mut R) -> Result<NamedTag, NbtError> {
            Ok(read_named_tag(reader)?.into())
        }
    }
    impl NbtWrite for &str {
        /// Write a string to a writer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            let length: u16 = self.len() as u16;
            length.nbt_write(writer)?;
            Ok(writer.write_all(self.as_bytes()).map(|_| self.len() + 2)?)
        }
    }
    impl NbtWrite for String {
        /// Write a string to a writer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            self.as_str().nbt_write(writer)
        }
    }
    impl<T: NbtWrite + NonByte> NbtWrite for Vec<T> {
        /// Write a [Vec] to a writer.
        /// This will also write the size of the [Vec] as a Big-Endian 32-bit integer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            (self.len() as u32).nbt_write(writer)?;
            write_array(writer, self.as_slice()).map(|size| size + 4)
        }
    }
    impl NbtWrite for Vec<i8> {
        /// Write a bytearray to a writer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            (self.len() as u32).nbt_write(writer)?;
            let u8slice: &[u8] = bytemuck::cast_slice(self.as_slice());
            Ok(write_bytes(writer, u8slice)? + 4)
        }
    }
    impl NbtWrite for Map {
        /// Write a [Map] to a writer.
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            let write_size = self
                .iter()
                .try_fold(
                    0usize,
                    |size, (key, tag)| {
                        write_named_tag(writer, tag, key).map(|written| written + size)
                    },
                )?;
            0u8.nbt_write(writer).map(|size| write_size + size)
        }
    }
    impl NbtWrite for NamedTag {
        ///Attempt to write a [NamedTag] to a writer. This is a wrapper around `write_named_tag(writer, self.tag(), self.name())`
        fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
            write_named_tag(writer, &self.tag, &self.name)
        }
    }
    impl NbtSize for NamedTag {
        /// Get the serialization size in bytes.
        fn nbt_size(&self) -> usize {
            self.name.nbt_size() + self.tag.nbt_size() + 1
        }
    }
}
pub(crate) mod table {
    pub use tag_info_table;
}
pub mod tag {
    use crate::{family::*, Map, tag_info_table};
    use num_traits::ToPrimitive;
    use num_traits::Zero;
    use std::fmt::Display;
    /// Marks that a type is directly represented as an NBT tag type.
    pub trait NbtType {
        /// The Minecraft NBT type ID.
        const ID: TagID;
        /// Converts to [`Tag`].
        fn nbt(self) -> Tag;
    }
    /// A trait for NBT encoder..
    /// This trait is intended for types that don't have a direct
    /// NBT representation, but can be encoded as an NBT tree.
    pub trait EncodeNbt {
        /// Encode as NBT.
        /// This typically results in a [`Tag::Compound`], but may result in other [`Tag`] variants.
        fn encode_nbt(self) -> Tag;
    }
    /// A trait for a non-consuming NBT decoder.
    /// This trait is intended for types that don't have a direct
    /// NBT representation, but can be decoded from NBT data.
    pub trait DecodeNbt: Sized {
        type Error;
        /// Tries to decode from NBT.
        fn decode_nbt(nbt: Tag) -> Result<Self, Self::Error>;
    }
    /**
		The NBT Tag enum.<br>
		To see what types are supported, take a look at the table in [tag_info_table] located in [`/src/table.rs`].
		*/
    pub enum Tag {
        Byte(i8),
        Short(i16),
        Int(i32),
        Long(i64),
        Float(f32),
        Double(f64),
        ByteArray(std::vec::Vec<i8>),
        String(std::string::String),
        List(crate::tag::ListTag),
        Compound(crate::Map),
        IntArray(std::vec::Vec<i32>),
        LongArray(std::vec::Vec<i64>),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Tag {
        #[inline]
        fn clone(&self) -> Tag {
            match self {
                Tag::Byte(__self_0) => Tag::Byte(::core::clone::Clone::clone(__self_0)),
                Tag::Short(__self_0) => Tag::Short(::core::clone::Clone::clone(__self_0)),
                Tag::Int(__self_0) => Tag::Int(::core::clone::Clone::clone(__self_0)),
                Tag::Long(__self_0) => Tag::Long(::core::clone::Clone::clone(__self_0)),
                Tag::Float(__self_0) => Tag::Float(::core::clone::Clone::clone(__self_0)),
                Tag::Double(__self_0) => {
                    Tag::Double(::core::clone::Clone::clone(__self_0))
                }
                Tag::ByteArray(__self_0) => {
                    Tag::ByteArray(::core::clone::Clone::clone(__self_0))
                }
                Tag::String(__self_0) => {
                    Tag::String(::core::clone::Clone::clone(__self_0))
                }
                Tag::List(__self_0) => Tag::List(::core::clone::Clone::clone(__self_0)),
                Tag::Compound(__self_0) => {
                    Tag::Compound(::core::clone::Clone::clone(__self_0))
                }
                Tag::IntArray(__self_0) => {
                    Tag::IntArray(::core::clone::Clone::clone(__self_0))
                }
                Tag::LongArray(__self_0) => {
                    Tag::LongArray(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Tag {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Tag::Byte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Byte",
                        &__self_0,
                    )
                }
                Tag::Short(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Short",
                        &__self_0,
                    )
                }
                Tag::Int(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Int",
                        &__self_0,
                    )
                }
                Tag::Long(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Long",
                        &__self_0,
                    )
                }
                Tag::Float(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Float",
                        &__self_0,
                    )
                }
                Tag::Double(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Double",
                        &__self_0,
                    )
                }
                Tag::ByteArray(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ByteArray",
                        &__self_0,
                    )
                }
                Tag::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "String",
                        &__self_0,
                    )
                }
                Tag::List(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "List",
                        &__self_0,
                    )
                }
                Tag::Compound(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Compound",
                        &__self_0,
                    )
                }
                Tag::IntArray(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IntArray",
                        &__self_0,
                    )
                }
                Tag::LongArray(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LongArray",
                        &__self_0,
                    )
                }
            }
        }
    }
    ///The NBT tag type ID.
    pub enum TagID {
        Byte = 0001,
        Short = 0002,
        Int = 0003,
        Long = 0004,
        Float = 0005,
        Double = 0006,
        ByteArray = 0007,
        String = 0008,
        List = 0009,
        Compound = 0010,
        IntArray = 0011,
        LongArray = 0012,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TagID {
        #[inline]
        fn clone(&self) -> TagID {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TagID {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TagID {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TagID {
        #[inline]
        fn eq(&self, other: &TagID) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TagID {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TagID {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for TagID {
        #[inline]
        fn partial_cmp(
            &self,
            other: &TagID,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for TagID {
        #[inline]
        fn cmp(&self, other: &TagID) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TagID {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                TagID::Byte => ::core::fmt::Formatter::write_str(f, "Byte"),
                TagID::Short => ::core::fmt::Formatter::write_str(f, "Short"),
                TagID::Int => ::core::fmt::Formatter::write_str(f, "Int"),
                TagID::Long => ::core::fmt::Formatter::write_str(f, "Long"),
                TagID::Float => ::core::fmt::Formatter::write_str(f, "Float"),
                TagID::Double => ::core::fmt::Formatter::write_str(f, "Double"),
                TagID::ByteArray => ::core::fmt::Formatter::write_str(f, "ByteArray"),
                TagID::String => ::core::fmt::Formatter::write_str(f, "String"),
                TagID::List => ::core::fmt::Formatter::write_str(f, "List"),
                TagID::Compound => ::core::fmt::Formatter::write_str(f, "Compound"),
                TagID::IntArray => ::core::fmt::Formatter::write_str(f, "IntArray"),
                TagID::LongArray => ::core::fmt::Formatter::write_str(f, "LongArray"),
            }
        }
    }
    ///Enum type for [Tag::List].
    pub enum ListTag {
        ///Represents a ListTag without any elements.
        Empty,
        Byte(Vec<i8>),
        Short(Vec<i16>),
        Int(Vec<i32>),
        Long(Vec<i64>),
        Float(Vec<f32>),
        Double(Vec<f64>),
        ByteArray(Vec<std::vec::Vec<i8>>),
        String(Vec<std::string::String>),
        List(Vec<crate::tag::ListTag>),
        Compound(Vec<crate::Map>),
        IntArray(Vec<std::vec::Vec<i32>>),
        LongArray(Vec<std::vec::Vec<i64>>),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ListTag {
        #[inline]
        fn clone(&self) -> ListTag {
            match self {
                ListTag::Empty => ListTag::Empty,
                ListTag::Byte(__self_0) => {
                    ListTag::Byte(::core::clone::Clone::clone(__self_0))
                }
                ListTag::Short(__self_0) => {
                    ListTag::Short(::core::clone::Clone::clone(__self_0))
                }
                ListTag::Int(__self_0) => {
                    ListTag::Int(::core::clone::Clone::clone(__self_0))
                }
                ListTag::Long(__self_0) => {
                    ListTag::Long(::core::clone::Clone::clone(__self_0))
                }
                ListTag::Float(__self_0) => {
                    ListTag::Float(::core::clone::Clone::clone(__self_0))
                }
                ListTag::Double(__self_0) => {
                    ListTag::Double(::core::clone::Clone::clone(__self_0))
                }
                ListTag::ByteArray(__self_0) => {
                    ListTag::ByteArray(::core::clone::Clone::clone(__self_0))
                }
                ListTag::String(__self_0) => {
                    ListTag::String(::core::clone::Clone::clone(__self_0))
                }
                ListTag::List(__self_0) => {
                    ListTag::List(::core::clone::Clone::clone(__self_0))
                }
                ListTag::Compound(__self_0) => {
                    ListTag::Compound(::core::clone::Clone::clone(__self_0))
                }
                ListTag::IntArray(__self_0) => {
                    ListTag::IntArray(::core::clone::Clone::clone(__self_0))
                }
                ListTag::LongArray(__self_0) => {
                    ListTag::LongArray(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ListTag {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ListTag::Empty => ::core::fmt::Formatter::write_str(f, "Empty"),
                ListTag::Byte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Byte",
                        &__self_0,
                    )
                }
                ListTag::Short(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Short",
                        &__self_0,
                    )
                }
                ListTag::Int(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Int",
                        &__self_0,
                    )
                }
                ListTag::Long(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Long",
                        &__self_0,
                    )
                }
                ListTag::Float(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Float",
                        &__self_0,
                    )
                }
                ListTag::Double(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Double",
                        &__self_0,
                    )
                }
                ListTag::ByteArray(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ByteArray",
                        &__self_0,
                    )
                }
                ListTag::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "String",
                        &__self_0,
                    )
                }
                ListTag::List(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "List",
                        &__self_0,
                    )
                }
                ListTag::Compound(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Compound",
                        &__self_0,
                    )
                }
                ListTag::IntArray(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IntArray",
                        &__self_0,
                    )
                }
                ListTag::LongArray(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LongArray",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl TagID {
        ///PascalCase title of this [TagID].
        pub const fn title(self) -> &'static str {
            match self {
                TagID::Byte => "Byte",
                TagID::Short => "Short",
                TagID::Int => "Int",
                TagID::Long => "Long",
                TagID::Float => "Float",
                TagID::Double => "Double",
                TagID::ByteArray => "ByteArray",
                TagID::String => "String",
                TagID::List => "List",
                TagID::Compound => "Compound",
                TagID::IntArray => "IntArray",
                TagID::LongArray => "LongArray",
            }
        }
        ///In the format of `TAG_TagTitle`.
        pub const fn name(self) -> &'static str {
            match self {
                TagID::Byte => "TAG_Byte",
                TagID::Short => "TAG_Short",
                TagID::Int => "TAG_Int",
                TagID::Long => "TAG_Long",
                TagID::Float => "TAG_Float",
                TagID::Double => "TAG_Double",
                TagID::ByteArray => "TAG_ByteArray",
                TagID::String => "TAG_String",
                TagID::List => "TAG_List",
                TagID::Compound => "TAG_Compound",
                TagID::IntArray => "TAG_IntArray",
                TagID::LongArray => "TAG_LongArray",
            }
        }
    }
    impl Tag {
        ///Returns the NBT type ID.
        pub fn id(&self) -> TagID {
            match self {
                Tag::Byte(_) => TagID::Byte,
                Tag::Short(_) => TagID::Short,
                Tag::Int(_) => TagID::Int,
                Tag::Long(_) => TagID::Long,
                Tag::Float(_) => TagID::Float,
                Tag::Double(_) => TagID::Double,
                Tag::ByteArray(_) => TagID::ByteArray,
                Tag::String(_) => TagID::String,
                Tag::List(_) => TagID::List,
                Tag::Compound(_) => TagID::Compound,
                Tag::IntArray(_) => TagID::IntArray,
                Tag::LongArray(_) => TagID::LongArray,
            }
        }
    }
    impl ListTag {
        ///Returns the list type ID. Returns [TagID::Byte] for [ListTag::Empty].
        pub fn id(&self) -> TagID {
            match self {
                ListTag::Empty => TagID::Byte,
                ListTag::Byte(_) => TagID::Byte,
                ListTag::Short(_) => TagID::Short,
                ListTag::Int(_) => TagID::Int,
                ListTag::Long(_) => TagID::Long,
                ListTag::Float(_) => TagID::Float,
                ListTag::Double(_) => TagID::Double,
                ListTag::ByteArray(_) => TagID::ByteArray,
                ListTag::String(_) => TagID::String,
                ListTag::List(_) => TagID::List,
                ListTag::Compound(_) => TagID::Compound,
                ListTag::IntArray(_) => TagID::IntArray,
                ListTag::LongArray(_) => TagID::LongArray,
            }
        }
        /**
			Returns the number of elements in the list.<br>
			Returns `0` for [ListTag::Empty].
			*/
        pub fn len(&self) -> usize {
            match self {
                ListTag::Byte(list) => list.len(),
                ListTag::Short(list) => list.len(),
                ListTag::Int(list) => list.len(),
                ListTag::Long(list) => list.len(),
                ListTag::Float(list) => list.len(),
                ListTag::Double(list) => list.len(),
                ListTag::ByteArray(list) => list.len(),
                ListTag::String(list) => list.len(),
                ListTag::List(list) => list.len(),
                ListTag::Compound(list) => list.len(),
                ListTag::IntArray(list) => list.len(),
                ListTag::LongArray(list) => list.len(),
                ListTag::Empty => 0,
            }
        }
    }
    impl TryFrom<u8> for TagID {
        type Error = crate::NbtError;
        /**
			Attempts to create a [TagID] from a [u8].<br>
			Errors:
			- [NbtError::End]
			- [NbtError::Unsupported] { id_encountered }
			*/
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0001 => Ok(TagID::Byte),
                0002 => Ok(TagID::Short),
                0003 => Ok(TagID::Int),
                0004 => Ok(TagID::Long),
                0005 => Ok(TagID::Float),
                0006 => Ok(TagID::Double),
                0007 => Ok(TagID::ByteArray),
                0008 => Ok(TagID::String),
                0009 => Ok(TagID::List),
                0010 => Ok(TagID::Compound),
                0011 => Ok(TagID::IntArray),
                0012 => Ok(TagID::LongArray),
                0 => Err(crate::NbtError::End),
                other => {
                    Err(crate::NbtError::Unsupported {
                        id_encountered: other,
                    })
                }
            }
        }
    }
    impl NbtType for i8 {
        ///The tag type ID.
        const ID: TagID = TagID::Byte;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &i8 {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for i8 {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::Byte(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::Primitive for i8 {}
    impl From<i8> for Tag {
        ///Create a [Tag::Byte] from its representational type.
        fn from(value: i8) -> Self {
            Tag::Byte(value)
        }
    }
    impl From<Vec<i8>> for ListTag {
        ///Create a [ListTag::Byte] from its representational vector type.
        fn from(value: Vec<i8>) -> Self {
            ListTag::Byte(value)
        }
    }
    impl From<&[i8]> for ListTag {
        ///Create a [ListTag::Byte] from its representational slice type.
        fn from(value: &[i8]) -> Self {
            ListTag::Byte(value.to_vec())
        }
    }
    impl TryFrom<Tag> for i8 {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<i8, ()> {
            if let Tag::Byte(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for i16 {
        ///The tag type ID.
        const ID: TagID = TagID::Short;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &i16 {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for i16 {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::Short(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonBytePrimitive for i16 {}
    impl From<i16> for Tag {
        ///Create a [Tag::Short] from its representational type.
        fn from(value: i16) -> Self {
            Tag::Short(value)
        }
    }
    impl From<Vec<i16>> for ListTag {
        ///Create a [ListTag::Short] from its representational vector type.
        fn from(value: Vec<i16>) -> Self {
            ListTag::Short(value)
        }
    }
    impl From<&[i16]> for ListTag {
        ///Create a [ListTag::Short] from its representational slice type.
        fn from(value: &[i16]) -> Self {
            ListTag::Short(value.to_vec())
        }
    }
    impl TryFrom<Tag> for i16 {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<i16, ()> {
            if let Tag::Short(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for i32 {
        ///The tag type ID.
        const ID: TagID = TagID::Int;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &i32 {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for i32 {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::Int(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonBytePrimitive for i32 {}
    impl From<i32> for Tag {
        ///Create a [Tag::Int] from its representational type.
        fn from(value: i32) -> Self {
            Tag::Int(value)
        }
    }
    impl From<Vec<i32>> for ListTag {
        ///Create a [ListTag::Int] from its representational vector type.
        fn from(value: Vec<i32>) -> Self {
            ListTag::Int(value)
        }
    }
    impl From<&[i32]> for ListTag {
        ///Create a [ListTag::Int] from its representational slice type.
        fn from(value: &[i32]) -> Self {
            ListTag::Int(value.to_vec())
        }
    }
    impl TryFrom<Tag> for i32 {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<i32, ()> {
            if let Tag::Int(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for i64 {
        ///The tag type ID.
        const ID: TagID = TagID::Long;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &i64 {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for i64 {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::Long(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonBytePrimitive for i64 {}
    impl From<i64> for Tag {
        ///Create a [Tag::Long] from its representational type.
        fn from(value: i64) -> Self {
            Tag::Long(value)
        }
    }
    impl From<Vec<i64>> for ListTag {
        ///Create a [ListTag::Long] from its representational vector type.
        fn from(value: Vec<i64>) -> Self {
            ListTag::Long(value)
        }
    }
    impl From<&[i64]> for ListTag {
        ///Create a [ListTag::Long] from its representational slice type.
        fn from(value: &[i64]) -> Self {
            ListTag::Long(value.to_vec())
        }
    }
    impl TryFrom<Tag> for i64 {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<i64, ()> {
            if let Tag::Long(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for f32 {
        ///The tag type ID.
        const ID: TagID = TagID::Float;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &f32 {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for f32 {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::Float(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonBytePrimitive for f32 {}
    impl From<f32> for Tag {
        ///Create a [Tag::Float] from its representational type.
        fn from(value: f32) -> Self {
            Tag::Float(value)
        }
    }
    impl From<Vec<f32>> for ListTag {
        ///Create a [ListTag::Float] from its representational vector type.
        fn from(value: Vec<f32>) -> Self {
            ListTag::Float(value)
        }
    }
    impl From<&[f32]> for ListTag {
        ///Create a [ListTag::Float] from its representational slice type.
        fn from(value: &[f32]) -> Self {
            ListTag::Float(value.to_vec())
        }
    }
    impl TryFrom<Tag> for f32 {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<f32, ()> {
            if let Tag::Float(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for f64 {
        ///The tag type ID.
        const ID: TagID = TagID::Double;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &f64 {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for f64 {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::Double(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonBytePrimitive for f64 {}
    impl From<f64> for Tag {
        ///Create a [Tag::Double] from its representational type.
        fn from(value: f64) -> Self {
            Tag::Double(value)
        }
    }
    impl From<Vec<f64>> for ListTag {
        ///Create a [ListTag::Double] from its representational vector type.
        fn from(value: Vec<f64>) -> Self {
            ListTag::Double(value)
        }
    }
    impl From<&[f64]> for ListTag {
        ///Create a [ListTag::Double] from its representational slice type.
        fn from(value: &[f64]) -> Self {
            ListTag::Double(value.to_vec())
        }
    }
    impl TryFrom<Tag> for f64 {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<f64, ()> {
            if let Tag::Double(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for std::vec::Vec<i8> {
        ///The tag type ID.
        const ID: TagID = TagID::ByteArray;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &std::vec::Vec<i8> {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for std::vec::Vec<i8> {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::ByteArray(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonByte for std::vec::Vec<i8> {}
    impl From<std::vec::Vec<i8>> for Tag {
        ///Create a [Tag::ByteArray] from its representational type.
        fn from(value: std::vec::Vec<i8>) -> Self {
            Tag::ByteArray(value)
        }
    }
    impl From<Vec<std::vec::Vec<i8>>> for ListTag {
        ///Create a [ListTag::ByteArray] from its representational vector type.
        fn from(value: Vec<std::vec::Vec<i8>>) -> Self {
            ListTag::ByteArray(value)
        }
    }
    impl From<&[std::vec::Vec<i8>]> for ListTag {
        ///Create a [ListTag::ByteArray] from its representational slice type.
        fn from(value: &[std::vec::Vec<i8>]) -> Self {
            ListTag::ByteArray(value.to_vec())
        }
    }
    impl TryFrom<Tag> for std::vec::Vec<i8> {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<std::vec::Vec<i8>, ()> {
            if let Tag::ByteArray(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for std::string::String {
        ///The tag type ID.
        const ID: TagID = TagID::String;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &std::string::String {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for std::string::String {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::String(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonByte for std::string::String {}
    impl From<std::string::String> for Tag {
        ///Create a [Tag::String] from its representational type.
        fn from(value: std::string::String) -> Self {
            Tag::String(value)
        }
    }
    impl From<Vec<std::string::String>> for ListTag {
        ///Create a [ListTag::String] from its representational vector type.
        fn from(value: Vec<std::string::String>) -> Self {
            ListTag::String(value)
        }
    }
    impl From<&[std::string::String]> for ListTag {
        ///Create a [ListTag::String] from its representational slice type.
        fn from(value: &[std::string::String]) -> Self {
            ListTag::String(value.to_vec())
        }
    }
    impl TryFrom<Tag> for std::string::String {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<std::string::String, ()> {
            if let Tag::String(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for crate::tag::ListTag {
        ///The tag type ID.
        const ID: TagID = TagID::List;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &crate::tag::ListTag {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for crate::tag::ListTag {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::List(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonByte for crate::tag::ListTag {}
    impl From<crate::tag::ListTag> for Tag {
        ///Create a [Tag::List] from its representational type.
        fn from(value: crate::tag::ListTag) -> Self {
            Tag::List(value)
        }
    }
    impl From<Vec<crate::tag::ListTag>> for ListTag {
        ///Create a [ListTag::List] from its representational vector type.
        fn from(value: Vec<crate::tag::ListTag>) -> Self {
            ListTag::List(value)
        }
    }
    impl From<&[crate::tag::ListTag]> for ListTag {
        ///Create a [ListTag::List] from its representational slice type.
        fn from(value: &[crate::tag::ListTag]) -> Self {
            ListTag::List(value.to_vec())
        }
    }
    impl TryFrom<Tag> for crate::tag::ListTag {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<crate::tag::ListTag, ()> {
            if let Tag::List(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for crate::Map {
        ///The tag type ID.
        const ID: TagID = TagID::Compound;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &crate::Map {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for crate::Map {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::Compound(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonByte for crate::Map {}
    impl From<crate::Map> for Tag {
        ///Create a [Tag::Compound] from its representational type.
        fn from(value: crate::Map) -> Self {
            Tag::Compound(value)
        }
    }
    impl From<Vec<crate::Map>> for ListTag {
        ///Create a [ListTag::Compound] from its representational vector type.
        fn from(value: Vec<crate::Map>) -> Self {
            ListTag::Compound(value)
        }
    }
    impl From<&[crate::Map]> for ListTag {
        ///Create a [ListTag::Compound] from its representational slice type.
        fn from(value: &[crate::Map]) -> Self {
            ListTag::Compound(value.to_vec())
        }
    }
    impl TryFrom<Tag> for crate::Map {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<crate::Map, ()> {
            if let Tag::Compound(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for std::vec::Vec<i32> {
        ///The tag type ID.
        const ID: TagID = TagID::IntArray;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &std::vec::Vec<i32> {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for std::vec::Vec<i32> {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::IntArray(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonByte for std::vec::Vec<i32> {}
    impl From<std::vec::Vec<i32>> for Tag {
        ///Create a [Tag::IntArray] from its representational type.
        fn from(value: std::vec::Vec<i32>) -> Self {
            Tag::IntArray(value)
        }
    }
    impl From<Vec<std::vec::Vec<i32>>> for ListTag {
        ///Create a [ListTag::IntArray] from its representational vector type.
        fn from(value: Vec<std::vec::Vec<i32>>) -> Self {
            ListTag::IntArray(value)
        }
    }
    impl From<&[std::vec::Vec<i32>]> for ListTag {
        ///Create a [ListTag::IntArray] from its representational slice type.
        fn from(value: &[std::vec::Vec<i32>]) -> Self {
            ListTag::IntArray(value.to_vec())
        }
    }
    impl TryFrom<Tag> for std::vec::Vec<i32> {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<std::vec::Vec<i32>, ()> {
            if let Tag::IntArray(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    impl NbtType for std::vec::Vec<i64> {
        ///The tag type ID.
        const ID: TagID = TagID::LongArray;
        ///Converts to an NBT [Tag].
        fn nbt(self) -> Tag {
            self.into()
        }
    }
    impl EncodeNbt for &std::vec::Vec<i64> {
        ///Encodes self as an NBT tag.
        fn encode_nbt(self) -> Tag {
            self.clone().into()
        }
    }
    impl DecodeNbt for std::vec::Vec<i64> {
        type Error = ();
        ///Attempts to decode the tag.
        fn decode_nbt(tag: Tag) -> Result<Self, ()> {
            if let Tag::LongArray(tag) = tag {
                return Ok(tag);
            }
            Err(())
        }
    }
    impl crate::family::NonByte for std::vec::Vec<i64> {}
    impl From<std::vec::Vec<i64>> for Tag {
        ///Create a [Tag::LongArray] from its representational type.
        fn from(value: std::vec::Vec<i64>) -> Self {
            Tag::LongArray(value)
        }
    }
    impl From<Vec<std::vec::Vec<i64>>> for ListTag {
        ///Create a [ListTag::LongArray] from its representational vector type.
        fn from(value: Vec<std::vec::Vec<i64>>) -> Self {
            ListTag::LongArray(value)
        }
    }
    impl From<&[std::vec::Vec<i64>]> for ListTag {
        ///Create a [ListTag::LongArray] from its representational slice type.
        fn from(value: &[std::vec::Vec<i64>]) -> Self {
            ListTag::LongArray(value.to_vec())
        }
    }
    impl TryFrom<Tag> for std::vec::Vec<i64> {
        type Error = ();
        ///Tries to recreate a representational type from a [Tag].
        fn try_from(value: Tag) -> Result<std::vec::Vec<i64>, ()> {
            if let Tag::LongArray(inner) = value {
                return Ok(inner);
            }
            Err(())
        }
    }
    /// Represents a Named NBT Tag, often used as a Tag Root for an NBT file.
    /// This is also sometimes called a root tag.
    pub struct NamedTag {
        pub(crate) name: String,
        pub(crate) tag: Tag,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NamedTag {
        #[inline]
        fn clone(&self) -> NamedTag {
            NamedTag {
                name: ::core::clone::Clone::clone(&self.name),
                tag: ::core::clone::Clone::clone(&self.tag),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NamedTag {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "NamedTag",
                "name",
                &&self.name,
                "tag",
                &&self.tag,
            )
        }
    }
    impl NamedTag {
        /// Creates a new NamedTag that has a blank name (`String::default()`)
        pub fn new<T>(tag: T) -> Self
        where
            T: Into<Tag>,
        {
            Self {
                name: String::default(),
                tag: tag.into(),
            }
        }
        /// Creates a NamedTag with the supplied name.
        pub fn with_name<S, T>(name: S, tag: T) -> Self
        where
            S: Into<String>,
            T: Into<Tag>,
        {
            Self {
                name: name.into(),
                tag: tag.into(),
            }
        }
        /// Return the name.
        pub fn name(&self) -> &str {
            &self.name
        }
        /// Immutably borrow the Tag.
        pub fn tag(&self) -> &Tag {
            &self.tag
        }
        /// Mutably borrow the NamedTag's tag value.
        pub fn tag_mut(&mut self) -> &mut Tag {
            &mut self.tag
        }
        /// Irreversibly take the [Tag] from the [NamedTag], ignoring the name.
        pub fn take_tag(self) -> Tag {
            self.tag
        }
        /// Set the NamedTag's tag value.
        pub fn set_tag<T: Into<Tag>>(&mut self, tag: T) {
            self.tag = tag.into();
        }
        /// Set the NamedTag's name.
        pub fn set_name<T: Into<String>>(&mut self, name: T) {
            self.name = name.into();
        }
    }
    /// Creates a NamedTag from (Into<String>, Into<Tag>)
    impl<S, T> From<(S, T)> for NamedTag
    where
        S: Into<String>,
        T: Into<Tag>,
    {
        /// Convert to a NamedTag from a Tuple.
        fn from(value: (S, T)) -> Self {
            Self {
                name: value.0.into(),
                tag: value.1.into(),
            }
        }
    }
    /// Creates a (From<String>, Tag) from a NamedTag.
    impl<S> From<NamedTag> for (S, Tag)
    where
        S: From<String>,
    {
        /// Convert to a Tuple from a NamedTag.
        fn from(value: NamedTag) -> Self {
            (S::from(value.name), value.tag)
        }
    }
    impl TagID {
        /// Returns this TagID as an isize.
        pub fn value(self) -> isize {
            self as isize
        }
    }
    impl Tag {
        /// PascalCase title of this Tag.
        pub fn title(&self) -> &'static str {
            self.id().title()
        }
        /// In the format of TAG_TagTitle.
        pub fn name(&self) -> &'static str {
            self.id().name()
        }
        /// Create a [Tag::Byte] from the provided value.
        /// If the provided value cannot be converted to an [i8], then [Tag::Byte(0)] will be returned.
        pub fn byte<T: ToPrimitive>(value: T) -> Tag {
            if let Some(value) = value.to_i8() { Tag::Byte(value) } else { Tag::Byte(0) }
        }
        /// Create a [Tag::Short] from the provided value.
        /// If the provided value cannot be converted to an [i16], then `Tag::Short(0)` will be returned.
        pub fn short<T: ToPrimitive>(value: T) -> Tag {
            if let Some(value) = value.to_i16() {
                Tag::Short(value)
            } else {
                Tag::Short(0)
            }
        }
        /// Create a [Tag::Int] from the provided value.
        /// If the provided value cannot be converted to an [i32], then `Tag::Int(0)` will be returned.
        pub fn int<T: ToPrimitive>(value: T) -> Tag {
            if let Some(value) = value.to_i32() { Tag::Int(value) } else { Tag::Int(0) }
        }
        /// Create a [Tag::Long] from the provided value.
        /// If the provided value cannot be converted to an [i64], then `Tag::Long(0)` will be returned.
        pub fn long<T: ToPrimitive>(value: T) -> Tag {
            if let Some(value) = value.to_i64() {
                Tag::Long(value)
            } else {
                Tag::Long(0)
            }
        }
        /// Create a [Tag::Float] from the provided value.
        /// If the provided value cannot be converted to an [f32], then `Tag::Float(f32::NAN)` will be returned.
        pub fn float<T: ToPrimitive>(value: T) -> Tag {
            if let Some(value) = value.to_f32() {
                Tag::Float(value)
            } else {
                Tag::Float(f32::NAN)
            }
        }
        /// Create a [Tag::Double] from the provided value.
        /// If the provided value cannot be converted to an [f64]m then `Tag::Double(f64::NAN)` will be returned.
        pub fn double<T: ToPrimitive>(value: T) -> Tag {
            if let Some(value) = value.to_f64() {
                Tag::Double(value)
            } else {
                Tag::Double(f64::NAN)
            }
        }
        /// Create a [Tag::ByteArray] from the provided iterable.
        pub fn bytearray<T: Into<i8>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
            Tag::ByteArray(it.into_iter().map(T::into).collect())
        }
        /// Create a [Tag::ByteArray] from the provided iterable.
        pub fn bytes<T: Into<u8>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
            Tag::ByteArray(it.into_iter().map(|value| value.into() as i8).collect())
        }
        /// Create a [Tag::IntArray] from the provided iterable.
        pub fn intarray<T: Into<i32>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
            Tag::IntArray(it.into_iter().map(T::into).collect())
        }
        /// Create a [Tag::LongArray] from the provided iterable.
        pub fn longarray<T: Into<i64>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
            Tag::LongArray(it.into_iter().map(T::into).collect())
        }
        /// Create a [Tag::String].
        pub fn string<S: Into<String>>(value: S) -> Tag {
            Tag::String(value.into())
        }
        /// Create a [Tag::List].
        pub fn list<T: NbtType, IT: IntoIterator<Item = T>>(it: IT) -> Tag
        where
            Vec<T>: Into<ListTag>,
        {
            Tag::List(ListTag::from(it.into_iter().collect::<Vec<T>>().into()))
        }
        /// Create a [Tag::Compound].
        pub fn compound<T, IT, S>(items: IT) -> Tag
        where
            T: Into<Tag>,
            IT: IntoIterator<Item = (S, T)>,
            S: Into<String>,
        {
            let mut result = Map::new();
            items
                .into_iter()
                .for_each(|(name, tag)| {
                    result.insert(name.into(), tag.into());
                });
            Tag::Compound(result)
        }
    }
    /// Creates a [Tag::Byte] from a boolean value.
    impl From<bool> for Tag {
        /// Create a [Tag::Byte] from a boolean value.
        fn from(on: bool) -> Self {
            Tag::Byte(if on { 1 } else { 0 })
        }
    }
    /// Creates a [Tag::String] from &str
    impl From<&str> for Tag {
        /// Creates a [Tag::String].
        fn from(value: &str) -> Self {
            Tag::String(String::from(value))
        }
    }
    /// Attempts to create a [bool] from a [Tag].
    /// The [Tag] must be a numeric type, such as [Tag::Byte], or [Tag::Float]. `0` Represents `false` and non-zero represents `true`.
    impl TryFrom<Tag> for bool {
        type Error = ();
        /// Tries to create a [bool] from a [Tag] value.
        /// The [Tag] type must be a numeric type, such as [Tag::Byte], [Tag::Int], [Tag::Float], etc.
        /// Returns `false` for zero, and `true` for non-zero.
        fn try_from(value: Tag) -> Result<Self, Self::Error> {
            Ok(
                match value {
                    Tag::Byte(inner) => !inner.is_zero(),
                    Tag::Short(inner) => !inner.is_zero(),
                    Tag::Int(inner) => !inner.is_zero(),
                    Tag::Long(inner) => !inner.is_zero(),
                    Tag::Float(inner) => !inner.is_zero(),
                    Tag::Double(inner) => !inner.is_zero(),
                    _ => return Err(()),
                },
            )
        }
    }
    impl Display for TagID {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                ::core::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_debug(&self)],
                    &[
                        ::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        },
                    ],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ),
            )
        }
    }
    impl Display for Tag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                ::core::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_debug(&self)],
                    &[
                        ::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        },
                    ],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ),
            )
        }
    }
    impl Display for ListTag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                ::core::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_debug(&self)],
                    &[
                        ::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        },
                    ],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ),
            )
        }
    }
    impl Display for NamedTag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                ::core::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_debug(&self)],
                    &[
                        ::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        },
                    ],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ),
            )
        }
    }
}
pub mod macros {
    pub use list;
    pub use compound;
}
pub mod snbt {
    /*!
This module is for parsing and generating SNBT. This module will only cover
Minecraft SNBT, and will not cover the Tag type extensions.

| Tag Type        | Syntax                                                                      |
|-----------------|-----------------------------------------------------------------------------|
|[Tag::Byte]      | `<number>b` or `<number>B`
|[Tag::Short]     | `<number>s` or `<number>S`
|[Tag::Int]       | `<integer_number>`
|[Tag::Long]      | `<number>l` or `<number>L`
|[Tag::Float]     | `<number>f` or `<number>F`
|[Tag::Double]    | `<decimal_number>`, `<number>d` or `<number>D`
|[Tag::ByteArray] | `[B; 0b, 1b, 2b]`
|[Tag::String]    | `"<text>"` or `'<text>'` or <identifier>
|[Tag::List]      | `[<tag>...]`
|[Tag::Compound]  | `{ <string:name> : <tag> }`
|[Tag::IntArray]  | `[I; 0, 1, 2 3]`
|[Tag::LongArray] | `[L; 0, 1, 2l, 3L]`

Note: Identifiers can include the following characters: [a-zA-Z0-9+-._].
For [Tag::List], the tag type for the list is determined by the type of the first tag.
*/
    use crate::*;
    use crate::tag::*;
    use chumsky::prelude::*;
    use chumsky::primitive::{Container, OneOf, NoneOf};
    use chumsky::Error;
    use std::collections::HashSet;
    use std::fmt::{Write, Display};
    use std::str::FromStr;
    pub enum Token {
        Comma,
        Colon,
        ArrayStart(ArrayType),
        OpenBracket,
        CloseBracket,
        OpenBrace,
        CloseBrace,
        Boolean(bool),
        Integer(String, IntegerType),
        Decimal(String, DecimalType),
        Identifier(String),
        StringLiteral(String),
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Token {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Token {
        #[inline]
        fn eq(&self, other: &Token) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Token::ArrayStart(__self_0), Token::ArrayStart(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Token::Boolean(__self_0), Token::Boolean(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        Token::Integer(__self_0, __self_1),
                        Token::Integer(__arg1_0, __arg1_1),
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    (
                        Token::Decimal(__self_0, __self_1),
                        Token::Decimal(__arg1_0, __arg1_1),
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    (Token::Identifier(__self_0), Token::Identifier(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Token::StringLiteral(__self_0), Token::StringLiteral(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Token {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Token {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<ArrayType>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<IntegerType>;
            let _: ::core::cmp::AssertParamIsEq<DecimalType>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Token {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Token,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match (self, other) {
                        (Token::ArrayStart(__self_0), Token::ArrayStart(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        (Token::Boolean(__self_0), Token::Boolean(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        (
                            Token::Integer(__self_0, __self_1),
                            Token::Integer(__arg1_0, __arg1_1),
                        ) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                __self_0,
                                __arg1_0,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                                }
                                cmp => cmp,
                            }
                        }
                        (
                            Token::Decimal(__self_0, __self_1),
                            Token::Decimal(__arg1_0, __arg1_1),
                        ) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                __self_0,
                                __arg1_0,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                                }
                                cmp => cmp,
                            }
                        }
                        (Token::Identifier(__self_0), Token::Identifier(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        (
                            Token::StringLiteral(__self_0),
                            Token::StringLiteral(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Token {
        #[inline]
        fn cmp(&self, other: &Token) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                ::core::cmp::Ordering::Equal => {
                    match (self, other) {
                        (Token::ArrayStart(__self_0), Token::ArrayStart(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (Token::Boolean(__self_0), Token::Boolean(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (
                            Token::Integer(__self_0, __self_1),
                            Token::Integer(__arg1_0, __arg1_1),
                        ) => {
                            match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                                }
                                cmp => cmp,
                            }
                        }
                        (
                            Token::Decimal(__self_0, __self_1),
                            Token::Decimal(__arg1_0, __arg1_1),
                        ) => {
                            match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                                }
                                cmp => cmp,
                            }
                        }
                        (Token::Identifier(__self_0), Token::Identifier(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (
                            Token::StringLiteral(__self_0),
                            Token::StringLiteral(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::Ordering::Equal,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Token {
        #[inline]
        fn clone(&self) -> Token {
            match self {
                Token::Comma => Token::Comma,
                Token::Colon => Token::Colon,
                Token::ArrayStart(__self_0) => {
                    Token::ArrayStart(::core::clone::Clone::clone(__self_0))
                }
                Token::OpenBracket => Token::OpenBracket,
                Token::CloseBracket => Token::CloseBracket,
                Token::OpenBrace => Token::OpenBrace,
                Token::CloseBrace => Token::CloseBrace,
                Token::Boolean(__self_0) => {
                    Token::Boolean(::core::clone::Clone::clone(__self_0))
                }
                Token::Integer(__self_0, __self_1) => {
                    Token::Integer(
                        ::core::clone::Clone::clone(__self_0),
                        ::core::clone::Clone::clone(__self_1),
                    )
                }
                Token::Decimal(__self_0, __self_1) => {
                    Token::Decimal(
                        ::core::clone::Clone::clone(__self_0),
                        ::core::clone::Clone::clone(__self_1),
                    )
                }
                Token::Identifier(__self_0) => {
                    Token::Identifier(::core::clone::Clone::clone(__self_0))
                }
                Token::StringLiteral(__self_0) => {
                    Token::StringLiteral(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Token {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Token::ArrayStart(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Token::Boolean(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Token::Integer(__self_0, __self_1) => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state)
                }
                Token::Decimal(__self_0, __self_1) => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state)
                }
                Token::Identifier(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Token::StringLiteral(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Token {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Token::Comma => ::core::fmt::Formatter::write_str(f, "Comma"),
                Token::Colon => ::core::fmt::Formatter::write_str(f, "Colon"),
                Token::ArrayStart(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ArrayStart",
                        &__self_0,
                    )
                }
                Token::OpenBracket => ::core::fmt::Formatter::write_str(f, "OpenBracket"),
                Token::CloseBracket => {
                    ::core::fmt::Formatter::write_str(f, "CloseBracket")
                }
                Token::OpenBrace => ::core::fmt::Formatter::write_str(f, "OpenBrace"),
                Token::CloseBrace => ::core::fmt::Formatter::write_str(f, "CloseBrace"),
                Token::Boolean(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Boolean",
                        &__self_0,
                    )
                }
                Token::Integer(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Integer",
                        &__self_0,
                        &__self_1,
                    )
                }
                Token::Decimal(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Decimal",
                        &__self_0,
                        &__self_1,
                    )
                }
                Token::Identifier(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Identifier",
                        &__self_0,
                    )
                }
                Token::StringLiteral(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "StringLiteral",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub enum ArrayType {
        Byte = 'B' as isize,
        Int = 'I' as isize,
        Long = 'L' as isize,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArrayType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArrayType {
        #[inline]
        fn eq(&self, other: &ArrayType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ArrayType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ArrayType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArrayType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArrayType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArrayType {
        #[inline]
        fn cmp(&self, other: &ArrayType) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArrayType {
        #[inline]
        fn clone(&self) -> ArrayType {
            match self {
                ArrayType::Byte => ArrayType::Byte,
                ArrayType::Int => ArrayType::Int,
                ArrayType::Long => ArrayType::Long,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ArrayType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArrayType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ArrayType::Byte => ::core::fmt::Formatter::write_str(f, "Byte"),
                ArrayType::Int => ::core::fmt::Formatter::write_str(f, "Int"),
                ArrayType::Long => ::core::fmt::Formatter::write_str(f, "Long"),
            }
        }
    }
    pub enum IntegerType {
        Byte,
        Int,
        Short,
        Long,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for IntegerType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for IntegerType {
        #[inline]
        fn eq(&self, other: &IntegerType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for IntegerType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for IntegerType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for IntegerType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &IntegerType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for IntegerType {
        #[inline]
        fn cmp(&self, other: &IntegerType) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IntegerType {
        #[inline]
        fn clone(&self) -> IntegerType {
            match self {
                IntegerType::Byte => IntegerType::Byte,
                IntegerType::Int => IntegerType::Int,
                IntegerType::Short => IntegerType::Short,
                IntegerType::Long => IntegerType::Long,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for IntegerType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IntegerType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                IntegerType::Byte => ::core::fmt::Formatter::write_str(f, "Byte"),
                IntegerType::Int => ::core::fmt::Formatter::write_str(f, "Int"),
                IntegerType::Short => ::core::fmt::Formatter::write_str(f, "Short"),
                IntegerType::Long => ::core::fmt::Formatter::write_str(f, "Long"),
            }
        }
    }
    pub enum DecimalType {
        Float,
        Double,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DecimalType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DecimalType {
        #[inline]
        fn eq(&self, other: &DecimalType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for DecimalType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for DecimalType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DecimalType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DecimalType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DecimalType {
        #[inline]
        fn cmp(&self, other: &DecimalType) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DecimalType {
        #[inline]
        fn clone(&self) -> DecimalType {
            match self {
                DecimalType::Float => DecimalType::Float,
                DecimalType::Double => DecimalType::Double,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DecimalType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DecimalType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DecimalType::Float => ::core::fmt::Formatter::write_str(f, "Float"),
                DecimalType::Double => ::core::fmt::Formatter::write_str(f, "Double"),
            }
        }
    }
    impl Token {
        pub fn comma() -> impl Parser<char, Token, Error = Simple<char>> {
            just(',').to(Token::Comma).labelled("Comma")
        }
        pub fn colon() -> impl Parser<char, Token, Error = Simple<char>> {
            just(':').to(Token::Colon).labelled("Colon")
        }
        pub fn array_start() -> impl Parser<char, Token, Error = Simple<char>> {
            just('[')
                .ignore_then(
                    choice((
                        keyword("b", true).to(ArrayType::Byte),
                        keyword("i", true).to(ArrayType::Int),
                        keyword("l", true).to(ArrayType::Long),
                    )),
                )
                .then_ignore(just(';'))
                .map(Token::ArrayStart)
                .labelled("Array Start")
        }
        pub fn open_bracket() -> impl Parser<char, Token, Error = Simple<char>> {
            just('[').to(Token::OpenBracket).labelled("Open Bracket")
        }
        pub fn close_bracket() -> impl Parser<char, Token, Error = Simple<char>> {
            just(']').to(Token::CloseBracket).labelled("Close Bracket")
        }
        pub fn open_brace() -> impl Parser<char, Token, Error = Simple<char>> {
            just('{').to(Token::OpenBrace).labelled("Open Brace")
        }
        pub fn close_brace() -> impl Parser<char, Token, Error = Simple<char>> {
            just('}').to(Token::CloseBrace).labelled("Close Brace")
        }
        pub fn boolean() -> impl Parser<char, Token, Error = Simple<char>> {
            choice((
                    text::keyword("true").to(Token::Boolean(true)),
                    text::keyword("false").to(Token::Boolean(false)),
                ))
                .labelled("Boolean")
        }
        pub fn integer() -> impl Parser<char, Token, Error = Simple<char>> {
            just::<char, _, Simple<char>>('-')
                .or_not()
                .chain::<char, _, _>(text::int(10))
                .collect::<String>()
                .then(
                    choice((
                            keyword("b", true).to(IntegerType::Byte),
                            keyword("s", true).to(IntegerType::Short),
                            keyword("l", true).to(IntegerType::Long),
                        ))
                        .or_not()
                        .map(|opt| opt.unwrap_or(IntegerType::Int)),
                )
                .then_ignore(
                    choice((
                            filter(|c: &char| {
                                !c.is_alphanumeric() && !['_', '+', '-', '.'].contains(c)
                            }),
                            end().to('\0'),
                        ))
                        .rewind(),
                )
                .map(|(int_text, int_type)| Token::Integer(int_text, int_type))
                .labelled("Integer")
        }
        pub fn decimal() -> impl Parser<char, Token, Error = Simple<char>> {
            just::<char, _, Simple<char>>('-')
                .or_not()
                .chain::<
                    char,
                    _,
                    _,
                >(
                    choice((
                        text::int(10)
                            .chain::<char, _, _>(just('.'))
                            .chain::<char, _, _>(text::digits(10))
                            .collect::<String>(),
                        text::int(10)
                            .then_ignore(
                                choice((keyword("d", true), keyword("f", true))).rewind(),
                            ),
                    )),
                )
                .collect::<String>()
                .then(
                    choice((
                            keyword("d", true).to(DecimalType::Double),
                            keyword("f", true).to(DecimalType::Float),
                        ))
                        .or_not()
                        .map(|opt| opt.unwrap_or(DecimalType::Double)),
                )
                .then_ignore(
                    choice((
                            filter(|c: &char| {
                                !c.is_alphanumeric() && !['_', '+', '-', '.'].contains(c)
                            }),
                            end().to('\0'),
                        ))
                        .rewind(),
                )
                .map(|(dec_str, dec_type)| Token::Decimal(dec_str, dec_type))
                .labelled("Decimal")
        }
        pub fn identifier() -> impl Parser<char, Token, Error = Simple<char>> {
            choice((filter(char::is_ascii_alphanumeric), one_of("+-_.")))
                .repeated()
                .at_least(1)
                .collect::<String>()
                .map(Token::Identifier)
                .labelled("Identifier")
        }
        pub fn string_literal() -> impl Parser<char, Token, Error = Simple<char>> {
            let escape = just::<_, _, Simple<char>>('\\')
                .ignore_then(
                    just('\\')
                        .or(just('/'))
                        .or(just('"'))
                        .or(just('\''))
                        .or(just('b').to('\x08'))
                        .or(just('f').to('\x0C'))
                        .or(just('n').to('\n'))
                        .or(just('r').to('\r'))
                        .or(just('t').to('\t')),
                );
            Token::identifier()
                .or(
                    choice::<
                        _,
                        Simple<char>,
                    >((
                            just('"')
                                .ignore_then(none_of("\\\"").or(escape.clone()).repeated())
                                .then_ignore(just('"'))
                                .collect::<String>(),
                            just('\'')
                                .ignore_then(none_of("\\'").or(escape.clone()).repeated())
                                .then_ignore(just('\''))
                                .collect::<String>(),
                        ))
                        .map(Token::StringLiteral),
                )
                .labelled("String Literal")
        }
        pub fn parse<S: AsRef<str>>(source: S) -> Result<Vec<Token>, Vec<Simple<char>>> {
            choice((
                    Self::comma(),
                    Self::colon(),
                    Self::array_start(),
                    Self::open_bracket(),
                    Self::close_bracket(),
                    Self::open_brace(),
                    Self::close_brace(),
                    Self::boolean(),
                    Self::integer(),
                    Self::decimal(),
                    Self::identifier(),
                    Self::string_literal(),
                ))
                .padded()
                .repeated()
                .at_least(1)
                .then_ignore(end())
                .collect::<Vec<Token>>()
                .parse(source.as_ref())
        }
    }
    /// Returns a parser that takes [Token] as input and returns a [Tag].
    fn parser() -> impl Parser<Token, Tag, Error = Simple<Token>> {
        let byte = filter::<
            Token,
            _,
            Simple<Token>,
        >(|token| match token {
                Token::Integer(_, IntegerType::Byte) => true,
                _ => false,
            })
            .try_map(|token, span| {
                match token {
                    Token::Integer(digits, IntegerType::Byte) => {
                        digits
                            .parse::<i8>()
                            .map_err(|_| Simple::custom(span, "Failed to parse."))
                    }
                    _ => Err(Simple::custom(span, "Invalid token.")),
                }
            });
        let short = filter::<
            Token,
            _,
            Simple<Token>,
        >(|token| match token {
                Token::Integer(_, IntegerType::Short) => true,
                _ => false,
            })
            .try_map(|token, span| {
                match token {
                    Token::Integer(digits, IntegerType::Short) => {
                        digits
                            .parse::<i16>()
                            .map_err(|_| Simple::custom(span, "Failed to parse."))
                    }
                    _ => Err(Simple::custom(span, "Invalid token.")),
                }
            });
        let int = filter::<
            Token,
            _,
            Simple<Token>,
        >(|token| match token {
                Token::Integer(_, IntegerType::Int) => true,
                _ => false,
            })
            .try_map(|token, span| {
                match token {
                    Token::Integer(digits, IntegerType::Int) => {
                        digits
                            .parse::<i32>()
                            .map_err(|_| Simple::custom(span, "Failed to parse."))
                    }
                    _ => Err(Simple::custom(span, "Invalid token.")),
                }
            });
        let long = filter::<
            Token,
            _,
            Simple<Token>,
        >(|token| match token {
                Token::Integer(_, IntegerType::Long) => true,
                _ => false,
            })
            .try_map(|token, span| {
                match token {
                    Token::Integer(digits, IntegerType::Long) => {
                        digits
                            .parse::<i64>()
                            .map_err(|_| Simple::custom(span, "Failed to parse."))
                    }
                    _ => Err(Simple::custom(span, "Invalid token.")),
                }
            });
        let float = filter::<
            Token,
            _,
            Simple<Token>,
        >(|token| match token {
                Token::Decimal(_, DecimalType::Float) => true,
                _ => false,
            })
            .try_map(|token, span| {
                match token {
                    Token::Decimal(digits, DecimalType::Float) => {
                        digits
                            .parse::<f32>()
                            .map_err(|_| Simple::custom(span, "Failed to parse."))
                    }
                    _ => Err(Simple::custom(span, "Invalid token.")),
                }
            });
        let double = filter::<
            Token,
            _,
            Simple<Token>,
        >(|token| match token {
                Token::Decimal(_, DecimalType::Double) => true,
                _ => false,
            })
            .try_map(|token, span| {
                match token {
                    Token::Decimal(digits, DecimalType::Double) => {
                        digits
                            .parse::<f64>()
                            .map_err(|_| Simple::custom(span, "Failed to parse."))
                    }
                    _ => Err(Simple::custom(span, "Invalid token.")),
                }
            });
        let byte = byte
            .or(
                choice((
                    filter(|token| match token {
                            Token::Boolean(true) => true,
                            _ => false,
                        })
                        .to(1i8),
                    filter(|token| match token {
                            Token::Boolean(false) => true,
                            _ => false,
                        })
                        .to(0i8),
                )),
            );
        let bytearray = (byte.clone())
            .separated_by(just(Token::Comma))
            .delimited_by(
                just(Token::ArrayStart(ArrayType::Byte)),
                just(Token::CloseBracket),
            );
        let intarray = (int.clone())
            .separated_by(just(Token::Comma))
            .delimited_by(
                just(Token::ArrayStart(ArrayType::Int)),
                just(Token::CloseBracket),
            );
        let longarray = (long.clone())
            .separated_by(just(Token::Comma))
            .delimited_by(
                just(Token::ArrayStart(ArrayType::Long)),
                just(Token::CloseBracket),
            );
        let byte = byte
            .or(
                filter::<
                    Token,
                    _,
                    Simple<Token>,
                >(|token| match token {
                        Token::Boolean(_) => true,
                        _ => false,
                    })
                    .map(|token| match token {
                        Token::Boolean(true) => 1i8,
                        _ => 0i8,
                    }),
            );
        let string = filter::<
            Token,
            _,
            Simple<Token>,
        >(|token| match token {
                Token::StringLiteral(_) | Token::Identifier(_) => true,
                _ => false,
            })
            .map(|token| match token {
                Token::StringLiteral(data) => data,
                Token::Identifier(data) => data,
                _ => {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(&["Impossible state."], &[]),
                    )
                }
            });
        let mut list = Recursive::declare();
        let mut compound = Recursive::declare();
        let tag_match = choice((
            compound.clone().map(Tag::Compound),
            list.clone().map(Tag::List),
            byte.clone().map(Tag::Byte),
            short.clone().map(Tag::Short),
            int.clone().map(Tag::Int),
            long.clone().map(Tag::Long),
            float.clone().map(Tag::Float),
            double.clone().map(Tag::Double),
            bytearray.clone().map(Tag::ByteArray),
            intarray.clone().map(Tag::IntArray),
            longarray.clone().map(Tag::LongArray),
            string.clone().map(Tag::String),
        ));
        list.define(
            choice::<
                _,
                Simple<Token>,
            >((
                (byte.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (short.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (int.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (long.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (float.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (double.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (bytearray.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (string.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (list.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (compound.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (intarray.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
                (longarray.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket))
                    .map(ListTag::from),
            )),
        );
        compound
            .define(
                string
                    .clone()
                    .then_ignore(just(Token::Colon))
                    .then(tag_match.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
                    .map(crate::Map::from_iter),
            );
        choice((
            compound.clone().map(Tag::Compound),
            list.clone().map(Tag::List),
            byte.clone().map(Tag::Byte),
            short.clone().map(Tag::Short),
            int.clone().map(Tag::Int),
            long.clone().map(Tag::Long),
            float.clone().map(Tag::Float),
            double.clone().map(Tag::Double),
            bytearray.clone().map(Tag::ByteArray),
            intarray.clone().map(Tag::IntArray),
            longarray.clone().map(Tag::LongArray),
            string.clone().map(Tag::String),
        ))
    }
    impl Tag {
        /// Attempt to parse Minecraft SNBT format into an NBT [Tag].
        /// ### Example
        /// ```
        /// # use rustnbt::{*,tag::*,io::*,snbt::*};
        /// let snbt = r#"
        /// {
        ///     byte1 : 0b,
        ///     byte2 : -10b,
        ///     byte3 : 127b,
        ///     short : 69s,
        ///     int : 420,
        ///     long : 69420,
        ///     float : 3f,
        ///     float2 : 3.14f,
        ///     double : 4d,
        ///     double2 : 4.5d,
        ///     double3 : 5.1,
        ///     bytearray : [B; true, false, 5b],
        ///     intarray : [I; 3, 5, 1],
        ///     longarray : [L; 3l, 4l, 5l],
        ///     lists : [
        ///         ["one", "two", 'three', 'four\\nnewline']
        ///     ],
        ///     compound : {
        ///         "test" : "The quick brown fox jumps over the lazy dog."
        ///     }
        /// }
        /// "#;
        /// if let Ok(Tag::Compound(result)) = Tag::parse(snbt) {
        ///     assert!(result.contains_key(&"double".to_string()));
        /// } else {
        ///     panic!();
        /// }
        /// ```
        pub fn parse<S: AsRef<str>>(source: S) -> Result<Tag, ParseError> {
            match Token::parse(source) {
                Ok(tokens) => {
                    match parser().parse(tokens) {
                        Ok(tag) => Ok(tag),
                        Err(errors) => Err(ParseError::ParseFailure(errors)),
                    }
                }
                Err(errors) => Err(ParseError::TokenizeError(errors)),
            }
        }
    }
    impl FromStr for Tag {
        type Err = ParseError;
        fn from_str(source: &str) -> Result<Self, Self::Err> {
            Tag::parse(source)
        }
    }
    fn is_ident_char(c: &char) -> bool {
        c.is_ascii_alphanumeric() || ['_', '-', '+', '.'].contains(c)
    }
    fn identifier<E: chumsky::Error<char>>() -> impl Parser<char, String, Error = E> {
        filter::<char, _, E>(is_ident_char).repeated().at_least(1).collect::<String>()
    }
    fn strcmp(ignore_case: bool, lhs: &str, rhs: &str) -> bool {
        if lhs.len() != rhs.len() {
            return false;
        }
        if ignore_case { lhs.to_lowercase() == rhs.to_lowercase() } else { lhs == rhs }
    }
    fn keyword<S: AsRef<str>>(
        word: S,
        ignore_case: bool,
    ) -> impl Parser<char, (), Error = Simple<char>> {
        identifier()
            .try_map(move |text, span| {
                if strcmp(ignore_case, word.as_ref(), &text) {
                    Ok(())
                } else {
                    Err(
                        Simple::custom(
                            span,
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Expected keyword: ", ", found "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(&word.as_ref()),
                                            ::core::fmt::ArgumentV1::new_display(&text),
                                        ],
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            })
    }
    fn no_case<C: Container<char>>(chars: C) -> HashSet<char> {
        chars
            .get_iter()
            .fold(
                HashSet::new(),
                |mut set, c| {
                    set.insert(c);
                    if c.is_lowercase() {
                        set.extend(c.to_uppercase());
                    } else {
                        set.extend(c.to_lowercase());
                    }
                    set
                },
            )
    }
    fn one_of_nc<C: Container<char>, E: Error<char>>(
        chars: C,
    ) -> OneOf<char, HashSet<char>, E> {
        one_of(no_case(chars))
    }
    fn none_of_nc<C: Container<char>, E: Error<char>>(
        chars: C,
    ) -> NoneOf<char, HashSet<char>, E> {
        none_of(no_case(chars))
    }
    pub enum ParseError {
        #[error("Found invalid token(s).")]
        TokenizeError(Vec<Simple<char>>),
        #[error("Failed to parse SNBT.")]
        ParseFailure(Vec<Simple<Token>>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParseError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ParseError::TokenizeError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TokenizeError",
                        &__self_0,
                    )
                }
                ParseError::ParseFailure(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ParseFailure",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for ParseError {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for ParseError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                ParseError::TokenizeError(_0) => {
                    __formatter
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["Found invalid token(s)."],
                                &[],
                            ),
                        )
                }
                ParseError::ParseFailure(_0) => {
                    __formatter
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["Failed to parse SNBT."],
                                &[],
                            ),
                        )
                }
            }
        }
    }
}
/// This is the Error type returned from NbtRead and NbtWrite operations that fail.
pub enum NbtError {
    /// Error from std::io::Error.
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// Failure to convert bytes to a UTF-8 string.
    #[error("Failed to read UTF-8 string.")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// Tag type ID was not recognized, and may be part of an unsupported format.
    #[error("Unsupported Tag ID.")]
    Unsupported { id_encountered: u8 },
    /// End marker (0x00) was encountered.
    #[error("Encountered the End tag ID marker.")]
    End,
}
#[allow(unused_qualifications)]
impl std::error::Error for NbtError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        use thiserror::__private::AsDynError;
        #[allow(deprecated)]
        match self {
            NbtError::IoError { 0: source, .. } => {
                std::option::Option::Some(source.as_dyn_error())
            }
            NbtError::FromUtf8Error { 0: source, .. } => {
                std::option::Option::Some(source.as_dyn_error())
            }
            NbtError::Unsupported { .. } => std::option::Option::None,
            NbtError::End { .. } => std::option::Option::None,
        }
    }
}
#[allow(unused_qualifications)]
impl std::fmt::Display for NbtError {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_imports)]
        use thiserror::__private::{DisplayAsDisplay, PathAsDisplay};
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            NbtError::IoError(_0) => {
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&_0.as_display())],
                        ),
                    )
            }
            NbtError::FromUtf8Error(_0) => {
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Failed to read UTF-8 string."],
                            &[],
                        ),
                    )
            }
            NbtError::Unsupported { id_encountered } => {
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(&["Unsupported Tag ID."], &[]),
                    )
            }
            NbtError::End {} => {
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Encountered the End tag ID marker."],
                            &[],
                        ),
                    )
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<std::io::Error> for NbtError {
    #[allow(deprecated)]
    fn from(source: std::io::Error) -> Self {
        NbtError::IoError { 0: source }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<std::string::FromUtf8Error> for NbtError {
    #[allow(deprecated)]
    fn from(source: std::string::FromUtf8Error) -> Self {
        NbtError::FromUtf8Error {
            0: source,
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for NbtError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            NbtError::IoError(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "IoError",
                    &__self_0,
                )
            }
            NbtError::FromUtf8Error(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "FromUtf8Error",
                    &__self_0,
                )
            }
            NbtError::Unsupported { id_encountered: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Unsupported",
                    "id_encountered",
                    &__self_0,
                )
            }
            NbtError::End => ::core::fmt::Formatter::write_str(f, "End"),
        }
    }
}
#[cfg(not(feature = "preserve_order"))]
/// The mapping type used for Tag::Compound.
pub type Map = std::collections::HashMap<String, tag::Tag>;
