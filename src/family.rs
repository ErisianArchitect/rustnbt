pub trait NbtFamily {}
pub trait Flag {}

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

make_flags![
    Primitive
    Byte
    Array
    Special
];