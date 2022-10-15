# rustnbt
NBT format library.

This implements Minecraft's NBT format, then adds some tags to the format.

# Coverage
```rs
//id  Name                Type                [Implementations (This is for codegen)...]
  01  Byte                i8                  [Include<Primitive>, Include<Byte>, Exclude<Array>, Exclude<Special>]
  02  Short               i16                 [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  03  Int                 i32                 [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  04  Long                i64                 [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  05  Float               f32                 [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  06  Double              f64                 [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  07  ByteArray           Vec::<i8>           [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  08  String              String              [Exclude<Primitive>, Exclude<Byte>, Exclude<Array>, Include<Special>]
  09  List                ListTag             [Exclude<Primitive>, Exclude<Byte>, Exclude<Array>, Include<Special>]
  10  Compound            Map                 [Exclude<Primitive>, Exclude<Byte>, Exclude<Array>, Include<Special>]
  11  IntArray            Vec::<i32>          [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  12  LongArray           Vec::<i64>          [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  13  UByte               u8                  [Include<Primitive>, Include<Byte>, Exclude<Array>, Exclude<Special>]
  14  UShort              u16                 [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  15  UInt                u32                 [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  16  ULong               u64                 [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  17  Bytes               Vec::<u8>           [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  18  ShortArray          Vec::<i16>          [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  19  UShortArray         Vec::<u16>          [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  20  UIntArray           Vec::<u32>          [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  21  ULongArray          Vec::<u64>          [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  22  I128                i128                [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  23  U128                u128                [Include<Primitive>, Exclude<Byte>, Exclude<Array>, Exclude<Special>]
  24  I128Array           Vec::<i128>         [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  25  U128Array           Vec::<u128>         [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  26  StringArray         Vec::<String>       [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  27  FloatArray          Vec::<f32>          [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  28  DoubleArray         Vec::<f64>          [Exclude<Primitive>, Exclude<Byte>, Include<Array>, Exclude<Special>]
  ```