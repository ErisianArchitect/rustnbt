// https://wiki.vg/NBT

use crate::io::*;
use indexmap::IndexMap;
use crate::table::tag_info_table;

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
            pub fn size_in_bytes(&self) -> usize {
                macro_rules! arm_match {
                    ($tag_title:ident $item_ident:ident $_:ty) => {
                        Tag::$tag_title($item_ident)
                    };
                    ($tag_title:ident) => {
                        Tag::$tag_title
                    };
                }
                macro_rules! arm_result {
                    ($tag_title:ident $item_ident:ident $_:ty) => {
                        $item_ident.size_in_bytes()
                    };
                    ($tag_title:ident) => {
                        0
                    };
                }
                match self {
                    $(arm_match!($title $(item $type_)?) => arm_result!($title $(item $type_)?),)+
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
                macro_rules! arm_match {
                    ($list_title:ident $arr_ident:ident $list_type:ty) => {
                        ListTag::$list_title($arr_ident)
                    };
                    ($list_title:ident) => {
                        ListTag::$list_title
                    };
                }
                macro_rules! arm_result {
                    ($list_title:ident $arr_ident:ident $list_type:ty) => {
                        $arr_ident.len()
                    };
                    ($list_title:ident) => {
                        0
                    };
                }
                match self {
                    $(arm_match!($title $(arr $type_)?) => arm_result!($title $(arr $type_)?),)+
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
