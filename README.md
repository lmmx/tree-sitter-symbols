# tree-sitter-symbols

[![crates.io](https://img.shields.io/crates/v/tss.svg)](https://crates.io/crates/tss)
[![documentation](https://docs.rs/tss/badge.svg)](https://docs.rs/tss)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/tss.svg)](./LICENSE)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/tss/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/tss/master)

Tree-sitter symbol node enums and metadata, generated at build time.

## Overview

The `tss-*` crates in this repo generate enums and metadata for tree-sitter language grammars. This means that downstream usage can replace string literal comparisons like `node.kind() == "function_item"` with type-safe enums, each tree-sitter node type being generated as an enum variant.

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

Default configuration includes all metadata:
```toml
tss-rust = "0.1"
```

Select specific metadata categories:
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

## Crates

So far only Rust has been covered. Additional languages TBC (Python is next in line)

### tss-rust

**tss-rust** provides Rust language support.

At build time, `tss-rust` reads `tree-sitter-rust`'s `NODE_TYPES` JSON
([constant](https://docs.rs/tree-sitter-rust/latest/tree_sitter_rust/constant.NODE_TYPES.html)) and generates:

- A `NodeType` enum with all 280 node types (as of `tree-sitter-rust` v0.24.0)
- `FromStr` implementation for parsing node type strings
- `Display` implementation for converting back to strings
- Documentation with links to Rust language reference

This means it can have zero runtime dependencies. All code generation happens at compile time.

See [the schema](https://github.com/lmmx/isotarp/blob/master/crates/tss-rust/codegen/schema.rs)
for what specifically it extracts from the `NODE_TYPES`. This schema was generated using
[genson-cli](https://docs.rs/genson-cli) as JSON schema then Rust (serde) structs generated
through the [app.quicktype.io](https://app.quicktype.io/?l=rust) web app.

## Licensing

MIT licensed - see [LICENSE](https://github.com/lmmx/tree-sitter-symbols/blob/master/LICENSE) for details.
