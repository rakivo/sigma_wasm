mod macros;
#[allow(unused)]
pub use macros::*;
mod colors;
#[allow(unused)]
pub use colors::*;
mod enums;
#[allow(unused)]
pub use enums::*;
#[allow(unused, dead_code, non_camel_case_types, non_snake_case)]
mod structs;
#[allow(unused)]
pub use structs::*;
#[allow(unused, dead_code, non_camel_case_types, non_snake_case)]
#[cfg(not(feature = "web"))]
mod fns;
#[allow(unused)]
#[cfg(not(feature = "web"))]
pub use fns::*;
#[cfg(feature = "web")]
#[allow(non_snake_case)]
mod web_fns;
#[allow(unused)]
#[cfg(feature = "web")]
pub use web_fns::*;
