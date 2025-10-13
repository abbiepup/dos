//! # The Rust Standard Library for DOS.

#![no_std]
#![feature(ascii_char)]
#![feature(assert_matches)]
#![feature(bstr)]
#![feature(cfg_select)]
#![feature(const_format_args)]
#![feature(decl_macro)]
#![feature(doc_cfg)]
#![feature(format_args_nl)]
#![feature(log_syntax)]
#![feature(prelude_import)]
#![feature(random)]
#![feature(rustc_attrs)]
#![feature(trace_macros)]

extern crate alloc as alloc_crate;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// Platform-abstraction modules.
mod sys;

// The Rust prelude.
pub mod prelude;

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[allow(unused)]
#[prelude_import]
use prelude::rust_2024::*;

pub mod alloc;
pub mod ascii;
pub mod bstr;
pub mod cp437;
pub mod fs;
pub mod io;
pub mod path;
pub mod process;
pub mod random;
pub mod time;

pub use core::{
    assert, assert_matches, cfg, column, compile_error, concat, const_format_args, env, file, format_args, format_args_nl, include, include_bytes,
    include_str, line, log_syntax, module_path, option_env, stringify, trace_macros,
};
