#![doc = "
The format module is for formatting NBT Tags into SNBT, which is a modified
version of JSON.
"]

use std::fmt::Write;

// Requirement: Trait similar to Display that can be applied to NBT types.

struct NbtFormatter {
    buffer: String,
}

impl Write for NbtFormatter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        todo!()
    }
}

trait FormatNbt: Sized {
    fn format_nbt<W: Write>(&self, writer: W);
}