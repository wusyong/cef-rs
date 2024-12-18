#![allow(dead_code, unused_imports)]

use convert_case::{Boundary, Case, Casing};
use core::error;
use quote::{quote, ToTokens};
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::{Debug, Display},
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Error)]
pub enum Unrecognized {
    #[error("Unrecognized Field Type")]
    FieldType,
    #[error("Unrecognized Function Argument")]
    FnArg,
    #[error("Unrecognized Generic Type")]
    Generic,
    #[error("Unrecognized Interface Declaration")]
    Interface,
    #[error("Failed to Parse Bindings")]
    Parse(#[from] syn::Error),
}

enum FieldType<'a> {
    Value { ty: String },
    Fn { sig: &'a syn::Signature },
}

impl Debug for FieldType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Value { ty } => write!(f, "{}", ty),
            FieldType::Fn { sig } => write!(f, "fn (...)"),
        }
    }
}

#[derive(Debug)]
struct FieldDeclaration<'a> {
    name: String,
    ty: FieldType<'a>,
}

#[derive(Debug, Default)]
struct StructDeclaration<'a> {
    name: String,
    fields: Vec<FieldDeclaration<'a>>,
}

#[derive(Debug, Default)]
struct ParseTree<'a> {
    type_aliases: BTreeMap<String, String>,
    structs: Vec<StructDeclaration<'a>>,
}

impl Display for ParseTree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "//! ParseTree")?;
        for (alias_name, alias_ty) in &self.type_aliases {
            writeln!(f, "type {} = {};", alias_name, alias_ty)?;
        }
        Ok(())
    }
}

impl<'a> TryFrom<&'a syn::File> for ParseTree<'a> {
    type Error = Unrecognized;

    fn try_from(value: &syn::File) -> Result<Self, Self::Error> {
        let mut tree = Self::default();
        for item in &value.items {
            match item {
                syn::Item::Type(item_type) => {
                    let alias_name = item_type.ident.to_string();
                    let alias_ty = item_type.ty.to_token_stream().to_string();
                    tree.type_aliases.insert(alias_name, alias_ty);
                }
                // syn::Item::Struct(item_struct) => {
                //     let mut struct_decl = StructDeclaration::default();
                //     struct_decl.name = item_struct.ident.to_string();
                //     for field in &item_struct.fields {
                //         let field_name = field.ident.as_ref().unwrap().to_string();
                //         let field_ty = match &field.ty {
                //             syn::Type::Path(path) => {
                //                 let ty = path.path.segments.last().unwrap().ident.to_string();
                //                 FieldType::Value { ty }
                //             }
                //             syn::Type::BareFn(bare_fn) => FieldType::Fn { sig: &bare_fn.sig },
                //             _ => return Err(Unrecognized::FieldType),
                //         };
                //         struct_decl.fields.push(FieldDeclaration {
                //             name: field_name,
                //             ty: field_ty,
                //         });
                //     }
                //     tree.structs.push(struct_decl);
                // }
                _ => {}
            }
        }
        Ok(tree)
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
