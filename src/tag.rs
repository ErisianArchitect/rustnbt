// https://wiki.vg/NBT

use crate::io::*;
use indexmap::IndexMap;
use crate::tag_info_table;
use num_traits::{
    ToPrimitive,

};

pub type Map = IndexMap<String, Tag>;

// This macro is not for those with weak dispositions.
macro_rules! tag_data {
    ($($id:literal $title:ident $type_:ty)+) => {
        pub(crate) const TAG_NAMES: &[&str] = &[
            "TAG_End",
            $(concat!("TAG_", stringify!($title)),)+
        ];

        pub(crate) const TAG_TITLES: &[&str] = &[
            "End",
            $(stringify!($title),)+
        ];

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum TagID {
            End = 0,
            Unsupported = -1,
            $($title = $id,)+
        }

        impl<T: ToPrimitive> From<T> for TagID {
            fn from(value: T) -> Self {
                match value.to_usize() {
                    $(
                        Some($id) => TagID::$title,
                    )+
                    Some(0) => TagID::End,
                    _ => TagID::Unsupported,
                }
            }
        }

        #[derive(Clone)]
        pub enum Tag {
            $($title($type_),)+
        }

        impl Tag {
            pub fn id(&self) -> TagID {
                match self {
                    $(Tag::$title(_) => TagID::$title,)+
                }
            }
        }

        $(
            impl From<$type_> for Tag {
                fn from(value: $type_) -> Self {
                    Tag::$title(value)
                }
            }
        )+

        #[derive(Clone)]
        pub enum ListTag {
            End,
            $($title(Vec<$type_>),)+
        }

        $(
            impl From<Vec<$type_>> for ListTag {
                fn from(value: Vec<$type_>) -> Self {
                    ListTag::$title(value)
                }
            }
        )+

        impl ListTag {
            pub fn id(&self) -> TagID {
                match self {
                    ListTag::End => TagID::End,
                    $(ListTag::$title(_) => TagID::$title,)+
                }
            }
            pub fn len(&self) -> usize {
                match self {
                    $(ListTag::$title(list) => list.len(),)+
                    ListTag::End => 0,
                }
            }
        }
    };
}

tag_info_table!(tag_data);

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
    pub fn value(self) -> usize {
        self as usize
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
