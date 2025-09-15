fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_dir = std::path::PathBuf::from(&crate_dir);
    let workspace_dir = crate_dir.parent().unwrap();

    let config_file = workspace_dir.join("cbindgen.toml");

    let include_dir = workspace_dir.join("include");
    let out_file = include_dir.join("resl.h");

    std::fs::create_dir_all(&include_dir).unwrap();

    cbindgen::generate_with_config(
        &crate_dir,
        cbindgen::Config::from_file(config_file).unwrap(),
    )
    .expect("Unable to generate bindings")
    .write_to_file(out_file);

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=cbindgen.toml");
}
