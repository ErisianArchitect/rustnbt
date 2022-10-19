use crate::family::*;

// The next line is the pattern to capture the lines of the table.
// ($($id:literal $title:ident $type_:path $([$($impl:path),*])?)+) => {}

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
///     ($($id:literal $title:ident $type_:path $([$($impl:path),*])?)+) => {
///         // generation code here.
///     }
/// }
///
/// tag_info_table!(read_table);
/// ```
#[macro_export]
macro_rules! tag_info_table {
    ($macro:ident) => {
        $macro! {
            //id  Name                Type                [Implementations...]
              01  Byte                i8                  [Primitive]
              02  Short               i16                 [NonBytePrimitive]
              03  Int                 i32                 [NonBytePrimitive]
              04  Long                i64                 [NonBytePrimitive]
              05  Float               f32                 [NonBytePrimitive]
              06  Double              f64                 [NonBytePrimitive]
              07  ByteArray           Vec::<i8>           [NonByte]
              08  String              String              [NonByte]
              09  List                ListTag             [NonByte]
              10  Compound            Map                 [NonByte]
              11  IntArray            Vec::<i32>          [NonByte]
              12  LongArray           Vec::<i64>          [NonByte]
              13  UByte               u8                  [Primitive]
              14  UShort              u16                 [NonBytePrimitive]
              15  UInt                u32                 [NonBytePrimitive]
              16  ULong               u64                 [NonBytePrimitive]
              17  Bytes               Vec::<u8>           [NonByte]
              18  ShortArray          Vec::<i16>          [NonByte]
              19  UShortArray         Vec::<u16>          [NonByte]
              20  UIntArray           Vec::<u32>          [NonByte]
              21  ULongArray          Vec::<u64>          [NonByte]
              22  I128                i128                [NonBytePrimitive]
              23  U128                u128                [NonBytePrimitive]
              24  I128Array           Vec::<i128>         [NonByte]
              25  U128Array           Vec::<u128>         [NonByte]
              26  StringArray         Vec::<String>       [NonByte]
              27  FloatArray          Vec::<f32>          [NonByte]
              28  DoubleArray         Vec::<f64>          [NonByte]
        }
    };
}

pub use tag_info_table;
