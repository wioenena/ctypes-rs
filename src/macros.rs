#[macro_export]
macro_rules! def_type {
    ($name:ident, $type:ty) => {
        pub type $name = $type;
    };
}


#[macro_export]
macro_rules! def_constant {
    ($name:ident, $type:ty, $value:expr) => {
        pub const $name: $type = $value;
    };
}