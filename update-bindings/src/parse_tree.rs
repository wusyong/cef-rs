#![allow(dead_code, unused_imports)]

use convert_case::{Boundary, Case, Casing};
use syn::parse::Parse;
use core::error;
use std::{
    collections::VecDeque,
    fmt::Display,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Error)]
pub enum Unrecognized {
    #[error("Unrecognized Return Type")]
    ReturnType,
    #[error("Unrecognized Function Argument")]
    FnArg,
    #[error("Unrecognized Generic Type")]
    Generic,
    #[error("Unrecognized Interface Declaration")]
    Interface,
    #[error("Failed to Parse Bindings")]
    Parse(#[from] syn::Error),
}

struct ParseTree;

impl Display for ParseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "// ParseTree")
    }
}

impl TryFrom<&syn::File> for ParseTree {
    type Error = Unrecognized;

    fn try_from(_value: &syn::File) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

pub fn parse_bindings(source_path: &Path) -> crate::Result<String> {
    let bindings = read_bindings(source_path)?;
    let parsed = syn::parse_file(&bindings)?;
    let parse_tree = ParseTree::try_from(&parsed)?;
    Ok(parse_tree.to_string())
}

fn read_bindings(source_path: &Path) -> crate::Result<String> {
    let mut source_file = fs::File::open(source_path)?;
    let mut updated = String::default();
    source_file.read_to_string(&mut updated)?;
    Ok(updated)
}

fn format_bindings(source_path: &Path) -> crate::Result<()> {
    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(source_path);
    cmd.output()?;
    Ok(())
}