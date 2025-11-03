#[cfg(feature = "lang-rust")]
pub use tree_sitter_symbols_rust;
#[cfg(feature = "lang-rust")]
pub use tree_sitter_symbols_rust::NodeType as NodeTypeRust;

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
