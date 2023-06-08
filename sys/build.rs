#[cfg(not(feature = "dox"))]
fn main() {
    let path = match std::env::var("FLATPAK") {
        Ok(_) => String::from("/usr/lib"),
        Err(_) => match std::env::var("HOME") {
            Ok(mut val) => {
                val.push_str(
                    "/.local/share/cef",
                );
                val
            }
            Err(e) => panic!("Couldn't get the path of shared library: {e}"),
        },
    };
    println!("cargo:rustc-link-lib=cef");
    println!("cargo:rustc-link-search={path}");
}

#[cfg(feature = "dox")]
fn main() {}
