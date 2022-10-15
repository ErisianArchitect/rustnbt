use crate::family::*;

// ($($id:literal $title:ident $type_:ty $([$($family:ty),*])?)+) => {}
#[macro_export]
macro_rules! tag_info_table {($macro:ident) => {$macro!{
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
}}}

pub(crate) use tag_info_table;