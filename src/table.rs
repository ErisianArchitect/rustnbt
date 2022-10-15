use crate::family::*;

/// $id:literal $name:ident $($type_:ty)?
#[macro_export]
macro_rules! tag_info_table {
    ($macro:ident) => {
        $macro!{
            /*
            00  End                 _
            */
            01  Byte                i8                  [Allow<Primitive>, Allow<Byte>, Block<Array>]
            02  Short               i16                 [Allow<Primitive>, Block<Byte>, Block<Array>]
            03  Int                 i32                 [Allow<Primitive>, Block<Byte>, Block<Array>]
            04  Long                i64                 [Allow<Primitive>, Block<Byte>, Block<Array>]
            05  Float               f32                 [Allow<Primitive>, Block<Byte>, Block<Array>]
            06  Double              f64                 [Allow<Primitive>, Block<Byte>, Block<Array>]
            07  ByteArray           Vec::<i8>           [Block<Primitive>, Block<Byte>, Allow<Array>]
            08  String              String              [Block<Primitive>, Block<Byte>, Block<Array>]
            09  List                ListTag             [Block<Primitive>, Block<Byte>, Block<Array>]
            10  Compound            Map                 [Block<Primitive>, Block<Byte>, Block<Array>]
            11  IntArray            Vec::<i32>          [Block<Primitive>, Block<Byte>, Allow<Array>]
            12  LongArray           Vec::<i64>          [Block<Primitive>, Block<Byte>, Allow<Array>]
            13  UByte               u8                  [Allow<Primitive>, Allow<Byte>, Block<Array>]
            14  UShort              u16                 [Allow<Primitive>, Block<Byte>, Block<Array>]
            15  UInt                u32                 [Allow<Primitive>, Block<Byte>, Block<Array>]
            16  ULong               u64                 [Allow<Primitive>, Block<Byte>, Block<Array>]
            17  Bytes               Vec::<u8>           [Block<Primitive>, Block<Byte>, Allow<Array>]
            18  ShortArray          Vec::<i16>          [Block<Primitive>, Block<Byte>, Allow<Array>]
            19  UShortArray         Vec::<u16>          [Block<Primitive>, Block<Byte>, Allow<Array>]
            20  UIntArray           Vec::<u32>          [Block<Primitive>, Block<Byte>, Allow<Array>]
            21  ULongArray          Vec::<u64>          [Block<Primitive>, Block<Byte>, Allow<Array>]
            22  I128                i128                [Allow<Primitive>, Block<Byte>, Block<Array>]
            23  U128                u128                [Allow<Primitive>, Block<Byte>, Block<Array>]
            24  I128Array           Vec::<i128>         [Block<Primitive>, Block<Byte>, Allow<Array>]
            25  U128Array           Vec::<u128>         [Block<Primitive>, Block<Byte>, Allow<Array>]
            26  StringArray         Vec::<String>       [Block<Primitive>, Block<Byte>, Allow<Array>]
            27  FloatArray          Vec::<f32>          [Block<Primitive>, Block<Byte>, Allow<Array>]
            28  DoubleArray         Vec::<f64>          [Block<Primitive>, Block<Byte>, Allow<Array>]
        }
    }
}

pub(crate) use tag_info_table;