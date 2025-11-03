# tss

[![crates.io](https://img.shields.io/crates/v/tss.svg)](https://crates.io/crates/tss)
[![documentation](https://docs.rs/tss/badge.svg)](https://docs.rs/tss)
[![MIT licensed](https://img.shields.io/crates/l/tss.svg)](../../LICENSE)

Generated node type enums and metadata from tree-sitter languages.

## Convenience crate

This is a convenience wrapper if you are using multiple crates, or just want to switch between
languages easily.

Whereas the individual language crates provide `NodeType`, this crate re-exports each language you
configure in via the features with the language name appended in Pascal case

So the Rust language `tree-sitter-rust` crate `NODE_TYPES` constant shipped as `NodeType` in
`tss-rust` as `tree_sitter_symbols_rust::NodeType` is re-exported in `tss` as
`tree_sitter_symbols::NodeTypeRust`. Distinctly named types can reduce the ability to get confused
about which you're working with when handling tree-sitter parse trees in multiple languages.

## Usage

```toml
[dependencies]
tss = { version = "0.1", features = ["lang-rust"] }
tree-sitter = "0.24"
tree-sitter-rust = "0.24"
```

```rust
use tree_sitter_symbols::NodeTypeRust;
use std::str::FromStr;

let node_type = NodeTypeRust::from_str("function_item")?;
assert_eq!(node_type, NodeTypeRust::FunctionItem);
assert_eq!(node_type.to_string(), "function_item");
```

I might change this to be modules that always re-export the name `NodeType` in future.

## Features

Default includes all languages:

```toml
[features]
# Convenience features
default = ["lang-all"]
lang-all = ["lang-rust"]

# Language features
lang-rust = ["dep:tss-rust"]
# ...
```

You can override that to select particular languages:
```toml
tss = { version = "0.1", default-features = false, features = ["lang-rust"] }
```

For all available metadata features see the repo
[Cargo.toml](https://github.com/lmmx/tree-sitter-symbols/blob/master/crates/tss/Cargo.toml).

## How It Works

At build time, each language reads the corresponding `tree-sitter-*` language crate's `NODE_TYPES` constant and generates:

- A `NodeType` enum with all node types
- `FromStr` for parsing node type strings
- `Display` for converting back to strings
- Documentation linking to language reference (best effort, not comprehensive)

This allows a crate to ship this information as an enum type with zero runtime dependencies. All generation happens at compile time.
This is useful for type safety (rather than checking for strings, as well as hopefully to demystify how tree-sitter languages work.

## Licensing

MIT licensed - see [LICENSE](https://github.com/lmmx/tss/blob/master/LICENSE) for details.
