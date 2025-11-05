//! Convenience wrapper for tree-sitter symbol node type enums across multiple languages.
//!
//! `tss` provides a unified interface for working with tree-sitter node types from multiple
//! language grammars. Rather than importing language-specific crates directly, you can enable
//! languages through features and access them through this single crate.
//!
//! # Quick Start
//!
//! ```toml
//! [dependencies]
//! tss = { version = "0.2", features = ["lang-rust-full"] }
//! tree-sitter = "0.24"
//! tree-sitter-rust = "0.24"
//! ```
//!
//! ```
//! # #[cfg(feature = "lang-rust-full")]
//! # {
//! use tree_sitter_symbols::NodeTypeRust;
//! use std::str::FromStr;
//!
//! let node_type = NodeTypeRust::from_str("function_item")?;
//! assert_eq!(node_type, NodeTypeRust::FunctionItem);
//! assert_eq!(node_type.to_string(), "function_item");
//! # }
//! # Ok::<(), String>(())
//! ```
//!
//! # Language-Specific Type Names
//!
//! Each language's `NodeType` is re-exported with the language name in `PascalCase`:
//!
//! - **Rust**: `tree_sitter_symbols_rust::NodeType` → `NodeTypeRust`
//! - **Python** (future): `tree_sitter_symbols_python::NodeType` → `NodeTypePython`
//!
//! This naming convention prevents confusion when working with multiple language grammars
//! simultaneously. You always know which language a node type belongs to.
//!
//! # Features
//!
//! **By default, this crate has no features enabled and provides no functionality.** You must
//! explicitly enable the languages you need.
//!
//! ## Language Selection
//!
//! Enable languages individually or all at once:
//!
//! ```toml
//! # Single language with metadata only (no node types)
//! tss = { version = "0.2", features = ["lang-rust"] }
//!
//! # All languages with metadata only (currently just Rust)
//! tss = { version = "0.2", features = ["lang-all"] }
//!
//! # All languages with full node types (currently just Rust)  
//! tss = { version = "0.2", features = ["lang-all-full"] }
//! ```
//!
//! ## Language Variants: Metadata-Only vs. Full
//!
//! Each language has two feature variants:
//!
//! ### `lang-{name}` - Metadata Only
//!
//! Enables the language crate with `meta_full` but **no node types**. The `NodeType` enum
//! exists but has no variants, making it essentially unusable for pattern matching. This is
//! useful if you only need the metadata features or plan to enable specific node types manually.
//!
//! ```toml
//! # Rust with metadata only - NodeType enum exists but is empty
//! tss = { version = "0.2", features = ["lang-rust"] }
//! ```
//!
//! ### `lang-{name}-full` - Complete Functionality  
//!
//! Enables the language crate with the `full` feature, which includes both `meta_full` and
//! `node_full`. This gives you all node types and all metadata - complete functionality.
//!
//! ```toml
//! # Rust with all node types + all metadata
//! tss = { version = "0.2", features = ["lang-rust-full"] }
//! ```
//!
//! ### Aggregate Features
//!
//! - **`lang-all`**: Enables all languages with metadata only (no node types)
//! - **`lang-all-full`**: Enables all languages with full functionality (all nodes + metadata)
//!
//! ```toml
//! # All languages, metadata only
//! tss = { version = "0.2", features = ["lang-all"] }
//!
//! # All languages, complete functionality
//! tss = { version = "0.2", features = ["lang-all-full"] }
//! ```
//!
//! ## Combining with Language Crate Features
//!
//! For fine-grained control, enable `tss` with a metadata-only language, then add the underlying
//! language crate with specific node type features:
//!
//! ```toml
//! [dependencies]
//! tss = { version = "0.2", features = ["lang-rust"] }
//! tss-rust = { version = "0.2", features = [
//!     "function_item",
//!     "struct_item",
//!     "impl_item"
//! ]}
//! ```
//!
//! This gives you `NodeTypeRust` through `tss` with only the specific node types you need,
//! plus all metadata from the `lang-rust` feature.
//!
//! # When to Use This Crate
//!
//! **Use `tss`** when:
//! - You're working with multiple tree-sitter languages
//! - You want a unified import for all language support
//! - You prefer explicit type names like `NodeTypeRust` vs. `NodeType`
//! - You want `lang-all` or `lang-all-full` to easily enable all supported languages
//!
//! **Use language crates directly** (e.g., `tss-rust`) when:
//! - You're only working with a single language
//! - You want minimal dependencies
//! - You prefer the simpler `NodeType` name
//! - You need fine-grained control over individual node type features
//!
//! # How It Works
//!
//! This crate is a thin wrapper that re-exports the underlying language crates based on
//! enabled features. It adds zero runtime overhead - the re-exports are resolved at compile
//! time and generate identical code to using the language crates directly.
//!
//! Each language crate (`tss-rust`, etc.) generates node type enums at build time from the
//! corresponding `tree-sitter-*` crate's `NODE_TYPES` constant. See the individual language
//! crate documentation for details on code generation and available metadata features.
//!
//! # Available Languages
//!
//! Currently supported:
//! - **Rust** (`lang-rust`, `lang-rust-full`)
//!
//! Coming soon:
//! - **Python** (`lang-python`, `lang-python-full`)
//!
//! Additional languages will be added based on community demand.

#![allow(clippy::negative_feature_names)]
#![allow(clippy::redundant_feature_names)]

#[cfg(feature = "lang-rust")]
/// Re-export of the complete `tree_sitter_symbols_rust` module.
///
/// Provides access to all types and functions from the Rust language crate.
/// Use this when you need the full module API, or prefer to access `NodeType`
/// through the module path.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "lang-rust")]
/// # {
/// use tree_sitter_symbols::tree_sitter_symbols_rust;
///
/// // Access through the module
/// let node_type = tree_sitter_symbols_rust::NodeType::FunctionItem;
/// # }
/// ```
pub use tree_sitter_symbols_rust;

#[cfg(feature = "lang-rust")]
/// Rust language node type enum.
///
/// This is a re-export of `tree_sitter_symbols_rust::NodeType` with a language-specific
/// name to avoid confusion when working with multiple languages.
///
/// # Examples
///
/// ```
/// # #[cfg(all(feature = "lang-rust-full"))]
/// # {
/// use tree_sitter_symbols::NodeTypeRust;
/// use std::str::FromStr;
///
/// let node_type = NodeTypeRust::from_str("function_item")?;
/// assert_eq!(node_type, NodeTypeRust::FunctionItem);
/// # }
/// # Ok::<(), String>(())
/// ```
///
/// See the [`tree_sitter_symbols_rust`] module documentation for details on available
/// features and metadata.
pub use tree_sitter_symbols_rust::NodeType as NodeTypeRust;

#[cfg(feature = "lang-rust-full")]
#[cfg(test)]
mod tests {
    #[cfg(feature = "lang-rust")]
    use super::*;

    #[cfg(feature = "lang-rust")]
    mod rust_tests {
        use super::tree_sitter_symbols_rust::*;
        use std::str::FromStr;

        #[test]
        fn test_from_str() {
            assert_eq!(
                NodeType::from_str("function_item").unwrap(),
                NodeType::FunctionItem
            );
        }

        #[test]
        fn test_display() {
            assert_eq!(NodeType::FunctionItem.to_string(), "function_item");
        }
    }
}
