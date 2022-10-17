pub trait Not<F> {}

pub struct Byte(u8);

impl<N: NonByte> Not<Byte> for N {}

pub trait NonByte {}
pub trait Primitive {}
pub trait NonBytePrimitive {}
impl<T: NonBytePrimitive> NonByte for T {}
impl<T: NonBytePrimitive> Primitive for T {}
