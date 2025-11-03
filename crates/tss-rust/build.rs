use std::env;
use std::fs::File;
use std::path::Path;

mod codegen;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let mut f = File::create(dest_path).unwrap();

    codegen::generator::generate(&mut f).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/codegen.rs");
    println!("cargo:rerun-if-changed=src/codegen");
}
