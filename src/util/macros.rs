

// To help utilize the table in /src/table.rs
/// Choose code based on whether or not $type_ exists.
/// This is to filter the End tag.
/// Usage: table_arm_filter!($($type_)? : { true block } else { false block })
#[macro_export]
macro_rules! table_arm_filter {
    ($type_:ty : {$($present:tt)*} else {$($absent:tt)*}) => {
        $($present)*
    };
    ( : {$($present:tt)*} else {$($absent:tt)*}) => {
        $($absent)*
    };
}

pub(crate) use table_arm_filter;