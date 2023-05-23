#[cfg(not(feature = "dox"))]
fn main() {
    let path = "";
    println!("cargo:rustc-link-lib=cef");
    println!("cargo:rustc-link-search={path}");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={path}");
}

#[cfg(feature = "dox")]
fn main() {}
