#![allow(unused)]

use rustnbt::{io::*, tag::*,Map};

use std::{
    fs::File,
    io::{BufReader, BufWriter, Cursor, Error, Read, Write},
    ops::Div,
    time::{Duration, Instant},
};

#[inline(always)]
fn timer<F: FnOnce()>(callback: F) -> Duration {
    let now = Instant::now();
    callback();
    now.elapsed()
}

fn write_long<W: Write>(writer: &mut W, long: i64) -> Result<usize, Error> {
    let mut bytes: [u8; 8] = long.to_be_bytes();
    Ok(writer.write_all(&bytes).map(|_| 8usize)?)
}

fn read_long<R: Read>(reader: &mut R) -> Result<i64, Error> {
    let mut bytes = [0u8; 8];
    reader.read_exact(&mut bytes)?;
    Ok(i64::from_be_bytes(bytes))
}

fn method1<R: Read>(mut reader: R) -> Result<Vec<i64>, Error> {
    let mut buf = BufReader::with_capacity(1048576, reader);
    method2(buf)
}

fn method2<R: Read>(mut reader: R) -> Result<Vec<i64>, Error> {
    (0..1048576i64).map(|_| read_long(&mut reader)).collect()
}

fn write_longs<W: Write>(mut writer: W) -> Result<usize, Error> {
    let mut writer = BufWriter::with_capacity(1048576, writer);
    (0..1048576i64).map(|i| write_long(&mut writer, i)).sum()
}

/*
I have three ideas for methods:
    read/write individually
    read/write in a batch (ideal)
    read/write in a batch, but also use custom reordering
*/

/// A const function that returns the number of bytes that size kibibytes would be.
const fn kibibytes(size: usize) -> usize {
    size << 10
}

/// A const function that returns the number of bytes that size mebibytes would be.
const fn mebibytes(size: usize) -> usize {
    size << 20
}

/// A const function that returns the number of bytes that size gibibytes would be.
const fn gibibytes(size: usize) -> usize {
    size << 30
}

macro_rules! compounder {
    ($($name:ident : $tag:expr),+) => {
        {
            Tag::Compound(Map::from([
                $(
                    (String::from(stringify!($name)), Tag::from($tag)),
                )+
            ]))
        }
    };
}

fn test_tag() -> Tag {
    let byte = Tag::Byte(i8::MAX);
    let short = Tag::Short(i16::MAX);
    let int = Tag::Int(69420);
    let long = Tag::Long(i64::MAX);
    let float = Tag::Float(3.14_f32);
    let double = Tag::Double(3.14159265358979_f64);
    let bytearray = Tag::ByteArray(vec![1,2,3,4]);
    let string = Tag::String(String::from("The quick brown fox jumps over the lazy dog🎈🎄"));
    let list = Tag::List(ListTag::from(vec![byte.clone(),short.clone(),int.clone(),long.clone()]));
    let intarray = Tag::IntArray(vec![1,1,2,3,5,8,13,21,34,55,89,144]);
    let longarray = Tag::LongArray((0..8).map(|i| (0xFu64 << i) as i64 ).collect());
    let compound = Map::from([
        ("Byte".to_owned(), byte.clone()),
        ("Short".to_owned(), short.clone()),
        ("Int".to_owned(), int.clone()),
        ("Long".to_owned(), long.clone()),
        ("Float".to_owned(), float.clone()),
        ("Double".to_owned(), double.clone()),
        ("ByteArray".to_owned(), bytearray.clone()),
        ("String".to_owned(), string.clone()),
        ("List".to_owned(), list.clone()),
        ("Compound".to_owned(), Tag::Compound(Map::from([
            ("One".to_owned(), 1.into()),
            ("Two".to_owned(), 2.into()),
            ("Three".to_owned(), 3.into()),
            ("True".to_owned(), true.into()),
            ("False".to_owned(), false.into()),
            ("Empty List".to_owned(), Tag::List(ListTag::Empty)),
        ]))),
        ("IntArray".to_owned(), intarray.clone()),
        ("LongArray".to_owned(), longarray.clone()),
    ]);
    Tag::Compound(compound)
}

struct Tester(String);

impl<IT: IntoIterator<Item = i8>> From<IT> for Tester {
    fn from(it: IT) -> Self {
        todo!()
    }
}

#[test]
fn compounder_test() {
    let compound = compounder!{
        byte : 1i8,
        string : "The quick brown fox jumps over the lazy dog.",
        int : 1234,
        bytes : vec![1i8, 2, 3, 4, 5]
    };
    println!("{}", compound);
}

fn main() -> Result<(),std::io::Error> {
    use std::fs::File;
    use std::io::*;
    let path = "./ignore/output.nbt";
    let mut file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let tag = test_tag();
    let named = NamedTag::with_name("Test NBT Root.", tag);
    println!("Tag is type {} and is {} bytes in size.", named.tag().id(), named.nbt_size());
    if let Ok(written) = named.nbt_write(&mut writer) {
        println!("Wrote {} bytes to file.", written);
    } else {
        println!("Failed to write to file.")
    }
    drop(writer);
    let mut file = File::open(path)?;
    let mut reader = BufReader::new(file);
    if let Ok((name, tag)) = read_named_tag(&mut reader) {
        println!("Root Name: {}", name);
        println!("Root Value: {}", tag);
    } else {
        println!("Failed for some reason.");
    }
    Ok(())
}
