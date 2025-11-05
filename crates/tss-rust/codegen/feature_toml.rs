use ropey::Rope;
use textum::{Boundary, BoundaryMode, Patch, PatchError, Snippet, Target};

pub fn update_cargo_toml_features(feature_names: &[String]) -> Result<(), PatchError> {
    // Use the correct path relative to the crate root
    // During build script execution, the working directory is the crate root
    let cargo_toml_path = std::env::current_dir()
        .map_err(|e| PatchError::IoError(e))?
        .join("Cargo.toml");

    let cargo_toml_str = cargo_toml_path.to_str().ok_or_else(|| {
        PatchError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid path",
        ))
    })?;

    // Read the current Cargo.toml
    let content = std::fs::read_to_string(&cargo_toml_path)?;
    let mut rope = Rope::from_str(&content);

    // Generate feature lines
    let mut feature_lines = Vec::new();
    for feat in feature_names {
        feature_lines.push(format!("{feat} = []"));
    }
    let replacement = format!("\n{}\n", feature_lines.join("\n"));

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
        file: cargo_toml_str.to_string(),
        snippet,
        replacement,
    };

    // Apply the patch
    patch.apply(&mut rope)?;

    // Write back to file
    std::fs::write(&cargo_toml_path, rope.to_string())?;

    Ok(())
}
