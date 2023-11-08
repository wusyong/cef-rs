use std::env;

// Example custom build script.
fn main() {
    let home = env::var("HOME").unwrap();
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rustc-link-search={}/.local/share/cef",home);
}