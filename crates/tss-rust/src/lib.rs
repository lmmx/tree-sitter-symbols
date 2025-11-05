#![allow(clippy::negative_feature_names)]
#![allow(clippy::redundant_feature_names)]
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
