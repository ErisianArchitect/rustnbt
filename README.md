# rustnbt

NBT format library.

This implements Minecraft's NBT format, then adds some tags to the format.

## Coverage

```rs
//ID  Name                Type         
//=====================================
 001  Byte                i8
 002  Short               i16
 003  Int                 i32
 004  Long                i64
 005  Float               f32
 006  Double              f64
 007  ByteArray           Vec::<i8>
 008  String              String
 009  List                ListTag
 010  Compound            Map
 011  IntArray            Vec::<i32>
 012  LongArray           Vec::<i64>
 013  UByte               u8
//==Extensions=========================
 128  UShort              u16
 129  UInt                u32
 130  ULong               u64
 131  Bytes               Vec::<u8>
 132  ShortArray          Vec::<i16>
 133  UShortArray         Vec::<u16>
 134  UIntArray           Vec::<u32>
 135  ULongArray          Vec::<u64>
 136  I128                i128
 137  U128                u128
 138  I128Array           Vec::<i128>
 139  U128Array           Vec::<u128>
 140  StringArray         Vec::<String>
 141  FloatArray          Vec::<f32>
 142  DoubleArray         Vec::<f64>
```
At some point in the future, I hope to write up a spec for the extensions, but it is a logical extension of Minecraft's NBT.

One thing to note is that the extension types have IDs that start at `128`. This is to attempt to prevent collisions with any potential future additions to Minecraft's NBT specification.

### Reason

I needed a library that could serialize and deserialize NBT, and I didn't want to use someone else's library, so I wrote my own and then added some extra functionality as the cherry on top.<br>
Although I wrote it for my own purposes, you are free to use it for your own.

## Before Use

If you prefer that the order of elements in a Compound tag are preserved, you can add the `preserve_order` feature.<br>
If you would like to try out the extensions, you will need the `extensions` feature enabled.

# Example Usage

## Creating Tags.

```rs
let byte = Tag::Byte(i8::MAX);
let short = Tag::Short(i16::MAX);
let int = Tag::Int(69420);
let long = Tag::Long(i64::MAX);
let float = Tag::Float(3.14_f32);
let double = Tag::Double(3.14159265358979_f64);
let bytearray = Tag::ByteArray(vec![1,2,3,4]);
let string = Tag::String(String::from("The quick brown fox jumps over the lazy dog🎈🎄"));
let list = Tag::List(ListTag::from(vec!["One".to_owned(),"Two".to_owned(), "Three".to_owned()]));
let intarray = Tag::IntArray(vec![1,1,2,3,5,8,13,21,34,55,89,144]);
let longarray = Tag::LongArray(
    (0..8)
        .map(|i| (0xFu64 << i) as i64 )
        .collect()
);
// There are alternate ways to create a Tag::Compound. There is also a macro. More on that later.
let compound = Tag::Compound(Map::from([
    ("Byte".to_owned(), byte),
    ("Short".to_owned(), short),
    ("Int".to_owned(), int),
    ("Long".to_owned(), long),
    ("Float".to_owned(), float),
    ("Double".to_owned(), double),
    ("ByteArray".to_owned(), bytearray),
    ("String".to_owned(), string),
    ("List".to_owned(), list),
    ("Compound".to_owned(), Tag::Compound(Map::from([
        ("One".to_owned(), 1.into()),
        ("Two".to_owned(), 2.into()),
        ("Three".to_owned(), 3.into()),
        ("True".to_owned(), true.into()),
        ("False".to_owned(), false.into()),
        ("Empty List".to_owned(), Tag::List(ListTag::Empty)),
    ]))),
    ("IntArray".to_owned(), intarray),
    ("LongArray".to_owned(), longarray),
]));
```

## Reading NBT from a file

```rs
let mut file = File::open(path).expect("Failed to open the file.");
// Get the file size to find an appropriate buffer size.
let size = file.metadata().expect("Failed to unwrap metadata.").len() as usize;
// Max buffer size is 4mib
let buffer_capacity = size.min(rustnbt::mebibytes(4));
let mut reader = BufReader::with_capacity(buffer_capacity, file);
// Attempts to read a NamedTag from the reader.
// A NamedTag is a special type that holds a String name and a Tag.
// This is used to read the format that most NBT is written to file.
let root = NamedTag::nbt_read(reader.get_mut()).expect("Failed to read NBT.");
```

## Writing NBT to a file

```rs
let mut file = File::create(path).expect("Failed to create the file.");
// Restrict buffer capacity to 4mib.
let buffer_capacity = self.root.tag().nbt_size().min(rustnbt::mebibytes(4));
let mut writer = BufWriter::with_capacity(buffer_capacity, file);
root.nbt_write(writer.get_mut()).expect("Failed to write NBT.");
```