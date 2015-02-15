#![crate_name = "strophe"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(libc)]
extern crate libc;

pub use strophe::*;
mod strophe;
