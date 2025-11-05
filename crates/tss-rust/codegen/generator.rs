use super::rust_docs::get_doc_info;
use super::schema::NodeType;
use crate::codegen::utils::to_pascal_case;
use std::collections::HashMap;
use std::io::{self, Write};

/// Canonical feature name from the original node type and the named flag.
/// Uses the alias produced by `to_pascal_case`, converts to `snake_case`,
/// and appends _token for unnamed nodes.
fn feature_name(original: &str, named: bool) -> String {
    // use the exact same aliasing logic you already have for variants
    let alias = to_pascal_case(original);
    let mut feat = alias_to_snake(&alias);
    if !named {
        feat.push_str("_token");
    }
    feat
}

/// Convert `PascalCase` (alias) to `snake_case` feature name.
/// Examples:
///   "`DotDot`" -> "`dot_dot`"
///   "`MacroRulesBang`" -> "`macro_rules_bang`"
fn alias_to_snake(alias: &str) -> String {
    let mut s = String::with_capacity(alias.len() + 4);
    for (i, ch) in alias.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                s.push('_');
            }
            for low in ch.to_lowercase() {
                s.push(low);
            }
        } else {
            s.push(ch);
        }
    }
    s
}

pub fn generate<W: Write>(f: &mut W) -> io::Result<Vec<String>> {
    let node_types_json = tree_sitter_rust::NODE_TYPES;
    let node_types: Vec<NodeType> = serde_json::from_str(node_types_json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let variant_map = build_variant_map(&node_types);

    generate_enum(f, &node_types, &variant_map)?;
    generate_from_str(f, &node_types, &variant_map)?;
    generate_display(f, &node_types, &variant_map)?;

    // Collect all feature names
    let mut features = Vec::new();
    for (i, (original, _)) in variant_map.iter().enumerate() {
        let feat = feature_name(original, node_types[i].named);
        features.push(feat);
    }
    features.sort();

    let all_feats_count = features.len();
    features.dedup();
    if all_feats_count != features.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Duplicate feature names detected.",
        ));
    }

    Ok(features)
}

fn build_variant_map(node_types: &[NodeType]) -> Vec<(String, String)> {
    let mut seen = HashMap::new();
    let mut result = Vec::new();

    for node_type in node_types {
        let original = &node_type.node_type_name;
        let mut base_variant = to_pascal_case(original);

        // Add suffix for unnamed nodes to distinguish them
        if !node_type.named {
            base_variant.push_str("Token");
        }

        let variant_name = if let Some(count) = seen.get_mut(&base_variant) {
            *count += 1;
            format!("{base_variant}{count}")
        } else {
            seen.insert(base_variant.clone(), 1);
            base_variant
        };

        result.push((original.clone(), variant_name));
    }

    result
}

fn generate_enum<W: Write>(
    f: &mut W,
    node_types: &[NodeType],
    variant_map: &[(String, String)],
) -> io::Result<()> {
    writeln!(f, "/// Tree-sitter node types for Rust")?;
    writeln!(f, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(f, "pub enum NodeType {{")?;

    // Build a map to find paired variants
    let mut name_variants: std::collections::HashMap<&str, Vec<(&str, bool)>> =
        std::collections::HashMap::new();
    for (i, (original, variant_name)) in variant_map.iter().enumerate() {
        name_variants
            .entry(original.as_str())
            .or_default()
            .push((variant_name.as_str(), node_types[i].named));
    }

    for (i, (original, variant_name)) in variant_map.iter().enumerate() {
        let named = node_types[i].named;
        let named_str = if named { "named" } else { "unnamed" };
        writeln!(f, "    /// `{original}` ({named_str})")?;

        if let Some((label, link)) = get_doc_info(original) {
            writeln!(f, "    ///")?;
            writeln!(f, "    /// - **Rust reference**: [{label}]({link})")?;
        }

        // If there's a paired variant (named vs unnamed), emit cross-ref doc.
        let variants = &name_variants[original.as_str()];
        if variants.len() > 1 {
            let other = variants
                .iter()
                .find(|(v, n)| *v != variant_name && *n != node_types[i].named)
                .map(|(v, _)| v);
            if let Some(other_variant) = other {
                writeln!(f, "    ///")?;
                writeln!(
                    f,
                    "    /// - {} variant: [`Self::{}`]",
                    if node_types[i].named {
                        "unnamed"
                    } else {
                        "named"
                    },
                    other_variant
                )?;
            }
        }

        // Compute the feature name (original or original_token)
        let feat = feature_name(original, named);

        // Gate the variant itself on its feature or the global one
        writeln!(
            f,
            "    #[cfg(any(feature = \"{feat}\", feature = \"node_full\"))]"
        )?;
        writeln!(f, "    {variant_name},")?;
    }

    writeln!(f, "}}")?;
    writeln!(f)?;
    Ok(())
}

fn generate_from_str<W: Write>(
    f: &mut W,
    node_types: &[NodeType],
    variant_map: &[(String, String)],
) -> io::Result<()> {
    // build list of all generated feature names (from aliases) + global
    let mut all_feature_list = Vec::new();
    for (i, (original, _variant_name)) in variant_map.iter().enumerate() {
        let feat = feature_name(original, node_types[i].named);
        all_feature_list.push(format!("feature = \"{feat}\""));
    }
    all_feature_list.push("feature = \"node_full\"".to_string());
    let any_cfg = format!("any({})", all_feature_list.join(", "));

    // gate the whole impl
    writeln!(f, "#[cfg({any_cfg})]")?;
    writeln!(f, "impl std::str::FromStr for NodeType {{")?;
    writeln!(f, "    type Err = String;")?;
    writeln!(f)?;
    writeln!(f, "    #[allow(clippy::too_many_lines)]")?;
    writeln!(f, "    fn from_str(s: &str) -> Result<Self, Self::Err> {{")?;
    writeln!(f, "        match s {{")?;

    for (i, (original, variant_name)) in variant_map.iter().enumerate() {
        if node_types[i].named {
            let feat = feature_name(original, node_types[i].named);
            writeln!(
                f,
                "            #[cfg(any(feature = \"{feat}\", feature = \"node_full\"))]"
            )?;
            writeln!(f, "            {original:?} => Ok(Self::{variant_name}),")?;
        }
    }

    writeln!(
        f,
        "            _ => Err(format!(\"Unknown node type: {{s}}\")),"
    )?;
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    writeln!(f)?;
    Ok(())
}

fn generate_display<W: Write>(
    f: &mut W,
    node_types: &[NodeType],
    variant_map: &[(String, String)],
) -> io::Result<()> {
    // Build any(...) cfg with all valid feature names (derived from aliases)
    let mut feats = Vec::new();
    for (i, (original, _variant_name)) in variant_map.iter().enumerate() {
        let named = node_types[i].named;
        let feat_name = feature_name(original, named);
        feats.push(format!("feature = \"{feat_name}\""));
    }
    feats.push("feature = \"node_full\"".to_string());
    let any_cfg = format!("any({})", feats.join(", "));

    writeln!(f, "#[cfg({any_cfg})]")?;
    writeln!(f, "impl std::fmt::Display for NodeType {{")?;
    writeln!(f, "    #[allow(clippy::match_same_arms)]")?;
    writeln!(f, "    #[allow(clippy::too_many_lines)]")?;
    writeln!(
        f,
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )?;
    writeln!(f, "        match self {{")?;

    for (i, (original, variant_name)) in variant_map.iter().enumerate() {
        let named = node_types[i].named;
        let feat_name = feature_name(original, named);

        writeln!(
            f,
            "            #[cfg(any(feature = \"{feat_name}\", feature = \"node_full\"))]"
        )?;

        // Escape special characters for Rust string literals in generated code
        let escaped = original
            .replace('\\', r"\\") // backslash first!
            .replace('"', r#"\""#) // escape double quotes
            .replace('{', "{{") // escape opening brace for format strings
            .replace('}', "}}"); // escape closing brace for format strings

        writeln!(
            f,
            "            Self::{variant_name} => write!(f, \"{escaped}\"),"
        )?;
    }

    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    Ok(())
}
