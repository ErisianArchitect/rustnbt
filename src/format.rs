#![doc = "
The format module is for formatting NBT Tags into SNBT, which is a modified
version of JSON.
"]

use std::fmt::{Write, Display, Debug, Pointer};

use chumsky::chain::Chain;

/// Measures the size of the resulting string if this were converted to a string.
const fn num_width(n: i64) -> usize {
    const MAX_ACCUM: i64 = 1000000000000000000;
    match n {
        i64::MIN => 20,
        i64::MAX => 19,
        _ => {
            let mut size = if n < 0 { 2 } else { 1 };
            let n = n.abs();
            let mut accum = 10;
            // max: 1000000000000000000
            while accum <= n && accum < MAX_ACCUM {
                size += 1;
                accum *= 10;
            }
            size
        }
    }
}

fn write_indent<W: Write>(buffer: &mut W, indent: usize, indent_string: &str) {
    (0..indent)
        .for_each(|_| { 
            write!(buffer, "{indent_string}");
        });
}

fn escape_string<S: AsRef<str>, W: Write>(writer: &mut W, unescaped: S) -> std::fmt::Result {
    // Macros make the whole world better!
    macro_rules! match_char {
        ($buffer:expr, $input:expr; $($tok:tt => $escaped:expr),+) => {
            match $input {
                $(
                    $tok => write!($buffer, "{}", $escaped),
                )+
            }
        };
    }
    unescaped.as_ref().chars().try_for_each(|ch| {
        match_char!{writer, ch;
            // TODO: Find out what escape sequences are supported by Minecraft.
            '\\' => "\\\\",
            '/' => "\\/",
            '"' => "\\\"",
            '\'' => "\\'",
            '\x08' => "\\b",
            '\x0C' => "\\f",
            '\n' => "\\n",
            '\r' => "\\r",
            '\t' => "\\t",
            '\0' => "\\0",
            other => other
        }
    })
}

/// Space count constrained to powers of two with an upper-bound of 32 and a lower bound of 1.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
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

impl Default for SpaceCount {
    /// Returns [SpaceCount::Four].
    fn default() -> Self {
        Self::Four
    }
}

impl From<SpaceCount> for usize {
    fn from(count: SpaceCount) -> Self {
        count as usize
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Indent {
    /// Only a single tab.
    Tabs,
    Spaces(SpaceCount),
}

impl Default for Indent {
    /// Returns [Indent::Spaces]\([SpaceCount::Four]\)
    fn default() -> Self {
        Self::four_spaces()
    }
}

impl Indent {

    /// Returns the length of the indent string.
    pub fn len(&self) -> usize {
        match self {
            Indent::Tabs => 1,
            Indent::Spaces(count) => *count as usize,
        }
    }

    pub const fn space() -> Self {
        Self::Spaces(SpaceCount::One)
    }

    pub const fn two_spaces() -> Self {
        Self::Spaces(SpaceCount::Two)
    }

    pub const fn four_spaces() -> Self {
        Self::Spaces(SpaceCount::Four)
    }

    pub const fn eight_spaces() -> Self {
        Self::Spaces(SpaceCount::Eight)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Indentation {
    indent: Indent,
    // u32 just in case someone is really determined to write out some deeply nested structures for whatever reason.
    level: u32,
}

impl Indentation {

    /// Returns the length of the indentation string.
    pub fn len(&self) -> usize {
        self.indent.len() * self.level as usize
    }

    pub const fn new(indent: Indent) -> Self {
        Self {
            indent,
            level: 0,
        }
    }

    pub const fn level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub const fn space() -> Self {
        Self::new(Indent::space())
    }

    pub const fn two_spaces() -> Self {
        Self::new(Indent::two_spaces())
    }

    pub const fn four_spaces() -> Self {
        Self::new(Indent::four_spaces())
    }

    pub const fn eight_spaces() -> Self {
        Self::new(Indent::eight_spaces())
    }

    pub const fn spaces(count: SpaceCount) -> Self {
        Self::new(Indent::Spaces(count))
    }

    pub const fn tabs() -> Self {
        Self::new(Indent::Tabs)
    }

    pub fn indent(self) -> Self {
        Self {
            indent: self.indent,
            level: self.level + 1,
        }
    }
}

impl Display for Indentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.level)
            .try_for_each(|_| {
                write!(f, "{}", self.indent)
            })
    }
}

trait NbtDisplay {
    fn fmt_nbt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn wrap(self) -> DisplayWrapper<Self> where Self : Sized {
        DisplayWrapper(self)
    }

    fn wrap_borrow(&self) -> DisplayWrapper<&Self> {
        DisplayWrapper(self)
    }
}

/// Wraps a type that might implement [Display] allowing for fine tuning of displaying of the value.
struct DisplayWrapper<T>(T) ;

macro_rules! display_wrappers {
    ($($type:ty => {}$($suffix:ident)?;)+) => {
        $(
            impl NbtDisplay for $type {
                fn fmt_nbt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, concat!("{}"$(, stringify!($suffix))?), self)
                }
            }
        )+
    };
}

display_wrappers!{
    i8 => {}B;
    i16 => {}S;
    i32 => {};
    i64 => {}L;
    f32 => {}F;
    f64 => {}D;
}

impl NbtDisplay for String {
    fn fmt_nbt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        escape_string(f, self)?;
        write!(f, "\"")
    }
}

impl NbtDisplay for IndentedValue<&Vec<i8>> {
    fn fmt_nbt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[B;")?;
        // TODO
        if self.len() <= 16 {
        }
        write!(f, "]");
        Ok(())
    }
}

// fmt_nbt takes a reference to self, so if we have something like &&&&&&&T where T implements NbtDisplay
// we can dereference to &T so that it's just NbtDisplay::fmt_display(&T)
// That allows you to have a reference to a reference to a reference that implements NbtDisplay and you'll still be able to use it.
impl<T: NbtDisplay> NbtDisplay for &T {
    fn fmt_nbt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        T::fmt_nbt(self, f)
    }
}

// Wrapper type for creating display functions for SNBT.
struct IndentedValue<T> {
    value: T,
    indentation: Indentation,
}

impl<T> IndentedValue<T> {

    pub fn new(value: T) -> Self {
        Self::indented(value, Indentation::spaces(SpaceCount::Four))
    }

    pub fn indented(value: T, indentation: Indentation) -> Self {
        Self {
            value,
            indentation,
        }
    }

    pub fn indent<NT>(&self, value: NT) -> IndentedValue<NT> {
        IndentedValue::indented(value, self.indentation.indent())
    }

    pub(crate) fn write_indent<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "{}", self.indentation)
    }

}

impl<T: Display> Display for IndentedValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.indentation, self.value)
    }
}

impl<T: Display> IndentedValue<T> {
    pub fn write_indented_value<W: std::fmt::Write>(&self, writer: &mut W) {
        self.write_indent(writer);
        write!(writer, "{}{}", self.indentation, self.value);
    }
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

impl Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Indent::Tabs => write!(f, "\t"),
            Indent::Spaces(spaces) => write!(f, "{spaces}"),
        }
    }
}

impl<T: NbtDisplay> Display for DisplayWrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt_nbt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_width_test() {
        assert!(num_width(0) == 1);
        assert!(num_width(1) == 1);
        assert!(num_width(10) == 2);
        assert!(num_width(-1) == 2);
        assert!(num_width(-10) == 3);
        assert!(num_width(1234) == 4);
        assert!(num_width(12345) == 5);
        assert!(num_width(123456) == 6);
        assert!(num_width(-1234) == 5);
        assert!(num_width(-12345) == 6);
        assert!(num_width(-123456) == 7);
    }

}