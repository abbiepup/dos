//! Memory allocation APIs.
//!
//! In a given program, the standard library has one “global” memory allocator
//! that is used for example by `Box<T>` and `Vec<T>`.

#[doc(inline)]
pub use alloc_crate::alloc::*;

/// The default memory allocator provided by the operating system.
#[derive(Debug, Default, Clone, Copy)]
pub struct System;
