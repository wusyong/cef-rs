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

#[derive(Debug)]
struct MethodArgument {
    name: String,
    ty: String,
}

#[derive(Debug)]
struct MethodDeclaration {
    name: String,
    args: Vec<MethodArgument>,
    output: Option<String>,
}

impl Display for MethodDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let args = self
            .args
            .iter()
            .map(|arg| {
                if arg.name == "self_" {
                    String::from("&self")
                } else {
                    format!("{}: {}", arg.name, arg.ty)
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let output = self
            .output
            .as_deref()
            .map(|output| format!(" -> {output}"))
            .unwrap_or_default();
        writeln!(f, "    fn {name}({args}){output} {{}}")
    }
}

impl TryFrom<&syn::Field> for MethodDeclaration {
    type Error = Unrecognized;

    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        let name = value
            .ident
            .as_ref()
            .ok_or(Unrecognized::FieldType)?
            .to_string();

        // Look for a type matching std::option::Option<T>
        let syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path { segments, .. },
        }) = &value.ty
        else {
            return Err(Unrecognized::FieldType);
        };
        let mut segments_iter = segments.iter();
        let (
            Some(syn::PathSegment {
                ident: ident_std,
                arguments: syn::PathArguments::None,
            }),
            Some(syn::PathSegment {
                ident: ident_option,
                arguments: syn::PathArguments::None,
            }),
            Some(syn::PathSegment {
                ident: ident_type,
                arguments:
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args, ..
                    }),
            }),
            None,
        ) = (
            segments_iter.next(),
            segments_iter.next(),
            segments_iter.next(),
            segments_iter.next(),
        )
        else {
            return Err(Unrecognized::FieldType);
        };
        if ident_std.to_string() != "std"
            || ident_option.to_string() != "option"
            || ident_type.to_string() != "Option"
            || args.len() != 1
        {
            return Err(Unrecognized::FieldType);
        }

        // See if the Option<T> type is a function pointer
        let mut args = args.iter();
        let (
            Some(syn::GenericArgument::Type(syn::Type::BareFn(syn::TypeBareFn {
                unsafety: Some(_),
                abi: Some(syn::Abi {
                    name: Some(abi), ..
                }),
                inputs,
                variadic: None,
                output,
                ..
            }))),
            None,
        ) = (args.next(), args.next())
        else {
            return Err(Unrecognized::FieldType);
        };
        if abi.value() != "C" {
            return Err(Unrecognized::FieldType);
        }

        // Looks like a match, convert it to a MethodDeclaration
        let args = inputs
            .iter()
            .filter_map(|arg| {
                if let syn::BareFnArg {
                    name: Some((name, _)),
                    ty,
                    ..
                } = arg
                {
                    Some(MethodArgument {
                        name: name.to_string(),
                        ty: type_to_string(ty),
                    })
                } else {
                    None
                }
            })
            .collect();
        let output = match output {
            syn::ReturnType::Type(_, ty) => Some(type_to_string(ty)),
            _ => None,
        };

        Ok(Self { name, args, output })
    }
}

#[derive(Debug)]
struct FieldDeclaration {
    name: String,
    ty: String,
}

impl Display for FieldDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let ty = &self.ty;
        write!(f, "    {name}: {ty},")
    }
}

impl TryFrom<&syn::Field> for FieldDeclaration {
    type Error = Unrecognized;

    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        let name = value
            .ident
            .as_ref()
            .ok_or(Unrecognized::FieldType)?
            .to_string();
        let ty = type_to_string(&value.ty);

        Ok(Self { name, ty })
    }
}

#[derive(Debug, Default)]
struct StructDeclaration {
    name: String,
    fields: Vec<FieldDeclaration>,
    methods: Vec<MethodDeclaration>,
}

#[derive(Debug, Default)]
struct ParseTree {
    type_aliases: BTreeMap<String, String>,
    enums: Vec<String>,
    structs: Vec<StructDeclaration>,
}

impl Display for ParseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "//! ParseTree")?;
        for (alias_name, alias_ty) in &self.type_aliases {
            writeln!(f, "type {} = {};", alias_name, alias_ty)?;
        }
        for StructDeclaration {
            name,
            fields,
            methods,
        } in &self.structs
        {
            if fields.is_empty() {
                writeln!(f, "struct {name};")?;
            } else {
                writeln!(f, "struct {name} {{")?;
                for field in fields {
                    writeln!(f, "{field}")?;
                }
                writeln!(f, "}}")?;
            }

            if !methods.is_empty() {
                writeln!(f, "impl {name} {{")?;
                for method in methods {
                    writeln!(f, "{method}")?;
                }
                writeln!(f, "}}")?;
            }
        }
        for enum_name in &self.enums {
            writeln!(f, "enum {enum_name};")?;
        }
        Ok(())
    }
}

impl TryFrom<&syn::File> for ParseTree {
    type Error = Unrecognized;

    fn try_from(value: &syn::File) -> Result<Self, Self::Error> {
        let mut tree = Self::default();
        for item in &value.items {
            match item {
                syn::Item::Type(item_type) => {
                    let alias_name = item_type.ident.to_string();
                    let alias_ty = type_to_string(&item_type.ty);
                    tree.type_aliases.insert(alias_name, alias_ty);
                }
                syn::Item::Struct(item_struct) => {
                    if let syn::Fields::Named(fields) = &item_struct.fields {
                        let mut struct_decl = StructDeclaration::default();
                        struct_decl.name = item_struct.ident.to_string();
                        for field in fields.named.iter() {
                            if let Ok(field_decl) = MethodDeclaration::try_from(field) {
                                struct_decl.methods.push(field_decl);
                            } else if let Ok(field_decl) = FieldDeclaration::try_from(field) {
                                struct_decl.fields.push(field_decl);
                            }
                        }
                        tree.structs.push(struct_decl);
                    }
                }
                syn::Item::Enum(syn::ItemEnum { ident, .. }) => {
                    tree.enums.push(ident.to_string());
                }
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

fn type_to_string(ty: &syn::Type) -> String {
    ty.to_token_stream().to_string()
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
