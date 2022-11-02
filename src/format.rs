#![doc = "
The format module is for formatting NBT Tags into SNBT, which is a modified
version of JSON.
"]

// Requirement: Trait similar to Display that can be applied to NBT types.

trait FormatNbt<T> {
    fn format_nbt(input: &T) -> String;
}