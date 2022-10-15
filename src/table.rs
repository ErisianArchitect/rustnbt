use crate::family::*;

/// $id:literal $name:ident $($type_:ty)?
#[macro_export]
macro_rules! tag_info_table {
    ($macro:ident) => {
        $macro!{
            /*
            00  End                 _
            */
            01  Byte                i8                  [Allow<Primitive>, Allow<Byte>, Block<Array>, Block<Special>]
            02  Short               i16                 [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            03  Int                 i32                 [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            04  Long                i64                 [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            05  Float               f32                 [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            06  Double              f64                 [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            07  ByteArray           Vec::<i8>           [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            08  String              String              [Block<Primitive>, Block<Byte>, Block<Array>, Allow<Special>]
            09  List                ListTag             [Block<Primitive>, Block<Byte>, Block<Array>, Allow<Special>]
            10  Compound            Map                 [Block<Primitive>, Block<Byte>, Block<Array>, Allow<Special>]
            11  IntArray            Vec::<i32>          [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            12  LongArray           Vec::<i64>          [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            13  UByte               u8                  [Allow<Primitive>, Allow<Byte>, Block<Array>, Block<Special>]
            14  UShort              u16                 [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            15  UInt                u32                 [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            16  ULong               u64                 [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            17  Bytes               Vec::<u8>           [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            18  ShortArray          Vec::<i16>          [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            19  UShortArray         Vec::<u16>          [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            20  UIntArray           Vec::<u32>          [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            21  ULongArray          Vec::<u64>          [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            22  I128                i128                [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            23  U128                u128                [Allow<Primitive>, Block<Byte>, Block<Array>, Block<Special>]
            24  I128Array           Vec::<i128>         [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            25  U128Array           Vec::<u128>         [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            26  StringArray         Vec::<String>       [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            27  FloatArray          Vec::<f32>          [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
            28  DoubleArray         Vec::<f64>          [Block<Primitive>, Block<Byte>, Allow<Array>, Block<Special>]
        }
    }
}

pub(crate) use tag_info_table;