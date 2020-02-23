//! FFI Bindings for Berkeley DB.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

#[cfg(feature = "v5_3")]
include!(concat!(env!("OUT_DIR"), "/ffi_5_3.rs"));

#[cfg(feature = "v4_8")]
include!(concat!(env!("OUT_DIR"), "/ffi_4_8.rs"));
