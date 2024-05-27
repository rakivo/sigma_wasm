#[repr(C)]
pub struct Color {
    pub r: std::ffi::c_uchar,
    pub g: std::ffi::c_uchar,
    pub b: std::ffi::c_uchar,
    pub a: std::ffi::c_uchar,
}
pub mod colors;
pub use colors::*;
pub mod enums;
pub use enums::*;
