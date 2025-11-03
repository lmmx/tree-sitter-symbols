#![cfg(feature = "lang-rust")]
use std::str::FromStr;
use tree_sitter_symbols::NodeTypeRust as NodeType;

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

#[test]
fn from_str_works() {
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

#[test]
fn from_str_rejects_unknown() {
    assert!(NodeType::from_str("not_a_real_node_type").is_err());
}

#[test]
fn display_works() {
    assert_eq!(NodeType::FunctionItem.to_string(), "function_item");
    assert_eq!(NodeType::StructItem.to_string(), "struct_item");
    assert_eq!(NodeType::ImplItem.to_string(), "impl_item");
}

#[test]
fn roundtrip() {
    let original = "function_item";
    let parsed = NodeType::from_str(original).unwrap();
    let displayed = parsed.to_string();
    assert_eq!(original, displayed);
}
