use std::marker::PhantomData;

pub trait NbtFamily {}
pub trait Flag {}

macro_rules! make_families {
    ($($name:ident$(<$($tag:ident$(: $trait_:ident)?),+>)?,)+) => {
        $(
            pub  struct $name$(<$($tag $(: $trait_)? ,)+>($(PhantomData<$tag>),+))?;
            impl$(<$($tag$(: $trait_)?,)+>)? NbtFamily for $name$(<$($tag,)+>)? {}
        )+
    };
}

macro_rules! make_flags {
    ($($name:ident),+) => {
        $(
            pub struct $name;
            impl Flag for $name {}
        )+
    };
}

make_families![
    Allow<F: Flag>,
    Block<F: Flag>,
];

make_flags![
    Primitive,
    Byte,
    Array,
    Special
];