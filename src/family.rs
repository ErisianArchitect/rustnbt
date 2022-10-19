pub trait NonByte {}
pub trait Primitive {}
pub trait NonBytePrimitive {}
impl<T: NonBytePrimitive> NonByte for T {}
impl<T: NonBytePrimitive> Primitive for T {}
