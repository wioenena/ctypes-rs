mod macros;
#[allow(non_camel_case_types)]
#[cfg(feature = "win_types")]
pub mod win_types;


#[allow(non_camel_case_types)]
pub mod core {
    use crate::def_type;

    def_type!(char_t, i8);
    def_type!(u_char_t, u8);
    def_type!(short_t, i16);
    def_type!(u_short_t, u16);
    #[cfg(target_pointer_width = "16")]
    def_type!(int_t, i16);
    #[cfg(target_pointer_width = "16")]
    def_type!(u_int_t, u16);
    #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
    def_type!(int_t, i32);
    #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
    def_type!(u_int_t, u32);
    #[cfg(target_pointer_width = "32")]
    def_type!(long_t, i32);
    #[cfg(target_pointer_width = "32")]
    def_type!(u_long_t, u32);
    #[cfg(target_pointer_width = "64")]
    def_type!(long_t, i64);
    #[cfg(target_pointer_width = "64")]
    def_type!(u_long_t, u64);
    def_type!(float_t, f32);
    def_type!(double_t, f64);
    def_type!(int_max_t, self::int_t);
    def_type!(size_t, self::u_long_t);
    def_type!(ssize_t, self::long_t);
    def_type!(ptrdiff_t, self::long_t);
    def_type!(wchar16_t, u16);
    def_type!(wchar32_t, u32);
    pub enum void {}

    pub mod ptr {
        use crate::def_type;

        def_type!(void_ptr_t, *const super::void);
        def_type!(void_ptr_t_mut, *mut super::void);
        def_type!(nullptr_t, *const super::void);
        def_type!(nullptr_mut_t, *mut super::void);
    }

    #[allow(non_upper_case_globals)]
    pub mod constants {
        use crate::def_constant;

        def_constant!(CHAR_MIN, i8, i8::MIN);
        def_constant!(CHAR_MAX, i8, i8::MAX);
        def_constant!(UCHAR_MIN, u8, u8::MIN);
        def_constant!(UCHAR_MAX, u8, u8::MAX);
        def_constant!(INT_MIN, super::int_t, super::int_t::MIN);
        def_constant!(INT_MAX, super::int_t, super::int_t::MAX);
        def_constant!(UINT_MIN, super::u_int_t, super::u_int_t::MIN);
        def_constant!(UINT_MAX, super::u_int_t, super::u_int_t::MAX);
        def_constant!(LONG_MIN, super::long_t, super::long_t::MIN);
        def_constant!(LONG_MAX, super::long_t, super::long_t::MAX);
        def_constant!(ULONG_MIN, super::u_long_t, super::u_long_t::MIN);
        def_constant!(ULONG_MAX, super::u_long_t, super::u_long_t::MAX);
        def_constant!(SHORT_MIN, super::short_t, super::short_t::MIN);
        def_constant!(SHORT_MAX, super::short_t, super::short_t::MAX);
        def_constant!(USHORT_MIN, super::u_short_t, super::u_short_t::MIN);
        def_constant!(USHORT_MAX, super::u_short_t, super::u_short_t::MAX);
        def_constant!(FLOAT_MIN, super::float_t, super::float_t::MIN);
        def_constant!(FLOAT_MAX, super::float_t, super::float_t::MAX);
        def_constant!(DOUBLE_MIN, super::double_t, super::double_t::MIN);
        def_constant!(DOUBLE_MAX, super::double_t, super::double_t::MAX);
        def_constant!(TRUE, u8, 1);
        def_constant!(FALSE, u8, 0);
        def_constant!(NULL, u8, 0);
        def_constant!(nullptr, super::ptr::nullptr_t, 0 as super::ptr::nullptr_t);
    }
}

