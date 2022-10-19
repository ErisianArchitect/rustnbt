// https://wiki.vg/NBT

use crate::family::*;
use crate::io::*;
use crate::tag_info_table;
use crate::Map;
use crate::ThisError;

use num_traits::ToPrimitive;
use std::fmt::Debug;
use std::fmt::Display;

/// Marks that a type is directly represented as an NBT tag type.
pub trait NbtType {
    const ID: TagID;
    fn nbt(self) -> Tag;
}

/// A trait for encoding an object as NBT.
/// This trait is intended for objects that don't have a direct
/// NBT representation, but can be encoded as an NBT tree.
pub trait EncodeNbt {
    /// Encode as NBT.
    fn encode_nbt(&self) -> Tag;
}

/// A trait for decoding NBT into an object.
/// This trait is intended for object that don't have a direct
/// NBT representation, but can be decoded from NBT data.
pub trait DecodeNbt: Sized {
    type Error;
    /// Tries to decode from NBT.
    fn decode_nbt(nbt: Tag) -> Result<Self, Self::Error>;
}

macro_rules! tag_data {
    ($($id:literal $title:ident $type_:path $([$($impl:path),*])?)+) => {

        $(
            impl NbtType for $type_ {
                const ID: TagID = TagID::$title;
                fn nbt(self) -> Tag {
                    self.into()
                }
            }

            impl EncodeNbt for $type_ {
                fn encode_nbt(&self) -> Tag {
                    self.clone().into()
                }
            }

            impl DecodeNbt for $type_ {
                type Error = String;
                fn decode_nbt(tag: Tag) -> Result<Self, String> {
                    if let Tag::$title(tag) = tag {
                        return Ok(tag)
                    }
                    Err(format!("Failed to convert from NBT to {}. Found: {}", stringify!($type_), tag.id()))
                }
            }

            impl TryFrom<Tag> for $type_ {
                type Error = ();
                fn try_from(value: Tag) -> Result<$type_, ()> {
                    if let Tag::$title(inner) = value {
                        return Ok(inner);
                    }
                    Err(())
                }
            }

        )+

        $($($(
            impl $impl for $type_ {}
        )*)?)+

        /// The TagID represents the NBT type ID of a Tag.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        pub enum TagID {
            End = 0,
            Unsupported = -1,
            $(
                $title = $id,
            )+
        }

        impl TagID {
            /// PascalCase title of this TagID.
            pub fn title(self) -> &'static str {
                match self {
                    $(
                        TagID::$title => stringify!($title),
                    )+
                    TagID::End => "End",
                    TagID::Unsupported => "Unsupported",
                }
            }

            /// In the format of TAG_TagTitle.
            pub fn name(self) -> &'static str {
                match self {
                    $(
                        TagID::$title => concat!("TAG_", stringify!($title)),
                    )+
                    TagID::End => "TAG_End",
                    TagID::Unsupported => "TAG_Unsupported",
                }
            }
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

        /// The NBT Tag enum.
        /// To see what types are supported, take a look at `table.rs`.
        #[derive(Clone, Debug)]
        pub enum Tag {
            $($title($type_),)+
        }

        impl Tag {
            /// Returns the NBT type ID.
            pub fn id(&self) -> TagID {
                match self {
                    $(Tag::$title(_) => TagID::$title,)+
                }
            }
        }

        impl Display for Tag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{:#?}", self))
            }
        }

        $(
            impl From<$type_> for Tag {
                fn from(value: $type_) -> Self {
                    Tag::$title(value)
                }
            }
        )+

        /// Enum type for Tag::List.
        #[derive(Clone, Debug)]
        pub enum ListTag {
            /// Represents a ListTag without any elements.
            Empty,
            $($title(Vec<$type_>),)+
        }

        impl Display for ListTag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{:#?}", self))
            }
        }

        $(
            impl From<Vec<$type_>> for ListTag {
                fn from(value: Vec<$type_>) -> Self {
                    ListTag::$title(value)
                }
            }

            impl From<&[$type_]> for ListTag {
                fn from(value: &[$type_]) -> Self {
                    ListTag::$title(value.to_vec())
                }
            }
        )+

        impl ListTag {
            pub fn id(&self) -> TagID {
                match self {
                    ListTag::Empty => TagID::End,
                    $(ListTag::$title(_) => TagID::$title,)+
                }
            }
            pub fn len(&self) -> usize {
                match self {
                    $(ListTag::$title(list) => list.len(),)+
                    ListTag::Empty => 0,
                }
            }
        }
    };
}

tag_info_table!(tag_data);

impl Display for TagID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}

impl TagID {
    /// Returns this TagID as a usize.
    pub fn value(self) -> usize {
        self as usize
    }
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

    /// Create a Tag::ByteArray from the provided iterable.
    pub fn bytearray<T: Into<i8>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
        Tag::ByteArray(it.into_iter().map(T::into).collect())
    }

    /// Create a Tag::ByteArray from the provided iterable.
    pub fn bytes<T: Into<u8>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
        Tag::ByteArray(it.into_iter().map(|value| value.into() as i8).collect())
    }

    /// Create a Tag::IntArray from the provided iterable.
    pub fn intarray<T: Into<i32>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
        Tag::IntArray(it.into_iter().map(T::into).collect())
    }

    /// Create a Tag::ShortArray from the provided iterable.
    pub fn shortarray<T: Into<i16>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
        Tag::ShortArray(it.into_iter().map(T::into).collect())
    }

    /// Create a Tag::LongArray from the provided iterable.
    pub fn longarray<T: Into<i64>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
        Tag::LongArray(it.into_iter().map(T::into).collect())
    }

    /// Create a Tag::String.
    pub fn string<S: Into<String>>(value: S) -> Tag {
        Tag::String(value.into())
    }
    
    /// Create a Tag::List.
    pub fn list<T>(array: T) -> Tag
    where
        T: Into<ListTag>,
    {
        Tag::List(array.into())
    }

    /// Create a Tag::Compound.
    pub fn compound<S, T, IT>(items: IT) -> Tag
    where
        S: Into<String>,
        T: Into<Tag>,
        IT: IntoIterator<Item = (S, T)>,
    {
        let mut result = Map::new();
        items.into_iter().for_each(|(name, tag)| {
            result.insert(name.into(), tag.into());
        });
        Tag::Compound(result)
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Tag::String(String::from(value))
    }
}

/// Represents a Named NBT Tag, often used as a Tag Root for an NBT file.
#[derive(Clone, Debug)]
pub struct NamedTag {
    pub(crate) name: String,
    pub(crate) tag: Tag,
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

    /// Creates a new NamedTag with a name.
    pub fn with_name<S, T>(name: S, tag: T) -> Self
    where
        S: Into<String>,
        T: Into<Tag> {
            Self {
                name: name.into(),
                tag: tag.into(),
            }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn tag_mut(&mut self) -> &mut Tag {
        &mut self.tag
    }

    pub fn set_tag<T: Into<Tag>>(&mut self, tag: T) {
        self.tag = tag.into();
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = name.into();
    }
}

impl<S, T> From<(S, T)> for NamedTag
where
    S: Into<String>,
    T: Into<Tag>,
{
    fn from(value: (S, T)) -> Self {
        Self {
            name: value.0.into(),
            tag: value.1.into(),
        }
    }
}

impl<S: From<String>> From<NamedTag> for (S, Tag) {
    fn from(value: NamedTag) -> Self {
        (S::from(value.name), value.tag)
    }
}