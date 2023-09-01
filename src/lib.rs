//! # Parsing clang [API notes].
//!
//! Effectively, this means files ending with `.apinotes`.
//!
//! TODO.
//!
//! [API notes]: https://clang.llvm.org/docs/APINotes.html

#![warn(elided_lifetimes_in_paths)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/apinotes/0.0.1")]

#[cfg(test)]
mod clang_tests;
mod error;
mod general;
mod map_helper;
mod method_and_property;
mod mid_level;
mod top_level;

pub use self::error::Error;
pub use self::general::*;
pub use self::map_helper::Map;
pub use self::method_and_property::*;
pub use self::mid_level::*;
pub use self::top_level::*;

/// The file extension (without the leading dot) that API notes use.
///
/// Equivalent to `"apinotes"`.
pub const EXTENSION: &str = "apinotes";
