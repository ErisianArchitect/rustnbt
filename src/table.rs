/// Macro used for accessing the tag info table which contains information
/// about all the tag types, their tag name, their type, and the last column is for implementations.
/// Implementations are applied to the type of the tag.
/// So for example: Tag::Byte is i8, so we would implement Primitive for i8.
/// Implementations:
///            Primitive: Marker that tells the compiler that this is a known-sized type and should be treated as such.
///              NonByte: Marker that tells the compiler that this is a type that is not 8-bits wide.
///     NonBytePrimitive: Marker that combines the Primitive and NonByte marker.
/// Usage:
/// ```no_run
/// macro_rules! read_table {
///     //             $id: The tag ID that is written to file to mark a type. These are distinct integer values.
///     //                  Typically the ID will be a single 8-bit value, but as the extensions become more advanced, this may change.
///     //          $title: This is the title of this NBT type. This is different from the type name, and is used as the names of 
///     //                  variants in the TagID and Tag enums.
///     //           $type: The type that the tag holds and serializes/deserializes.
///     //        $subtype: The category that this tag type exists in. Either: scalar, array, or other.
///     //         $origin: This value will either be "minecraft" or "extension".
///     //                  This is used to control whether or not code is emitted depending on if the "extensions" feature is enabled.
///     //           $impl: These are trait implementations that are applied to the types. These traits do not do anything besides act as markers.
///     //           $attr: These are the attributes applied to code. This is so that we can enable or disable extensions.
///     ($($id:literal $title:ident $type:path [$subtype:ident] [$origin:ident] [$($impl:path),*] [$($attr:meta)?])+) => {
///         // generation code here.
///     }
/// }
///
/// tag_info_table!(read_table);
/// ```
#[macro_export]
// [IMPORTANT!] If this table is updated, search for `[table update]` within the project to find places that the code might need to be updated.
macro_rules! tag_info_table {
    ($macro:ident) => {
        $macro! {
//ID    Title           Type                                    [Subtype] [Origin   ] [Implementation                  ] [Attribute                  ]
0001    Byte            i8                                      [scalar ] [minecraft] [$crate::family::Primitive       ] [/*-[No Attribute]------- */]
0002    Short           i16                                     [scalar ] [minecraft] [$crate::family::NonBytePrimitive] [/*-[No Attribute]------- */]
0003    Int             i32                                     [scalar ] [minecraft] [$crate::family::NonBytePrimitive] [/*-[No Attribute]------- */]
0004    Long            i64                                     [scalar ] [minecraft] [$crate::family::NonBytePrimitive] [/*-[No Attribute]------- */]
0005    Float           f32                                     [scalar ] [minecraft] [$crate::family::NonBytePrimitive] [/*-[No Attribute]------- */]
0006    Double          f64                                     [scalar ] [minecraft] [$crate::family::NonBytePrimitive] [/*-[No Attribute]------- */]
0007    ByteArray       std::vec::Vec::<i8>                     [array  ] [minecraft] [$crate::family::NonByte         ] [/*-[No Attribute]------- */]
0008    String          std::string::String                     [other  ] [minecraft] [$crate::family::NonByte         ] [/*-[No Attribute]------- */]
0009    List            $crate::tag::ListTag                    [other  ] [minecraft] [$crate::family::NonByte         ] [/*-[No Attribute]------- */]
0010    Compound        $crate::Map                             [other  ] [minecraft] [$crate::family::NonByte         ] [/*-[No Attribute]------- */]
0011    IntArray        std::vec::Vec::<i32>                    [array  ] [minecraft] [$crate::family::NonByte         ] [/*-[No Attribute]------- */]
0012    LongArray       std::vec::Vec::<i64>                    [array  ] [minecraft] [$crate::family::NonByte         ] [/*-[No Attribute]------- */]
0128    UByte           u8                                      [scalar ] [extension] [$crate::family::Primitive       ] [cfg(feature = "extensions")]
0129    UShort          u16                                     [scalar ] [extension] [$crate::family::NonBytePrimitive] [cfg(feature = "extensions")]
0130    UInt            u32                                     [scalar ] [extension] [$crate::family::NonBytePrimitive] [cfg(feature = "extensions")]
0131    ULong           u64                                     [scalar ] [extension] [$crate::family::NonBytePrimitive] [cfg(feature = "extensions")]
0132    Bytes           std::vec::Vec::<u8>                     [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0133    ShortArray      std::vec::Vec::<i16>                    [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0134    UShortArray     std::vec::Vec::<u16>                    [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0135    UIntArray       std::vec::Vec::<u32>                    [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0136    ULongArray      std::vec::Vec::<u64>                    [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0137    I128            i128                                    [scalar ] [extension] [$crate::family::NonBytePrimitive] [cfg(feature = "extensions")]
0138    U128            u128                                    [scalar ] [extension] [$crate::family::NonBytePrimitive] [cfg(feature = "extensions")]
0139    I128Array       std::vec::Vec::<i128>                   [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0140    U128Array       std::vec::Vec::<u128>                   [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0141    StringArray     std::vec::Vec::<std::string::String>    [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0142    FloatArray      std::vec::Vec::<f32>                    [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
0143    DoubleArray     std::vec::Vec::<f64>                    [array  ] [extension] [$crate::family::NonByte         ] [cfg(feature = "extensions")]
        }
    };
}

pub use tag_info_table;
