// https://wiki.vg/NBT

use crate::family::*;
use crate::tag_info_table;
use crate::Map;
use crate::ThisError;

use num_traits::ToPrimitive;
use num_traits::Zero;
use std::fmt::Debug;
use std::fmt::Display;

/// Marks that a type is directly represented as an NBT tag type.
pub trait NbtType {
    /// The Minecraft NBT type ID.
    const ID: TagID;
    /// Converts to [`Tag`].
    fn nbt(self) -> Tag;
}

/// A trait for encoding an object as NBT.
/// This trait is intended for types that don't have a direct
/// NBT representation, but can be encoded as an NBT tree.
pub trait EncodeNbt {
    /// Encode as NBT.
    /// This typically results in a [`Tag::Compound`], but may result in other [`Tag`] variants.
    fn encode_nbt(&self) -> Tag;
}

/// A trait for decoding NBT into an object.
/// This trait is intended for types that don't have a direct
/// NBT representation, but can be decoded from NBT data.
pub trait DecodeNbt: Sized + EncodeNbt {
    type Error;
    /// Tries to decode from NBT.
    fn decode_nbt(nbt: Tag) -> Result<Self, Self::Error>;
}

/// This is where a majority of the generation for the code in this module happens.
/// It utilizes the table in `\src\table.rs`.
macro_rules! tag_data {
    ($($id:literal $title:ident $type:path [$subtype:ident] [$origin:ident] [$($impl:path),*] [$($attr:meta),*])+) => {
        /// The NBT Tag enum.
        /// To see what types are supported, take a look at `table.rs`.
        #[derive(Clone, Debug)]
        pub enum Tag {
            $(
                $(#[$attr])*
                $title($type),
            )+
        }

        /// The TagID represents the NBT type ID of a Tag.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        pub enum TagID {
            End = 0,
            Unsupported = -1,
            $(
                $(#[$attr])*
                $title = $id,
            )+
        }

        /// Enum type for Tag::List.
        #[derive(Clone, Debug)]
        pub enum ListTag {
            /// Represents a ListTag without any elements.
            Empty,
            $(
                $(#[$attr])*
                $title(Vec<$type>),
            )+
        }

        impl TagID {
            /// PascalCase title of this TagID.
            pub fn title(self) -> &'static str {
                match self {
                    $(
                        $(#[$attr])*
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
                        $(#[$attr])*
                        TagID::$title => concat!("TAG_", stringify!($title)),
                    )+
                    TagID::End => "TAG_End",
                    TagID::Unsupported => "TAG_Unsupported",
                }
            }
        }

        impl Tag {
            /// Returns the NBT type ID.
            pub fn id(&self) -> TagID {
                match self {
                    $(
                        $(#[$attr])*
                        Tag::$title(_) => TagID::$title,
                    )+
                }
            }
        }

        impl ListTag {
            /// Returns the Tag type ID.
            pub fn id(&self) -> TagID {
                match self {
                    ListTag::Empty => TagID::End,
                    $(
                        $(#[$attr])*
                        ListTag::$title(_) => TagID::$title,
                    )+
                }
            }

            /// Returns the number of elements in the list.
            pub fn len(&self) -> usize {
                match self {
                    $(
                        $(#[$attr])*
                        ListTag::$title(list) => list.len(),
                    )+
                    ListTag::Empty => 0,
                }
            }
        }

        impl<T: ToPrimitive> From<T> for TagID {
            fn from(value: T) -> Self {
                match value.to_usize() {
                    $(
                        $(#[$attr])*
                        Some($id) => TagID::$title,
                    )+
                    Some(0) => TagID::End,
                    _ => TagID::Unsupported,
                }
            }
        }

        $(
            $(#[$attr])*
            impl NbtType for $type {
                const ID: TagID = TagID::$title;
                fn nbt(self) -> Tag {
                    self.into()
                }
            }

            $(#[$attr])*
            impl EncodeNbt for $type {
                fn encode_nbt(&self) -> Tag {
                    self.clone().into()
                }
            }

            $(#[$attr])*
            impl DecodeNbt for $type {
                type Error = String;
                fn decode_nbt(tag: Tag) -> Result<Self, String> {
                    if let Tag::$title(tag) = tag {
                        return Ok(tag)
                    }
                    Err(format!("Failed to convert from NBT to {}. Found: {}", stringify!($type), tag.id()))
                }
            }

            $(#[$attr])*
            $(
                impl $impl for $type {}
            )*

            $(#[$attr])*
            impl From<$type> for Tag {
                fn from(value: $type) -> Self {
                    Tag::$title(value)
                }
            }

            /// From a vector to a ListTag.
            $(#[$attr])*
            impl From<Vec<$type>> for ListTag {
                fn from(value: Vec<$type>) -> Self {
                    ListTag::$title(value)
                }
            }

            /// From a slice to a ListTag.
            $(#[$attr])*
            impl From<&[$type]> for ListTag {
                fn from(value: &[$type]) -> Self {
                    ListTag::$title(value.to_vec())
                }
            }

            $(#[$attr])*
            impl TryFrom<Tag> for $type {
                type Error = ();
                fn try_from(value: Tag) -> Result<$type, ()> {
                    if let Tag::$title(inner) = value {
                        return Ok(inner);
                    }
                    Err(())
                }
            }
        )+
    };
}

tag_info_table!(tag_data);

/// Represents a Named NBT Tag, often used as a Tag Root for an NBT file.
#[derive(Clone, Debug)]
pub struct NamedTag {
    pub(crate) name: String,
    pub(crate) tag: Tag,
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

    #[cfg(feature = "extensions")]
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

    /// Get the name of the NamedTag.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Borrow the NamedTag's tag value.
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// Mutably borrow the NamedTag's tag value.
    pub fn tag_mut(&mut self) -> &mut Tag {
        &mut self.tag
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

/// Creates a Tag::Byte from a boolean value.
impl From<bool> for Tag {
    /// Create a Tag::Byte from a boolean value.
    fn from(on: bool) -> Self {
        Tag::Byte(if on { 1 } else { 0 })
    }
}

/// Creates a Tag from &str
impl From<&str> for Tag {
    /// Creates a [Tag::String].
    fn from(value: &str) -> Self {
        Tag::String(String::from(value))
    }
}

/// Creates a NamedTag from (Into<String>, Into<Tag>)
impl<S, T> From<(S,T)> for NamedTag
where
    S: Into<String>,
    T: Into<Tag>
{
    /// Convert to a NamedTag from a Tuple.
    fn from(value: (S, T)) -> Self {
        Self {
            name: value.0.into(),
            tag: value.1.into(),
        }
    }
}

impl TryFrom<Tag> for bool {
    type Error = ();

    /// Tries to create a bool from a Tag value.
    /// The Tag type must be a numeric type, such as `Tag::Byte`, `Tag::Int`, `Tag::Float`, `Tag::U128`, etc.
    /// Returns false for zero, and true for non-zero.
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        Ok(match value {
            Tag::Byte(inner) => !inner.is_zero(),
            Tag::Short(inner) => !inner.is_zero(),
            Tag::Int(inner) => !inner.is_zero(),
            Tag::Long(inner) => !inner.is_zero(),
            Tag::Float(inner) => !inner.is_zero(),
            Tag::Double(inner) => !inner.is_zero(),
            #[cfg(feature = "extensions")]
            Tag::UByte(inner) => !inner.is_zero(),
            #[cfg(feature = "extensions")]
            Tag::UShort(inner) => !inner.is_zero(),
            #[cfg(feature = "extensions")]
            Tag::UInt(inner) => !inner.is_zero(),
            #[cfg(feature = "extensions")]
            Tag::ULong(inner) => !inner.is_zero(),
            #[cfg(feature = "extensions")]
            Tag::I128(inner) => !inner.is_zero(),
            #[cfg(feature = "extensions")]
            Tag::U128(inner) => !inner.is_zero(),
            // [table update]
            _ => return Err(()),
        })
    }
}

impl<S> From<NamedTag> for (S,Tag)
where
    S: From<String> {
    fn from(value: NamedTag) -> Self {
        (S::from(value.name), value.tag)
    }
    
}

impl Display for TagID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}

impl Display for ListTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn value_tests(){
        use crate::tag::*;
        let named: (String, Tag) = NamedTag::with_name("test", Tag::Byte(13)).into();
        assert_eq!(named.0, "test");
        assert!(matches!(named.1, Tag::Byte(13)));
        let byte = Tag::Byte(i8::MAX);
        assert!(matches!(byte, Tag::Byte(i8::MAX)));
        let short = Tag::Short(i16::MAX);
        assert!(matches!(short, Tag::Short(i16::MAX)));
        let int = Tag::Int(69420);
        assert!(matches!(int, Tag::Int(69420)));
        let long = Tag::Long(i64::MAX);
        assert!(matches!(long, Tag::Long(i64::MAX)));
        let float = Tag::Float(3.14_f32);
        let double = Tag::Double(3.14159265358979_f64);
        let bytearray = Tag::ByteArray(vec![1,2,3,4]);
        let list = Tag::List(ListTag::Empty);
        let intarray = Tag::IntArray(vec![1,1,2,3,5,8,13,21,34,55,89,144]);
        let longarray = Tag::LongArray(vec![1,3,3,7, 1337, 13,37, 1,3,37,1,337, 133,7, 1,33,7,13,3,7]);
        let compound = Tag::Compound(Map::from([
            ("Byte".to_owned(), byte.clone()),
            ("Short".to_owned(), short.clone()),
            ("Pi".to_owned(), double.clone()),
        ]));
    }

}