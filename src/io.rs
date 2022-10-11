// https://wiki.vg/NBT

#[allow(unused)]
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Seek, SeekFrom, Write};

use crate::{tag::*, tag_info_table};

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

pub trait NBTWrite {
    fn nbt_write<W: Write>(&self, writer: &mut W) -> Result<usize, Error>;
}

impl NBTSize for String {
    fn size_in_bytes(&self) -> usize {
        2usize + self.len()
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

macro_rules! tag_io {
    ($($id:literal $title:ident $($type_:ty)?)+) => {
        impl NBTSize for ListTag {
            fn size_in_bytes(&self) -> usize {
                macro_rules! arm_match {
                    ($tag_title:ident $item_ident:ident $_:ty) => {
                        ListTag::$tag_title($item_ident)
                    };
                    ($tag_title:ident) => {
                        ListTag::$tag_title
                    };
                }
                macro_rules! arm_result {
                    ($tag_title:ident $item_ident:ident $_:ty) => {
                        $item_ident.iter().map(|item| item.size_in_bytes()).sum::<usize>() + 5
                    };
                    ($tag_title:ident) => {
                        5
                    };
                }
                match self {
                    $(arm_match!($title $(item $type_)?) => arm_result!($title $(item $type_)?),)+
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
