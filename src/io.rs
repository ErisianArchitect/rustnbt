// https://wiki.vg/NBT
// https://minecraft.fandom.com/wiki/NBT_format

#[allow(unused)]
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Seek, SeekFrom, Write};
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

const fn kib(size: usize) -> usize {
    size * 1024
}

const fn mib(size: usize) -> usize {
    kib(kib(1)) * size
}

const fn gib(size: usize) -> usize {
    mib(kib(1)) * size
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
    let mut reader = BufReader::with_capacity(length.min(mib(64)), reader);
    reader.take(length as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_bytes<W: Write>(writer: &mut W, data: &[u8]) -> Result<usize, NbtError> {
    let mut writer = BufWriter::with_capacity(data.len().min(mib(64)), writer);
    Ok(writer.write_all(data).map(|_| data.len())?)
}


fn read_array<R: Read, T: NbtRead>(reader: &mut R, length: usize) -> Result<Vec<T>,NbtError> {
    (0..length)
        .map(|_| {
            T::nbt_read(reader)
        })
        .collect()
}

fn write_array<W: Write, T: NbtWrite>(writer: &mut W, data: &[T]) -> Result<usize, NbtError> {
    data.iter()
        .map(|item| item.nbt_write(writer))
        .sum()
}

/// Trait that gives the serialization size of various values.
pub trait NbtSize {
    fn size_in_bytes(&self) -> usize;
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

impl<T: NbtRead + Nbt<Block<Byte>>> NbtRead for Vec<T> {
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
        self.as_str().nbt_write(writer)
    }
}

impl NbtWrite for &str {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        let length: u16 = self.len() as u16;
        length.nbt_write(writer)?;
        Ok(writer.write(self.as_bytes())? + 2)
    }
}

impl<T: NbtWrite + Nbt<Block<Byte>>> NbtWrite for Vec<T> {
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
        Ok(write_bytes(writer, u8slice)?)
    }
}

impl NbtWrite for Vec<u8> {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, NbtError> {
        (self.len() as u32).nbt_write(writer)?;
        Ok(write_bytes(writer, &self)?)
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
            impl NbtSize for $primitive {
                fn size_in_bytes(&self) -> usize {
                    std::mem::size_of::<$primitive>()
                }
            }

            impl NbtSize for Vec<$primitive> {
                fn size_in_bytes(&self) -> usize {
                    self.len() * std::mem::size_of::<$primitive>() + 4
                }
            }

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
    ($($id:literal $title:ident $type_:ty $([$($impl:ty),*])?)+) => {


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
                            ListTag::$title(read_array(reader, length as usize)?)
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
            fn nbt_write_named<'a, W: Write>(&self, writer: &mut W, name: &str) -> Result<usize, NbtError> {
                match self {
                    $(
                        Tag::$title(tag) => {
                            // Ok(
                            //     TagID::$title.nbt_write(&mut writer)? +
                            //     name.into().nbt_write(&mut writer)? +
                            //     tag.nbt_write(writer)?
                            // )
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
                            Tag::$title(<$type_>::nbt_read(reader)?)
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
//include!("table.rs");
#[cfg(test)]
mod tests {
    #![allow(unused)]

    use crate::tag::*;
    use super::*;

    macro_rules! discard {
        ($($_:tt)*) => {
            
        };
    }

    /// A simple test tag that has some basic values.
    fn test_tag() -> Tag {
        let byte = Tag::Byte(43);
        let short = Tag::Short(1023);
        let int = Tag::Int(100000);
        let long = Tag::Long(i64::MAX);
        let float = Tag::Float(3.14);
        let double = Tag::Double(420.69);
        let bytearray = Tag::ByteArray((-128i8..=127).collect());
        let string = Tag::string("Hello, world!");
        let string_list = Tag::List(
            ListTag::String(vec![
                    String::from("One"),
                    String::from("Two"),
                    String::from("Three"),
                ]
            )
        );
        let empty_list = Tag::List(ListTag::Empty);
        let mut compound = Map::new();
        (0..10).for_each(|i| {
            compound.insert(format!("Item{i}").to_owned(), Tag::Int(i));
        });
        let compound = Tag::Compound(compound);
        let compound1 = Tag::compound([
           ("test", 3i8.into()),
           ("wow", Tag::string("Hello, world!")),
        ]);
        let intarray = Tag::IntArray(vec![1000000, 1009300, 1000020]);
        let longarray = Tag::LongArray((0i64..10i64).map(|v| v.wrapping_mul(1000)).collect());
        let mut tagroot = Map::new();
        macro_rules! root {
            ($($name:ident)+) => {
                $(tagroot.insert(String::from(stringify!($name)), $name);)+
            };
        }
        root!{
            compound1
            byte
            short
            int
            long
            float
            double
            bytearray
            string
            string_list
            compound
            intarray
            longarray
            empty_list
        }
        Tag::Compound(tagroot)
    }

    #[test]
    fn tag_test() {
        let tag = test_tag();
        if let Tag::Compound(map) = tag {
            let inner = map.get("string").expect("Not string");
            if let Tag::String(string) = inner {
                println!("{}", string);
            }
        };
    }

    #[test]
    fn timing_test() {
        use std::fs::*;
        use std::time::*;
        let now = Instant::now();
        let tag = test_tag();
        (0..100).for_each(|_| {
            let mut file = File::create("./ignore/test.nbt")
                .expect("Failed to create file.");
            let mut file = BufWriter::new(file);
            let write_size = tag.nbt_write_named(file.get_mut(), "TestRoot")
                .expect("Failed to write named tag.");
            let mut file = File::open("./ignore/test.nbt")
                .expect("Failed to open file.");
            let mut file = BufReader::new(file);
            let read_tag = Tag::nbt_read_named(file.get_mut())
                .expect("Failed to read named tag.");
        });
        let dur = now.elapsed();
        println!("Millis: {}", dur.as_millis());
    }

    #[test]
    fn size_test() {
        let testtag = test_tag();
        println!("Size: {}", testtag.size_in_bytes());
        use std::fs::*;
        let mut file = File::create("test_write.nbt").expect("Failed to create file.");
        let write_size = testtag.nbt_write_named(&mut file, "Root").expect("Failed to write tag.");
        println!("Write size: {}", write_size);
        let mut file = File::open("test_write.nbt").expect("Failed to open file.");
        let (name, tag) = Tag::nbt_read_named(&mut file).expect("Failure.");
        println!("    Name: {name}");
        println!("Tag Type: {}", tag.title());
        println!("    Size: {}", tag.size_in_bytes());
        println!("     Tag: {}", tag);
        println!("   First: {}", testtag.size_in_bytes());
        println!("    Last: {}", tag.size_in_bytes());
    }
}
