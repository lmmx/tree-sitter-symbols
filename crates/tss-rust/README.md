# tss-rust

[![crates.io](https://img.shields.io/crates/v/tss-rust.svg)](https://crates.io/crates/tss-rust)
[![documentation](https://docs.rs/tss-rust/badge.svg)](https://docs.rs/tss-rust)
[![MIT licensed](https://img.shields.io/crates/l/tss-rust.svg)](../../LICENSE)

Generated node type enums and metadata from tree-sitter-rust.

## Usage

```toml
[dependencies]
tss-rust = "0.1"
tree-sitter = "0.24"
tree-sitter-rust = "0.24"
```

```rust
use tree_sitter_symbols_rust::NodeType;
use std::str::FromStr;

let node_type = NodeType::from_str("function_item")?;
assert_eq!(node_type, NodeType::FunctionItem);
assert_eq!(node_type.to_string(), "function_item");
```

## Features

Default includes all metadata:
```toml
tss-rust = "0.1"
```

Select specific metadata:
```toml
tss-rust = { version = "0.1", default-features = false, features = ["meta-named", "meta-fields"] }
```

Available metadata features:
- `meta-named` - whether nodes are named in the grammar
- `meta-subtypes` - possible subtypes for each node
- `meta-fields` - named fields nodes can have
- `meta-children` - anonymous children nodes can have
- `meta-extra` - extra node markers
- `meta-root` - root node markers

## How It Works

At build time, each language reads the corresponding `tree-sitter-*` language crate's `NODE_TYPES` constant and generates:

- A `NodeType` enum with all 280 node types
- `FromStr` for parsing node type strings
- `Display` for converting back to strings
- Documentation linking to language reference (best effort, not comprehensive)

This allows a crate to ship this information as an enum type with zero runtime dependencies. All generation happens at compile time.
This is useful for type safety (rather than checking for strings, as well as hopefully to demystify how tree-sitter languages work.


## Licensing

MIT licensed - see [LICENSE](https://github.com/lmmx/tss/blob/master/LICENSE) for details.
