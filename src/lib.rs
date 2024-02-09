macro_rules! define_c_type {
    ($name:ident, $type:ty) => {
        pub type $name = $type;
    };
}

macro_rules! define_constant {
    ($name:ident, $type:ty, $value:expr) => {
        pub const $name: $type = $value;
    };
}

pub mod core {
    #[allow(non_camel_case_types)]
    pub mod types {
        define_c_type!(char_t, i8);
        define_c_type!(u_char_t, u8);
        define_c_type!(short_t, i16);
        define_c_type!(u_short_t, u16);
        #[cfg(target_pointer_width = "16")]
        define_c_type!(int_t, i16);
        #[cfg(target_pointer_width = "16")]
        define_c_type!(u_int_t, u16);
        #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
        define_c_type!(int_t, i32);
        #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
        define_c_type!(u_int_t, u32);
        #[cfg(target_pointer_width = "32")]
        define_c_type!(long_t, i32);
        #[cfg(target_pointer_width = "32")]
        define_c_type!(u_long_t, u32);
        #[cfg(target_pointer_width = "64")]
        define_c_type!(long_t, i64);
        #[cfg(target_pointer_width = "64")]
        define_c_type!(u_long_t, u64);
        define_c_type!(float_t, f32);
        define_c_type!(double_t, f64);
        define_c_type!(int_max_t, self::int_t);
        define_c_type!(size_t, self::u_long_t);
        define_c_type!(ssize_t, self::long_t);
        define_c_type!(ptrdiff_t, self::long_t);
        pub enum void {}

        pub mod ptr {
            define_c_type!(void_ptr_t, *const super::void);
            define_c_type!(void_ptr_t_mut, *mut super::void);
            define_c_type!(nullptr_t, *const super::void);
            define_c_type!(nullptr_mut_t, *mut super::void);
        }
    }

    #[allow(non_upper_case_globals)]
    pub mod constants {
        define_constant!(CHAR_MIN, i8, i8::MIN);
        define_constant!(CHAR_MAX, i8, i8::MAX);
        define_constant!(UCHAR_MIN, u8, u8::MIN);
        define_constant!(UCHAR_MAX, u8, u8::MAX);
        define_constant!(INT_MIN, super::types::int_t, super::types::int_t::MIN);
        define_constant!(INT_MAX, super::types::int_t, super::types::int_t::MAX);
        define_constant!(UINT_MIN, super::types::u_int_t, super::types::u_int_t::MIN);
        define_constant!(UINT_MAX, super::types::u_int_t, super::types::u_int_t::MAX);
        define_constant!(LONG_MIN, super::types::long_t, super::types::long_t::MIN);
        define_constant!(LONG_MAX, super::types::long_t, super::types::long_t::MAX);
        define_constant!(
            ULONG_MIN,
            super::types::u_long_t,
            super::types::u_long_t::MIN
        );
        define_constant!(
            ULONG_MAX,
            super::types::u_long_t,
            super::types::u_long_t::MAX
        );
        define_constant!(SHORT_MIN, super::types::short_t, super::types::short_t::MIN);
        define_constant!(SHORT_MAX, super::types::short_t, super::types::short_t::MAX);
        define_constant!(
            USHORT_MIN,
            super::types::u_short_t,
            super::types::u_short_t::MIN
        );
        define_constant!(
            USHORT_MAX,
            super::types::u_short_t,
            super::types::u_short_t::MAX
        );
        define_constant!(FLOAT_MIN, super::types::float_t, super::types::float_t::MIN);
        define_constant!(FLOAT_MAX, super::types::float_t, super::types::float_t::MAX);
        define_constant!(
            DOUBLE_MIN,
            super::types::double_t,
            super::types::double_t::MIN
        );
        define_constant!(
            DOUBLE_MAX,
            super::types::double_t,
            super::types::double_t::MAX
        );
        define_constant!(TRUE, u8, 1);
        define_constant!(FALSE, u8, 0);
        define_constant!(NULL, u8, 0);
        define_constant!(
            nullptr,
            super::types::ptr::nullptr_t,
            0 as super::types::ptr::nullptr_t
        );
    }
}

pub mod win_types;