use cbindgen;
use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cbindgen_toml = Path::new(&crate_dir).join("cbindgen.toml");

    // println!("cargo:rerun-if-changed={}", cbindgen_toml.display());

    cbindgen::Builder::new()
        .with_config(
            cbindgen::Config::from_file(cbindgen_toml).expect("Unable to read cbindgen.toml"),
        )
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}
