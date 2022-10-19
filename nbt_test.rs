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

fn test_tag() -> Tag {
    let byte = Tag::Byte(i8::MAX);
    let short = Tag::Short(i16::MAX);
    let int = Tag::Int(69420);
    let long = Tag::Long(i64::MAX);
    let float = Tag::Float(3.14_f32);
    let double = Tag::Double(3.14159265358979_f64);
    let bytearray = Tag::ByteArray(vec![1,2,3,4]);
    let list = Tag::List(ListTag::from(vec![1,2,3,4]));
    let intarray = Tag::IntArray(vec![1,1,2,3,5,8,13,21,34,55,89,144]);
    let longarray = Tag::LongArray(vec![1,3,3,7, 1337, 13,37, 1,3,37,1,337, 133,7, 1,33,7,13,3,7]);
    let mut compound = Map::from([
        ("Byte".to_owned(), byte.clone()),
        ("Short".to_owned(), short.clone()),
        ("Int".to_owned(), int.clone()),
        ("Long".to_owned(), long.clone()),
        ("Float".to_owned(), float.clone()),
        ("Double".to_owned(), double.clone()),
        ("ByteArray".to_owned(), bytearray.clone()),
        ("List".to_owned(), list.clone()),
        ("Empty List".to_owned(), Tag::List(ListTag::Empty)),
        ("IntArray".to_owned(), intarray.clone()),
        ("LongArray".to_owned(), longarray.clone()),
    ]);
    let mapclone = compound.clone();
    compound.insert("Compound".to_owned(), Tag::Compound(mapclone));
    Tag::Compound(compound)
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
