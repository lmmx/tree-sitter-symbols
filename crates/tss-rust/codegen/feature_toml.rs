use ropey::Rope;
use textum::{Boundary, BoundaryMode, Patch, PatchError, Snippet, Target};

pub fn update_cargo_toml_features(feature_names: &[String]) -> Result<(), PatchError> {
    // Skip feature rewriting unless explicitly enabled
    if std::env::var("REWRITE_FEATURES").is_err() {
        return Ok(());
    }

    // Use the correct path relative to the crate root
    let cargo_toml_path = std::env::current_dir()
        .map_err(PatchError::IoError)?
        .join("Cargo.toml");

    let cargo_toml_str = cargo_toml_path.to_str().ok_or_else(|| {
        PatchError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid path",
        ))
    })?;

    // Read the current Cargo.toml
    let content = std::fs::read_to_string(&cargo_toml_path)?;

    // Generate feature lines
    let mut feature_lines = Vec::new();
    for feat in feature_names {
        feature_lines.push(format!(r#"{feat} = ["node"]"#));
    }
    let new_features = format!("\n{}\n", feature_lines.join("\n"));

    // Check if features are already up to date
    let start_marker = "# <!-- generated-features-start -->";
    let end_marker = "# <!-- generated-features-end -->";

    if let Some(start_pos) = content.find(start_marker) {
        if let Some(end_pos) = content.find(end_marker) {
            let current_features = &content[start_pos + start_marker.len()..end_pos];
            if current_features.trim() == new_features.trim() {
                // Features are already up to date, no need to write
                return Ok(());
            }
        }
    }

    // Features need updating
    let mut rope = Rope::from_str(&content);

    // Create the patch using textum
    let start = Boundary::new(
        Target::Literal(start_marker.to_string()),
        BoundaryMode::Exclude,
    );
    let end = Boundary::new(
        Target::Literal(end_marker.to_string()),
        BoundaryMode::Exclude,
    );
    let snippet = Snippet::Between { start, end };

    let patch = Patch {
        file: cargo_toml_str.to_string(),
        snippet,
        replacement: new_features,
    };

    // Apply the patch
    patch.apply(&mut rope)?;

    // Write back to file
    std::fs::write(&cargo_toml_path, rope.to_string())?;

    Ok(())
}
