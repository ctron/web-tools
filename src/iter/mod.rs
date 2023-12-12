//! Help iterating of things
//!
//! Some types in the Web APIs are kind of iterable, but don't support Rust's iterator concept.
//! These helpers can be wrapped around those types, and implement [`IntoIterator`].

mod html_collection;
mod node_list;

pub use html_collection::*;
pub use node_list::*;
