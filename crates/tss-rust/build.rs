#![allow(clippy::too_many_lines)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::negative_feature_names)]
#![allow(clippy::redundant_feature_names)]
use std::env;
use std::fs::File;
use std::path::Path;

mod codegen;

use codegen::feature_toml::update_cargo_toml_features;
use codegen::generator::generate;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let mut f = File::create(dest_path).unwrap();

    let features = generate(&mut f).unwrap();
    update_cargo_toml_features(&features).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/codegen.rs");
    println!("cargo:rerun-if-changed=src/codegen");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
