use super::rust_docs::get_doc_info;
use super::schema::NodeType;
use super::utils::to_pascal_case;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn generate<W: Write>(f: &mut W) -> io::Result<()> {
    let node_types_json = tree_sitter_rust::NODE_TYPES;
    let node_types: Vec<NodeType> = serde_json::from_str(node_types_json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Build variant name mapping
    let variant_map = build_variant_map(&node_types);

    generate_enum(f, &node_types, &variant_map)?;
    generate_from_str(f, &node_types, &variant_map)?;
    generate_display(f, &variant_map)?;

    Ok(())
}

fn build_variant_map(node_types: &[NodeType]) -> Vec<(String, String)> {
    let mut seen = HashMap::new();
    let mut result = Vec::new();

    for node_type in node_types {
        let original = &node_type.node_type_type;
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
        let named = if node_types[i].named {
            "named"
        } else {
            "unnamed"
        };
        writeln!(f, "    /// `{original}` ({named})")?;

        // Add Rust docs link if available
        if let Some((label, link)) = get_doc_info(original) {
            writeln!(f, "    ///")?;
            writeln!(f, "    /// - **Rust reference**: [{label}]({link})")?;
        }

        // Add cross-reference if there's a paired variant
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
    writeln!(f, "impl std::str::FromStr for NodeType {{")?;
    writeln!(f, "    type Err = String;")?;
    writeln!(f)?;
    writeln!(f, "    fn from_str(s: &str) -> Result<Self, Self::Err> {{")?;
    writeln!(f, "        match s {{")?;

    for (i, (original, variant_name)) in variant_map.iter().enumerate() {
        // Only include named nodes in FromStr
        if node_types[i].named {
            writeln!(f, "            {original:?} => Ok(Self::{variant_name}),")?;
        }
    }

    writeln!(
        f,
        "            _ => Err(format!(\"Unknown node type: {{}}\", s)),"
    )?;
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    writeln!(f)?;
    Ok(())
}

fn generate_display<W: Write>(f: &mut W, variant_map: &[(String, String)]) -> io::Result<()> {
    writeln!(f, "impl std::fmt::Display for NodeType {{")?;
    writeln!(
        f,
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )?;
    writeln!(f, "        let s = match self {{")?;

    for (original, variant_name) in variant_map {
        writeln!(f, "            Self::{variant_name} => {original:?},")?;
    }

    writeln!(f, "        }};")?;
    writeln!(f, "        write!(f, \"{{}}\", s)")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    Ok(())
}
