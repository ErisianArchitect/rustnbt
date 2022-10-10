use indexmap::IndexMap;

type Map = IndexMap<String, Tag>;

// This macro is not for those with weak dispositions.
macro_rules! tag_info_table {
    ($($id:literal $title:ident $($type_:ty)?)+) => {
        pub(crate) const TAG_NAMES: &[&str] = &[
            $(concat!("TAG_", stringify!($title)),)+
        ];

        pub(crate) const TAG_TITLES: &[&str] = &[
            $(stringify!($title),)+
        ];

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum TagID {
            $($title = $id,)+
        }

        #[derive(Clone)]
        pub enum Tag {
            $($title$(($type_))?,)+
        }

        impl Tag {
            pub fn id(&self) -> TagID {
                match self {
                    $(Tag::$title{..} => TagID::$title,)+
                }
            }
        }

        $($(
            impl From<$type_> for Tag {
                fn from(value: $type_) -> Self {
                    Tag::$title(value)
                }
            }
        )?)+

        #[derive(Clone)]
        pub enum ListTag {
            $($title$((Vec<$type_>))?,)+
        }

        $($(
            impl From<Vec<$type_>> for ListTag {
                fn from(value: Vec<$type_>) -> Self {
                    ListTag::$title(value)
                }
            }
        )?)+

        impl ListTag {
            pub fn id(&self) -> TagID {
                match self {
                    $(ListTag::$title{..} => TagID::$title,)+
                }
            }
        }
    };
}

//  id|title    |optional type_
tag_info_table! {
    00  End
    01  Byte                i8
    02  Short               i16
    03  Int                 i32
    04  Long                i64
    05  Float               f32
    06  Double              f64
    07  ByteArray           Vec<i8>
    08  String              String
    09  List                ListTag
    10  Compound            Map
    11  IntArray            Vec<i32>
    12  LongArray           Vec<i64>
    13  UByte               u8
    14  UShort              u16
    15  UInt                u32
    16  ULong               u64
    17  Bytes               Vec<u8>
    18  ShortArray          Vec<i16>
    19  UShortArray         Vec<u16>
    20  UIntArray           Vec<u32>
    21  ULongArray          Vec<u64>
    22  I128                i128
    23  U128                u128
    24  I128Array           Vec<i128>
    25  U128Array           Vec<u128>
    26  StringArray         Vec<String>
}

impl Tag {
    pub fn title(&self) -> &'static str {
        self.id().title()
    }

    pub fn name(&self) -> &'static str {
        self.id().name()
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Tag::String(String::from(value))
    }
}

impl TagID {
    /// PascalCase title of this TagID.
    pub fn title(&self) -> &'static str {
        TAG_TITLES[self.value()]
    }
    /// In the format of TAG_TagTitle.
    pub fn name(&self) -> &'static str {
        TAG_NAMES[self.value()]
    }
    /// Returns this TagID as a usize.
    pub fn value(&self) -> usize {
        *self as usize
    }
}

/// Trait that gives the serialization size of various values.
pub trait NBTSize {
    fn size_in_bytes(&self) -> usize;
}

// impl NBTSize for:
// String, Map, and ListTag

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

#[cfg(test)]
mod tests {
    #[test]
    fn tag_test() {
        use super::*;
        let tag: Tag = Tag::List(ListTag::from(vec![1, 2, 3, 4]));
        println!("Tag Name: {}", tag.title());
    }
}
