# yew-tools

[![crates.io](https://img.shields.io/crates/v/yew-tools.svg)](https://crates.io/crates/web-tools)
[![docs.rs](https://docs.rs/yew-tools/badge.svg)](https://docs.rs/web-tools)

> Tools for the Web

## Usage

Add it to your project:

```shell
cargo add web-tools
```

## Rationale

Rust has a strict type system, which is great. But sometimes it's just convenient to use:

```rust
use web_tools::prelude::*;

fn callback(node: &NodeRef) {
    node.focus();
}
```

## Functionality

* Optimistic traits: execute some function on an element, if it supports it … or do nothing.
* Iterators for things that should be iterable, but don't implement `IntpIterator`.