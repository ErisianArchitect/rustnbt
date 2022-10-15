

pub trait NbtFamily {}
pub trait NbtFamilyFlag {}

pub trait Nbt<Family: NbtFamily> {}

pub struct Allow<Family: NbtFamilyFlag>(Family);
impl<Family: NbtFamilyFlag> NbtFamily for Allow<Family> {}

pub struct Block<Family: NbtFamilyFlag>(Family);
impl<Family: NbtFamilyFlag> NbtFamily for Block<Family> {}

pub struct Primitive;
impl NbtFamilyFlag for Primitive {}

pub struct Byte;
impl NbtFamilyFlag for Byte {}

pub struct Array;
impl NbtFamilyFlag for Array {}