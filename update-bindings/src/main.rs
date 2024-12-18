#[macro_use]
extern crate thiserror;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing Parent")]
    MissingParent(std::path::PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    Syn(#[from] syn::Error),
    #[error("Parsing bindgen output failed")]
    Parse(#[from] parse_tree::Unrecognized),
    #[error("Missing Path")]
    MissingPath(std::path::PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;

mod dirs;
mod parse_tree;

fn main() -> Result<()> {
    let mut sys_bindings = dirs::get_sys_dir()?;
    sys_bindings.push("src/bindings.rs");
    let mut cef_bindings = dirs::get_cef_dir()?;
    cef_bindings.push("src/bindings.rs");

    println!("cef: {}", sys_bindings.display());
    println!("sys: {}", cef_bindings.display());
    let tree = parse_tree::parse_bindings(&sys_bindings)?;
    println!("\n{tree}");

    Ok(())
}
