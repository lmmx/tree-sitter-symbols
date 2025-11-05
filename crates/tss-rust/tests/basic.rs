#![allow(clippy::negative_feature_names)]
#![allow(clippy::redundant_feature_names)]
use std::str::FromStr;
use tree_sitter_symbols_rust::NodeType;

#[test]
fn enum_variants_exist() {
    // Check some common node types exist as variants
    #[cfg(any(feature = "function_item", feature = "node_full"))]
    let _ = NodeType::FunctionItem;
    #[cfg(any(feature = "struct_item", feature = "node_full"))]
    let _ = NodeType::StructItem;
    #[cfg(any(feature = "impl_item", feature = "node_full"))]
    let _ = NodeType::ImplItem;
    #[cfg(any(feature = "mod_item", feature = "node_full"))]
    let _ = NodeType::ModItem;
    #[cfg(any(feature = "block", feature = "node_full"))]
    let _ = NodeType::Block;
    #[cfg(any(feature = "identifier", feature = "node_full"))]
    let _ = NodeType::Identifier;
}

#[test]
fn from_str_works() {
    #[cfg(any(feature = "function_item", feature = "node_full"))]
    assert_eq!(
        NodeType::from_str("function_item").unwrap(),
        NodeType::FunctionItem
    );
    #[cfg(any(feature = "struct_item", feature = "node_full"))]
    assert_eq!(
        NodeType::from_str("struct_item").unwrap(),
        NodeType::StructItem
    );
    #[cfg(any(feature = "impl_item", feature = "node_full"))]
    assert_eq!(NodeType::from_str("impl_item").unwrap(), NodeType::ImplItem);
}

// Feature gate for any node type (need at least 1 node type to use NodeType)
#[cfg(feature = "node")]
#[test]
fn from_str_rejects_unknown() {
    assert!(NodeType::from_str("not_a_real_node_type").is_err());
}

#[test]
fn display_works() {
    #[cfg(any(feature = "function_item", feature = "node_full"))]
    assert_eq!(NodeType::FunctionItem.to_string(), "function_item");
    #[cfg(any(feature = "struct_item", feature = "node_full"))]
    assert_eq!(NodeType::StructItem.to_string(), "struct_item");
    #[cfg(any(feature = "impl_item", feature = "node_full"))]
    assert_eq!(NodeType::ImplItem.to_string(), "impl_item");
}

#[cfg(any(feature = "function_item", feature = "node_full"))]
#[test]
fn roundtrip() {
    let original = "function_item";
    let parsed = NodeType::from_str(original).unwrap();
    let displayed = parsed.to_string();
    assert_eq!(original, displayed);
}
