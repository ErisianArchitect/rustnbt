# **WARNING!**

This library is in active development, and breaking changes are possible. If this warning is visible, that means that breaking changes are possible. It's also likely that those breaking changes won't be documented due to me constantly forgetting to document things.<br>
I have a few more planned features that I want to add before the first stable release.

# rustnbt

Minecraft NBT format library with support for SNBT (NBT text based format).<br>
The SNBT portion of the library is currently incomplete as of November 11th, 2022.<br>
It is currently only possible to decode SNBT into NBT. I'm currently in the depths of overengineering hell, so we'll see how long this takes.


https://wiki.vg/NBT <br>
https://minecraft.fandom.com/wiki/NBT_format

This implements Minecraft's NBT format.

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
```
### Reason

I needed a library that could serialize and deserialize NBT, and I didn't want to use someone else's library, so I wrote my own.<br>
Although I wrote it for my own purposes, you are free to use it for your own.

## Before Use

If you prefer that the order of elements in a Compound tag are preserved, you can add the `preserve_order` feature.
This feature will use [indexmap](https://docs.rs/indexmap/latest/indexmap/) to preserve order. This adds a small toll to the size of the Tag enum type, and also incurs a small performance penalty. Minecraft does not specify that tags must be in any particular order, so it is merely a matter of preference. This feature is off by default.<br>

### WARNING!

Currently, there is no functionality to verify the integrity of NBT while reading it. The algorithm just trusts that the NBT is not malformed. This means that the malformed NBT could theoretically tell your program to allocate 2<sup>32</sup>*8 bytes, which may not be favorable.
I plan on adding a function to verify the integrity of NBT before reading it, but for right now you may want to write your own verification methods for deserializing untrusted NBT. For the purpose that I plan on using this library, verification is not yet a necessity, so I plan on adding it in the future when it becomes a necessity.

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
// So let's say you want to create a compound, but you don't want to have
// to type String::from/to_string/to_owned/etc and Tag::from/into/etc.
// Well the compound macro makes it easy to create compound tags
// without hurting your pwecious wittle fingy wingies.
let compound = compound!(
	("One", 1),
	("Two".to_owned(), Tag::Byte(2)),
	("String", "Hello, world!")
);
// There is also a macro for Tag::List, but I don't feel like documenting
// it right now. Oh, alright. Fine. I'll tell show you.
let list_int = list!(
	1,
	2,
	3,
	4
);
// Unfortunately, it won't work with string literals ☹
// Just kidding! Yes it will!
let list_string = list!(
	"One",
	"Two",
	"Three",
	"Four"
);
```

## Reading NBT from a file

```rs
let mut file = File::open(path).expect("Failed to open the file.");
// Get the file size to find an appropriate buffer size.
let size = file.metadata().expect("Failed to unwrap metadata.").len() as usize;
let buffer_capacity = size.min(4096);
let mut reader = BufReader::with_capacity(buffer_capacity, file);
// Attempts to read a NamedTag from the reader.
// A NamedTag is a special type that holds a String name and a Tag.
// This is used to read the format that most NBT is written to file.
let root = reader.read_nbt().expect("Failed to read NBT.");
```

### You can also access the tag through a reference:

```rs
// Get a refernece to the tag
let tag_ref: &Tag = root.tag();
// Get a mutable reference to the tag
let tag_ref_mut: &mut Tag = root.tag_mut();
// Get the tag itself, consuming the NamedTag in the process
let tag: Tag = root.take_tag();
```

### Or if you need to, you can decompose the NamedTag into a tuple:

```rs
let (name, tag) = <(String, Tag)>::from(root);
```

## Writing NBT to a file

```rs
let mut file = File::create(path).expect("Failed to create the file.");
let root_size = root.nbt_size();
let buffer_capacity = root_size.min(4096);
let mut writer = BufWriter::with_capacity(buffer_capacity, file);
let bytes_written = writer.write_nbt(&root).expect("Failed to write NBT.");
println!("Wrote {} bytes.", bytes_written);
```

## Other stuff

If for whatever reason you want to know how large a Tag is when serialized, you can get that information with `NbtSize::nbt_size`.
`NbtSize` is a trait that is implemented for all NBT tag types, as well as for `Tag` and `NamedTag`.
This will tell you exactly how many bytes will be written when an NBT object is written to a writer.