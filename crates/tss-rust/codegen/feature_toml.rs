use std::io;

use ropey::Rope;
use textum::{Boundary, BoundaryMode, Patch, PatchError, Snippet, Target};

pub fn update_cargo_toml_features(feature_names: &[String]) -> Result<(), PatchError> {
    let cargo_toml_path = "Cargo.toml";

    // Read the current Cargo.toml
    let content = std::fs::read_to_string(cargo_toml_path)?;
    let mut rope = Rope::from_str(&content);

    // Generate feature lines
    let mut feature_lines = String::from("\n");
    for feat in feature_names {
        feature_lines.push_str(&format!("{feat} = []\n"));
    }

    // Create the patch using textum
    let start = Boundary::new(
        Target::Literal("# <!-- generated-features-start -->".to_string()),
        BoundaryMode::Exclude,
    );
    let end = Boundary::new(
        Target::Literal("# <!-- generated-features-end -->".to_string()),
        BoundaryMode::Exclude,
    );
    let snippet = Snippet::Between { start, end };

    let patch = Patch {
        file: cargo_toml_path.to_string(),
        snippet,
        replacement: feature_lines,
    };

    // Apply the patch
    patch.apply(&mut rope)?;

    // Write back to file
    std::fs::write(cargo_toml_path, rope.to_string())?;

    Ok(())
}
