use std::ffi::c_char;

#[allow(non_camel_case_types)]
pub type string = *const c_char;

#[allow(non_camel_case_types)]
pub type array<T> = *const T;
