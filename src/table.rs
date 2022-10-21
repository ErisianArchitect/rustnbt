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
///     ($($id:literal $title:ident $type:path [$subtype:ident] [$origin:ident] [$($impl:path),*] [$($attr:meta),*])+) => {
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
//ID    Title           Type            [Subtype]       [Origin   ]     [Implementations...]
0001    Byte            i8              [scalar ]       [minecraft]     [Primitive]         []
0002    Short           i16             [scalar ]       [minecraft]     [NonBytePrimitive]  []
0003    Int             i32             [scalar ]       [minecraft]     [NonBytePrimitive]  []
0004    Long            i64             [scalar ]       [minecraft]     [NonBytePrimitive]  []
0005    Float           f32             [scalar ]       [minecraft]     [NonBytePrimitive]  []
0006    Double          f64             [scalar ]       [minecraft]     [NonBytePrimitive]  []
0007    ByteArray       Vec::<i8>       [array  ]       [minecraft]     [NonByte]           []
0008    String          String          [other  ]       [minecraft]     [NonByte]           []
0009    List            ListTag         [other  ]       [minecraft]     [NonByte]           []
0010    Compound        Map             [other  ]       [minecraft]     [NonByte]           []
0011    IntArray        Vec::<i32>      [array  ]       [minecraft]     [NonByte]           []
0012    LongArray       Vec::<i64>      [array  ]       [minecraft]     [NonByte]           []
0013    UByte           u8              [scalar ]       [extension]     [Primitive]         [cfg(feature = "extension")]
0128    UShort          u16             [scalar ]       [extension]     [NonBytePrimitive]  [cfg(feature = "extension")]
0129    UInt            u32             [scalar ]       [extension]     [NonBytePrimitive]  [cfg(feature = "extension")]
0130    ULong           u64             [scalar ]       [extension]     [NonBytePrimitive]  [cfg(feature = "extension")]
0131    Bytes           Vec::<u8>       [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0132    ShortArray      Vec::<i16>      [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0133    UShortArray     Vec::<u16>      [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0134    UIntArray       Vec::<u32>      [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0135    ULongArray      Vec::<u64>      [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0136    I128            i128            [scalar ]       [extension]     [NonBytePrimitive]  [cfg(feature = "extension")]
0137    U128            u128            [scalar ]       [extension]     [NonBytePrimitive]  [cfg(feature = "extension")]
0138    I128Array       Vec::<i128>     [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0139    U128Array       Vec::<u128>     [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0140    StringArray     Vec::<String>   [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0141    FloatArray      Vec::<f32>      [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
0142    DoubleArray     Vec::<f64>      [array  ]       [extension]     [NonByte]           [cfg(feature = "extension")]
        }
    };
}

#[macro_export]
macro_rules! unwrap_block {
    ({$($tok:tt)*}) => {
        $($tok)*
    };
}

#[macro_export]
macro_rules! match_origin {
    (extension;  $(extension => $extension:block)? $(minecraft => $minecraft:block)?) => {
        $(
            unwrap_block!{$extension}
        )?
    };
    (minecraft; $(extension => $extension:block)? $(minecraft => $minecraft:block)?) => {
        $(
            unwrap_block!{$minecraft}
        )?
    };
}

#[macro_export]
macro_rules! match_subtype {
    (scalar; $(scalar = $scalar:block)? $(array = $array:block)? $(other = $other:block)?) => {
        unwrap_block!{$scalar}
    };
    (array; $(scalar = $scalar:block)? $(array = $array:block)? $(other = $other:block)?) => {
        unwrap_block!{$array}
    };
    (other; $(scalar = $scalar:block)? $(array = $array:block)? $(other = $other:block)?) => {
        unwrap_block!{$other}
    };
}

pub use match_subtype;
pub use unwrap_block;
pub use match_origin;
pub use tag_info_table;
