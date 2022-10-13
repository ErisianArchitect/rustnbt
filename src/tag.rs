// https://wiki.vg/NBT

use crate::io::*;
use indexmap::IndexMap;
use crate::tag_info_table;
use crate::table_arm_filter;

pub type Map = IndexMap<String, Tag>;

// This macro is not for those with weak dispositions.
macro_rules! tag_data {
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
                match self {
                    $(table_arm_filter!( $($type_)? : { ListTag::$title(arr) } else { ListTag::$title } )
                    =>table_arm_filter!( $($type_)? : { arr.len()            } else { 0               } ),)+
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
