/// Shorthand way to create a Tag::Compound.
/// Example:
/// ```no_run
/// compound!{
///     ("Item One", 0i8),
///     (String::from("Item Two"), 2i32),
///     ("Item Three", Tag::Byte(1))
/// }
/// ```
#[macro_export]
macro_rules! compound {
    ($(($name:expr, $value:expr)),+) => {
        $crate::tag::Tag::Compound($crate::Map::from([
            $(
                (String::from($name), $crate::tag::Tag::from($value)),
            )+
        ]))
    };
}

#[macro_export]
macro_rules! list {
    ($($item:expr),+) => {
        $crate::tag::Tag::List($crate::tag::ListTag::from(vec![
            $(
                $item,
            )+
        ]))
    };
    ($value:expr; $repititions:expr) => {
        $crate::tag::Tag::List($crate::tag::ListTag::from(vec![$value; $repititions]))
    };
    () => {
        $crate::tag::Tag::List($crate::tag::ListTag::Empty);
    };
}

pub use list;
pub use compound;