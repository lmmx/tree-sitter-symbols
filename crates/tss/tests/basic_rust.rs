#![allow(clippy::negative_feature_names)]
#![allow(clippy::redundant_feature_names)]
#![cfg(feature = "lang-rust")]

use tree_sitter_symbols::NodeTypeRust as NodeType;

// Check that a node type type exists (empty if no node features are on)
// You cannot do much without any node types
#[test]
fn enum_exists() {
    // This prints the type name at runtime
    println!("{}", std::any::type_name::<NodeType>());
}

#[cfg(feature = "lang-rust-full")]
#[test]
fn enum_variants_exist() {
    // Check some common node types exist as variants
    let _ = NodeType::FunctionItem;
    let _ = NodeType::StructItem;
    let _ = NodeType::ImplItem;
    let _ = NodeType::ModItem;
    let _ = NodeType::Block;
    let _ = NodeType::Identifier;
}

#[cfg(feature = "lang-rust-full")]
#[test]
fn from_str_works() {
    use std::str::FromStr;

    assert_eq!(
        NodeType::from_str("function_item").unwrap(),
        NodeType::FunctionItem
    );
    assert_eq!(
        NodeType::from_str("struct_item").unwrap(),
        NodeType::StructItem
    );
    assert_eq!(NodeType::from_str("impl_item").unwrap(), NodeType::ImplItem);
}

#[cfg(feature = "lang-rust-full")]
#[test]
fn from_str_rejects_unknown() {
    use std::str::FromStr;

    assert!(NodeType::from_str("not_a_real_node_type").is_err());
}

#[cfg(feature = "lang-rust-full")]
#[test]
fn display_works() {
    assert_eq!(NodeType::FunctionItem.to_string(), "function_item");
    assert_eq!(NodeType::StructItem.to_string(), "struct_item");
    assert_eq!(NodeType::ImplItem.to_string(), "impl_item");
}

#[cfg(feature = "lang-rust-full")]
#[test]
fn roundtrip() {
    use std::str::FromStr;

    let original = "function_item";
    let parsed = NodeType::from_str(original).unwrap();
    let displayed = parsed.to_string();
    assert_eq!(original, displayed);
}
