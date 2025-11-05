//! Generated node type enums and metadata from tree-sitter-rust.
//!
//! `tss-rust` provides type-safe Rust enums for every node type in the tree-sitter-rust grammar.
//! This eliminates error-prone string comparisons like `node.kind() == "function_item"` in favor
//! of `NodeType::FunctionItem`, enabling compile-time verification and better IDE support.
//!
//! # Quick Start
//!
//! ```toml
//! [dependencies]
//! tss-rust = "0.2"
//! tree-sitter = "0.24"
//! tree-sitter-rust = "0.24"
//! ```
//!
//! ```
//! use tree_sitter_symbols_rust::NodeType;
//! use std::str::FromStr;
//!
//! let node_type = NodeType::from_str("function_item")?;
//! assert_eq!(node_type, NodeType::FunctionItem);
//! assert_eq!(node_type.to_string(), "function_item");
//! # Ok::<(), String>(())
//! ```
//!
//! # Features
//!
//! The crate uses Cargo features to control which node types and metadata are available:
//!
//! ## Node Type Features
//!
//! Each tree-sitter node type corresponds to a feature. Enable specific nodes:
//!
//! ```toml
//! tss-rust = { version = "0.2", default-features = false, features = ["function_item", "struct_item"] }
//! ```
//!
//! Or enable all node types at once:
//!
//! ```toml
//! tss-rust = { version = "0.2", features = ["node_full"] }
//! ```
//!
//! The `node` feature is a tracking feature automatically enabled when any node type is active.
//!
//! ## Metadata Features
//!
//! Control which metadata from tree-sitter's `NODE_TYPES` JSON is included:
//!
//! - `meta_named` - whether nodes are named in the grammar
//! - `meta_subtypes` - possible subtypes for each node
//! - `meta_fields` - named fields nodes can have
//! - `meta_children` - anonymous children nodes can have
//! - `meta_extra` - extra node markers
//! - `meta_root` - root node markers
//!
//! Enable all metadata:
//!
//! ```toml
//! tss-rust = { version = "0.2", features = ["meta_full"] }
//! ```
//!
//! Or select specific metadata categories:
//!
//! ```toml
//! tss-rust = { version = "0.2", default-features = false, features = ["meta_named", "meta_fields"] }
//! ```
//!
//! The `meta` feature is a tracking feature automatically enabled when any metadata feature is active.
//!
//! ## Complete Feature Matrix
//!
//! ```toml
//! # Everything: all nodes + all metadata
//! tss-rust = { version = "0.2", features = ["full"] }
//!
//! # Just the nodes you need with selected metadata
//! tss-rust = { version = "0.2", default-features = false, features = [
//!     "function_item",
//!     "struct_item",
//!     "meta_named"
//! ]}
//! ```
//!
//! # How It Works
//!
//! At build time, `tss-rust` reads the `NODE_TYPES` constant from the `tree-sitter-rust` crate
//! and generates:
//!
//! - A `NodeType` enum with variants for all 280+ node types
//! - `FromStr` implementation for parsing node type strings
//! - `Display` implementation for converting back to strings
//! - Documentation linking to Rust language reference where applicable
//! - Feature-gated compilation so you only pay for what you use
//!
//! All generation happens at compile time with zero runtime dependencies. The generated code
//! is feature-gated, so when compiled with `--no-default-features`, the crate adds virtually
//! no overhead to your binary.
//!
//! # Design Philosophy
//!
//! **Type Safety**: String comparisons are error-prone and not checked at compile time. Using
//! enum variants catches typos early and enables exhaustive pattern matching.
//!
//! **Minimal Overhead**: With careful feature selection, you include only the node types you
//! actually use. The default features enable only metadata, keeping the baseline small.
//!
//! **Build-Time Generation**: By generating at build time rather than using macros, we get
//! better IDE support, faster compile times, and the ability to inspect generated code.

#![allow(clippy::negative_feature_names)]
#![allow(clippy::redundant_feature_names)]

// The generated code is included from the build script output
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(any(feature = "function_item", feature = "node_full"))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[cfg(any(feature = "function_item", feature = "node_full"))]
    #[test]
    fn test_from_str() {
        assert_eq!(
            NodeType::from_str("function_item").unwrap(),
            NodeType::FunctionItem
        );
    }

    #[cfg(any(feature = "function_item", feature = "node_full"))]
    #[test]
    fn test_display() {
        assert_eq!(NodeType::FunctionItem.to_string(), "function_item");
    }
}
