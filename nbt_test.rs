#![allow(unused)]

use rustnbt::{io::*, tag::*};

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

fn main() {
    use std::fs::File;

    println!("1KiB: {}", kibibytes(1024 * 1024));
    println!("1MiB: {}", mebibytes(1024));
    println!("1GiB: {}", gibibytes(1));
}
