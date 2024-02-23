#![allow(non_camel_case_types, non_snake_case)]

#[macro_use]
extern crate serde;

mod generated;
mod identity;

pub use generated::*;
pub use identity::*;
