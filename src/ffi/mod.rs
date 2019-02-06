#![allow(non_camel_case_types)]

pub use self::brotli::*;

mod brotli {
include!("_brotli.rs");
}
