// https://wiki.vg/NBT

use indexmap::IndexMap;

pub type Map = IndexMap<String, Tag>;

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
            pub fn len(&self) -> usize {
                // Gotta use some sub-macros to get this working right.
                // You may be wondering "why?"
                // Well, the answer is simple: Because ListTag::End would be a variant,
                // but there is no associated Vec to the End variant, meaning that we can't
                // match it in an arm to retrieve the len.
                // So we create a special macro to help us sort out that issue.
                macro_rules! len_arm_match {
                    ($list_title:ident $arr_ident:ident $list_type:ty) => {
                        ListTag::$list_title($arr_ident)
                    };
                    ($list_title:ident) => {
                        ListTag::$list_title
                    };
                }
                macro_rules! len_arm_result {
                    ($list_title:ident $arr_ident:ident $list_type:ty) => {
                        $arr_ident.len()
                    };
                    ($list_title:ident) => {
                        0
                    };
                }
                match self {
                    $(len_arm_match!($title $(arr $type_)?) => len_arm_result!($title $(arr $type_)?),)+
                }
            }
        }
    };
}

//  id |title              |optional type_
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
    /// PascalCase title of this TagID.
    pub fn title(&self) -> &'static str {
        self.id().title()
    }
    /// In the format of TAG_TagTitle.
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


#[cfg(test)]
mod tests {
    #[test]
    fn tag_test() {
        use super::*;
        let tag: Tag = Tag::List(ListTag::from(vec![1, 2, 3, 4]));
        println!("Tag Name: {}", tag.title());
    }
}
