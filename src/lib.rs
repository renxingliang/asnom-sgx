#![allow(bare_trait_objects)]
#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#![cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

#[macro_use]
extern crate nom;
extern crate byteorder;

pub mod parse;
pub mod write;

pub mod common;
pub mod universal;
pub mod structures;
pub mod structure;

pub use nom::IResult;
pub use nom::IResult::*;

pub use nom::Consumer;
pub use nom::ConsumerState;
pub use nom::Input;
pub use nom::Move;
pub use parse::Parser;
