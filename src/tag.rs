// https://wiki.vg/NBT

use crate::family::*;
use crate::io::NbtWrite;
use crate::tag_info_table;
use crate::Map;
use crate::ThisError;

use num_traits::ToPrimitive;
use num_traits::Zero;
use std::fmt::Debug;
use std::fmt::Display;
use std::string;

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

/// This is where a majority of the generation for the code in this module happens.
/// It utilizes the table in `\src\table.rs`.
macro_rules! tag_data {
    ($($id:literal $title:ident $type:path [$($impl:path)?] [$($attr:meta)?])+) => {
        #[doc = "
        The NBT Tag enum.<br>
        To see what types are supported, take a look at the table in [tag_info_table] located in [`/src/table.rs`].
        "]
        #[derive(Clone, Debug)]
        pub enum Tag {
            $(
                // There exists the temptation to add Tag::None/Empty/Null to be able to represent
                // a nothing value, but I don't think that would be useful at all.
                // I'm just writing this comment here in case future me has the same temptation and actually
                // wishes to follow through: Don't bother. It's probably not really necessary.
                $(#[$attr])?
                $title($type),
            )+
        }

        #[doc = "The NBT tag type ID."]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        pub enum TagID {
            $(
                $(#[$attr])?
                $title = $id,
            )+
        }

        #[doc = "Enum type for [Tag::List]."]
        #[derive(Clone, Debug)]
        pub enum ListTag {
            #[doc = "Represents a ListTag without any elements."]
            Empty,
            $(
                $(#[$attr])?
                $title(Vec<$type>),
            )+
        }

        impl TagID {
            #[doc = "PascalCase title of this [TagID]."]
            pub fn title(self) -> &'static str {
                match self {
                    $(
                        $(#[$attr])?
                        TagID::$title => stringify!($title),
                    )+
                }
            }

            #[doc = "In the format of TAG_TagTitle."] 
            pub fn name(self) -> &'static str {
                match self {
                    $(
                        $(#[$attr])?
                        TagID::$title => concat!("TAG_", stringify!($title)),
                    )+
                }
            }
        }

        impl Tag {
            #[doc = "Returns the NBT type ID."]
            pub fn id(&self) -> TagID {
                match self {
                    $(
                        $(#[$attr])?
                        Tag::$title(_) => TagID::$title,
                    )+
                }
            }
        }

        impl ListTag {
            #[doc = "Returns the list type ID."]
            pub fn id(&self) -> TagID {
                match self {
                    ListTag::Empty => TagID::Byte,
                    $(
                        $(#[$attr])?
                        ListTag::$title(_) => TagID::$title,
                    )+
                }
            }

            #[doc = "
            Returns the number of elements in the list.<br>
            Returns `0` for [ListTag::Empty].
            "]
            pub fn len(&self) -> usize {
                match self {
                    $(
                        $(#[$attr])?
                        ListTag::$title(list) => list.len(),
                    )+
                    ListTag::Empty => 0,
                }
            }
        }

        impl TryFrom<u8> for TagID {
            type Error = $crate::NbtError;
            #[doc = "
            Attempts to create a [TagID] from a [u8].<br>
            Errors:
            - [NbtError::End]
            - [NbtError::Unsupported] { id_encountered }
            "]
            fn try_from(value: u8) -> Result<Self,Self::Error> {
                match value {
                    $(
                        $(#[$attr])?
                        $id => Ok(TagID::$title),
                    )+
                    0 => Err($crate::NbtError::End),
                    other => Err($crate::NbtError::Unsupported { id_encountered: other }),
                }
            }
        }

        $(
            // NbtType implementations for all NBT representable types.
            $(#[$attr])?
            impl NbtType for $type {
                #[doc = "The tag type ID."]
                const ID: TagID = TagID::$title;
                #[doc = "Converts to an NBT [Tag]."]
                fn nbt(self) -> Tag {
                    self.into()
                }
            }

            // Implements non-consuming NBT encoders for all NBT representable types.
            // It's likely that you may want to keep the old value around rather
            // than consuming it and converting it to NBT. This is implemented for reference
            // types for that exact scenario.
            $(#[$attr])?
            impl EncodeNbt for &$type {
                #[doc = "Encodes self as an NBT tag."]
                fn encode_nbt(self) -> Tag {
                    self.clone().into()
                }
            }

            // Implements consuming NBT decoders for all NBT representable types.
            // The reason the decoder consumes the Tag is because a non-consuming decoder would
            // still need to clone the tag in order to return a result. It may be preferable to
            // not be forced to do a clone, so you're allowed to pass in the Tag to be consumed
            // so that you can avoid that clone, otherwise you can clone the tag yourself
            // before decoding it.
            $(#[$attr])?
            impl DecodeNbt for $type {
                type Error = ();
                #[doc = "Attempts to decode the tag."]
                fn decode_nbt(tag: Tag) -> Result<Self, ()> {
                    if let Tag::$title(tag) = tag {
                        return Ok(tag)
                    }
                    Err(())
                }
            }

            // Application of marker traits.
            // The marker traits are defined in `/src/family.rs`.
            // The marker traits are simply used to constrain trait bounds for implementations.
            // Example:
            // ```no_run
            // impl<T: crate::family::Primitive> SomeTrait for T {
            //     // ...
            // }
            // ```
            $(#[$attr])?
            $(
                impl $impl for $type {}
            )?

            // Create a Tag from its representational type.
            $(#[$attr])?
            impl From<$type> for Tag {
                #[doc = concat!("Create a [Tag::", stringify!($title), "] from its representational type.")]
                fn from(value: $type) -> Self {
                    Tag::$title(value)
                }
            }

            // Create a ListTag from a Vector
            $(#[$attr])?
            impl From<Vec<$type>> for ListTag {
                #[doc = concat!("Create a [ListTag::", stringify!($title), "] from its representational vector type.")]
                fn from(value: Vec<$type>) -> Self {
                    ListTag::$title(value)
                }
            }

            // Create a ListTag from a slice.
            $(#[$attr])?
            impl From<&[$type]> for ListTag {
                #[doc = concat!("Create a [ListTag::", stringify!($title), "] from its representational slice type.")]
                fn from(value: &[$type]) -> Self {
                    ListTag::$title(value.to_vec())
                }
            }

            // Try to recreate a representational type from an NBT Tag.
            $(#[$attr])?
            impl TryFrom<Tag> for $type {
                type Error = ();
                #[doc = "Tries to recreate a representational type from a [Tag]."]
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
/// This is also sometimes called a root tag.
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

    /// Creates a NamedTag with the supplied name.
    pub fn with_name<S, T>(name: S, tag: T) -> Self
    where
        S: Into<String>,
        T: Into<Tag> {
            Self {
                name: name.into(),
                tag: tag.into(),
            }
    }

    // When this is the root tag of an NBT file, the name is often empty.
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

/// Creates a (From<String>, Tag) from a NamedTag.
impl<S> From<NamedTag> for (S,Tag)
where
    S: From<String> {
    /// Convert to a Tuple from a NamedTag.
    fn from(value: NamedTag) -> Self {
        (S::from(value.name), value.tag)
    }
    
}

impl TagID {
    /// Returns this TagID as an isize.
    /// (Tag::Unsupported has a value of -1)
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
    /// If the provided value cannot be converted to an [i8], then `Tag::Byte(0)` will be returned.
    pub fn byte<T: ToPrimitive>(value: T) -> Tag {
        if let Some(value) = value.to_i8() {
            Tag::Byte(value)
        } else {
            // TODO: What should happen if the above operation fails?
            Tag::Byte(0)
        }
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
        if let Some(value) = value.to_i32() {
            Tag::Int(value)
        } else {
            Tag::Int(0)
        }
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

    #[cfg(feature = "extensions")]
    /// Create a [Tag::ShortArray] from the provided iterable.
    pub fn shortarray<T: Into<i16>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
        Tag::ShortArray(it.into_iter().map(T::into).collect())
    }

    /// Create a [Tag::LongArray] from the provided iterable.
    pub fn longarray<T: Into<i64>, IT: IntoIterator<Item = T>>(it: IT) -> Tag {
        Tag::LongArray(it.into_iter().map(T::into).collect())
    }

    /// Create a [Tag::String].
    pub fn string<S: Into<String>>(value: S) -> Tag {
        Tag::String(value.into())
    }
    
    // TODO: I don't think that I can make this work with an iterator, but I sure would like to try.
    /// Create a [Tag::List].
    pub fn list<T>(array: T) -> Tag
    where
        T: Into<ListTag>,
    {
        Tag::List(array.into())
    }

    /// Create a [Tag::Compound].
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

impl Display for NamedTag {
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