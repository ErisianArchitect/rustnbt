#![allow(unused)]

use rustnbt::{
    tag::*,
    io::*,
};

use std::{
    io::{
        Write,
        Read,
        Error,
        Cursor,
        BufReader,
        BufWriter,
    },
    fs::{
        File,
    },
    time::{
        Duration,
        Instant,
    }, ops::Div
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
    (0..1048576i64).map(|_| {
        read_long(&mut reader)
    })
    .collect()
}

fn write_longs<W: Write>(mut writer: W) -> Result<usize, Error> {
    let mut writer = BufWriter::with_capacity(1048576, writer);
    (0..1048576i64).map(|i| {
        write_long(&mut writer, i)
    })
    .sum()
}

/*
I have three ideas for methods:
    read/write individually
    read/write in a batch (ideal)
    read/write in a batch, but also use custom reordering
*/

fn main() {
    use std::fs::{
        File,
    };
    let avg = (0..10).map(|_| {
        let mut file = File::create("./ignore/longs").expect("Failed to create file.");
        let elapsed = timer(|| {
            write_longs(file).expect("Failed to write longs.");
        });
        println!("Elapsed: {}", elapsed.as_millis());
        elapsed.as_millis()
    })
    .sum::<u128>().div(10);
    println!("Average: {}", avg);
}

