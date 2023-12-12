//! Optimistic operations
//!
//! In may cases, Web APIs in Rust return [`Result`] or [`Option`] and enforce strict types. Then
//! again in JavaScript-land one can simply call `.focus()` on something and it works, breaks, or
//! does nothing. But the code looks a bit simpler.
//!
//! Optimistic traits bring part of this to Rust. Functions on [`yew::prelude::NodeRef`] just work,
//! or fail silently.

mod element;
mod html_element;
mod html_form_element;
mod html_input_element;

pub use element::*;
pub use html_element::*;
pub use html_form_element::*;
pub use html_input_element::*;
