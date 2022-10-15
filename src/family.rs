use crate::tag::byte;
pub trait NbtFamily {}
pub trait Flag {
    type Not;
}

macro_rules! make_flags {
    ($($name:ident)+) => {
        $(
            pub struct $name;
            impl Flag for $name {}
        )+
    };
}

pub trait Include<T: Flag> {}
pub trait Exclude<T: Flag> {}

pub trait Not<F: Flag> {}

#[allow(non_camel_case_types)]
pub struct nonbyte;

impl Flag for nonbyte {
    type Not = byte;
}

impl Flag for byte {
    type Not = nonbyte;
}

pub trait NonByte {}

impl<N: NonByte> Not<byte> for N {}

pub trait NonBytePrimitive {}
pub trait Primitive {}

impl<T: NonBytePrimitive> Primitive for T {}
impl<T: NonBytePrimitive> NonByte for T {}
