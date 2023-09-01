//! # Parsing clang [API notes].
//!
//! Effectively, this means files ending with `.apinotes`.
//!
//! TODO.
//!
//! [API notes]: https://clang.llvm.org/docs/APINotes.html

#![no_std]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/apinotes/0.0.1")]
