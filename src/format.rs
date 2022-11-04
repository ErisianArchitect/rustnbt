#![doc = "
The format module is for formatting NBT Tags into SNBT, which is a modified
version of JSON.
"]

use std::fmt::{Write, Display, Debug};

/// Space count constrained to powers of two with an upper-bound of 32 and a lower bound of 1.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(usize)]
pub enum SpaceCount {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    /// Do you really need 16 spaces?
    Sixteen = 16,
    /// Come on! 32? You do not need this many spaces! But fine.
    /// Have it your way. Here are your 32 spaces!
    ThirtyTwo = 32,
}

impl Display for SpaceCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SpaceCount::*;
        match self {
            One => write!(f, " "),
            Two => write!(f, "  "),
            Four => write!(f, "    "),
            Eight => write!(f, "        "),
            Sixteen => write!(f, "                "),
            ThirtyTwo => write!(f, "                                "),
        }
    }
}

impl From<SpaceCount> for usize {
    fn from(count: SpaceCount) -> Self {
        count as usize
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Indent {
    Tabs,
    Spaces(SpaceCount),
}

impl Indent {

    /// This is a common tab-width, so I figured I would add it as a shortcut.
    pub fn four_spaces() -> Self {
        Self::Spaces(SpaceCount::Four)
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Indent::Tabs => write!(f, "\t"),
            Indent::Spaces(spaces) => write!(f, "{spaces}"),
        }
    }
}

// Wrapper type for creating display functions for SNBT.
struct SnbtWrapper<T> {
    value: T,
    indent_level: usize,
    indent: Indent,
}

impl<T> SnbtWrapper<T> {

    pub fn new(value: T) -> Self {
        Self::indented(value, 0, Indent::Spaces(SpaceCount::Four))
    }

    pub fn indented(value: T, indent_level: usize, indent: Indent) -> Self {
        Self {
            value,
            indent_level,
            indent,
        }
    }

    pub fn indent<NT>(&self, value: NT) -> SnbtWrapper<NT> {
        SnbtWrapper::indented(value, self.indent_level + 1, self.indent)
    }

    fn write_indent<W: std::fmt::Write>(&self, writer: &mut W) {
        (0..self.indent_level)
            .for_each(|_| {
                write!(writer, "{}", self.indent);
            });
    }

}

impl<T: Display> SnbtWrapper<T> {
    pub fn write_indented_value<W: std::fmt::Write>(&self, writer: &mut W) {
        self.write_indent(writer);
        write!(writer, "{}", self.value);
    }
}

impl std::fmt::Debug for SnbtWrapper<i8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}B", self.value)
    }
}

impl Display for SnbtWrapper<i8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}B", self)
    }
}

impl Display for SnbtWrapper<Vec<i8>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn write_indent<W: Write>(buffer: &mut W, indent: usize, indent_string: &str) {
    (0..indent)
        .for_each(|_| { 
            write!(buffer, "{indent_string}");
        });
}

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