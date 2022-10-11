
#[macro_export]
macro_rules! tag_info_table {
    ($macro:ident) => {
        $macro!{
            00  End
            01  Byte                i8
            02  Short               i16
            03  Int                 i32
            04  Long                i64
            05  Float               f32
            06  Double              f64
            07  ByteArray           Vec<i8>
            08  String              String
            09  List                ListTag
            10  Compound            Map
            11  IntArray            Vec<i32>
            12  LongArray           Vec<i64>
            13  UByte               u8
            14  UShort              u16
            15  UInt                u32
            16  ULong               u64
            17  Bytes               Vec<u8>
            18  ShortArray          Vec<i16>
            19  UShortArray         Vec<u16>
            20  UIntArray           Vec<u32>
            21  ULongArray          Vec<u64>
            22  I128                i128
            23  U128                u128
            24  I128Array           Vec<i128>
            25  U128Array           Vec<u128>
            26  StringArray         Vec<String>
        }
    }
}

pub(crate) use tag_info_table;