

// To help utilize the table in /src/table.rs
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