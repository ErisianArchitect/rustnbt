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
// [IMPORTANT!] If this table is updated, search for `[table update]` within the project to find places that the code might need to be updated.
macro_rules! tag_info_table {
    ($macro:ident) => {
        $macro! {
            //id  Name                Type                [Implementations...]
            0001  Byte                i8                  [Primitive]
            0002  Short               i16                 [NonBytePrimitive]
            0003  Int                 i32                 [NonBytePrimitive]
            0004  Long                i64                 [NonBytePrimitive]
            0005  Float               f32                 [NonBytePrimitive]
            0006  Double              f64                 [NonBytePrimitive]
            0007  ByteArray           Vec::<i8>           [NonByte]
            0008  String              String              [NonByte]
            0009  List                ListTag             [NonByte]
            0010  Compound            Map                 [NonByte]
            0011  IntArray            Vec::<i32>          [NonByte]
            0012  LongArray           Vec::<i64>          [NonByte]
            0013  UByte               u8                  [Primitive]
            0014  UShort              u16                 [NonBytePrimitive]
            0015  UInt                u32                 [NonBytePrimitive]
            0016  ULong               u64                 [NonBytePrimitive]
            0017  Bytes               Vec::<u8>           [NonByte]
            0018  ShortArray          Vec::<i16>          [NonByte]
            0019  UShortArray         Vec::<u16>          [NonByte]
            0020  UIntArray           Vec::<u32>          [NonByte]
            0021  ULongArray          Vec::<u64>          [NonByte]
            0022  I128                i128                [NonBytePrimitive]
            0023  U128                u128                [NonBytePrimitive]
            0024  I128Array           Vec::<i128>         [NonByte]
            0025  U128Array           Vec::<u128>         [NonByte]
            0026  StringArray         Vec::<String>       [NonByte]
            0027  FloatArray          Vec::<f32>          [NonByte]
            0028  DoubleArray         Vec::<f64>          [NonByte]
        }
    };
}

pub(crate) use tag_info_table;
