//! # web-tools
//!
//! Tools for web programming in Rust.
//!
//! ## Rationale
//!
//! Rust has a strict type system, which is great. But sometimes it's just convenient to use:
//!
//! ```rust
//! use web_tools::prelude::*;
//!
//! #[cfg(feature = "yew")]
//! fn yew(node: &yew::prelude::NodeRef) {
//!     node.focus();
//! }
//!
//! fn vanilla(element: &web_sys::Element) {
//!     element.focus();
//! }
//! ```
//!
//! ## Functionality
//!
//! * Optimistic traits: execute some function on an element, if it supports it … or do nothing.
//! * Iterators for things that should be iterable, but don't implement `IntoIterator`.

pub mod iter;
pub mod optimistic;
pub mod prelude;
