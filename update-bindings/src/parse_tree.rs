#![allow(dead_code, unused_imports)]

use convert_case::{Boundary, Case, Casing};
use core::error;
use quote::{quote, ToTokens};
use regex::Regex;
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::{Debug, Display},
    fs,
    io::{Read, Write},
    iter::Iterator,
    path::{Path, PathBuf},
    sync::OnceLock,
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
    rust_name: String,
    ty: String,
}

#[derive(Debug)]
struct MethodDeclaration {
    name: String,
    original_name: Option<String>,
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
                    format!("{}: {}", arg.rust_name, arg.ty)
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let output = self
            .output
            .as_deref()
            .map(|output| format!(" -> {output}"))
            .unwrap_or_default();
        write!(f, "pub fn {name}({args}){output}")
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
                    let name = name.to_string();
                    let rust_name = make_snake_case_value_name(&name);
                    let ty = type_to_string(ty);
                    Some(MethodArgument {
                        name,
                        rust_name,
                        ty,
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

        Ok(Self {
            name,
            original_name: None,
            args,
            output,
        })
    }
}

#[derive(Debug)]
struct FieldDeclaration {
    name: String,
    rust_name: String,
    ty: String,
}

impl Display for FieldDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rust_name = &self.rust_name;
        let ty = &self.ty;
        write!(f, "pub {rust_name}: {ty},")
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
        let rust_name = make_snake_case_value_name(&name);
        let ty = type_to_string(&value.ty);

        Ok(Self {
            name,
            rust_name,
            ty,
        })
    }
}

#[derive(Debug, Default)]
struct StructDeclaration {
    name: String,
    rust_name: Option<String>,
    fields: Vec<FieldDeclaration>,
    methods: Vec<MethodDeclaration>,
}

struct BaseTypes(BTreeMap<String, String>);

impl BaseTypes {
    fn new<'a>(structs: impl Iterator<Item = &'a StructDeclaration>) -> Self {
        Self(
            structs
                .filter_map(|s| {
                    if s.fields.iter().map(|f| f.name.as_str()).eq(["base"]) {
                        s.fields
                            .get(0)
                            .and_then(|f| s.rust_name.as_ref().map(|n| (n.clone(), f.ty.clone())))
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }

    fn base(&self, name: &str) -> Option<&str> {
        self.0.get(name).map(String::as_str)
    }

    fn root<'a: 'b, 'b>(&'a self, name: &'b str) -> &'b str {
        self.base(name).map(|base| self.root(base)).unwrap_or(name)
    }
}

#[derive(Debug, Default)]
struct ParseTree {
    type_aliases: BTreeMap<String, String>,
    enums: Vec<String>,
    structs: Vec<StructDeclaration>,
    globals: Vec<MethodDeclaration>,
}

impl Display for ParseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = quote::quote! {
            use crate::{
                rc::{RcImpl, RefGuard},
                string::CefString,
                wrapper,
            };
        }
        .to_string();
        writeln!(f, "{}", header)?;

        for (alias_name, alias_ty) in &self.type_aliases {
            let alias_name = make_rust_type_name(&alias_name).unwrap_or_else(|| alias_name.clone());
            let alias_ty = make_rust_type_name(&alias_ty).unwrap_or_else(|| alias_ty.clone());
            if alias_name != alias_ty {
                writeln!(f, "type {} = {};", alias_name, alias_ty)?;
            }
        }

        let base_types = BaseTypes::new(self.structs.iter());

        for StructDeclaration {
            name,
            rust_name,
            fields,
            methods,
        } in &self.structs
        {
            let Some(rust_name) = rust_name.as_ref() else {
                continue;
            };

            let root = base_types.root(rust_name);
            if root == "BaseRefCounted" {
                if root != rust_name {
                    writeln!(
                        f,
                        r#"
                        wrapper!(
                            #[doc("See [cef_sys::{name}] for more documentation.")]
                            #[derive(Debug, Clone)]
                            pub struct {rust_name}(cef_sys::{name});
                        "#
                    )?;
                    for method in methods {
                        writeln!(f, "    {method};")?;
                    }
                    writeln!(f, ");")?;
                }

                continue;
            }

            if fields.is_empty() {
                writeln!(f, "pub type {rust_name} = cef_sys::{name};")?;
            } else {
                writeln!(f, "\nstruct {rust_name} {{")?;
                for field in fields {
                    writeln!(f, "    {field}")?;
                }
                writeln!(f, "}}")?;
            }

            if !methods.is_empty() {
                write!(f, "\nimpl {rust_name} {{")?;
                for method in methods {
                    let output = if method.output.is_some() {
                        String::from("Default::default()")
                    } else {
                        Default::default()
                    };
                    write!(f, "\n    {method} {{ {output} }}")?;
                    writeln!(f, "\n}}")?;
                }
            }
        }

        writeln!(f, "\n// Enum aliases")?;
        for enum_name in &self.enums {
            let Some(rust_name) = make_rust_type_name(enum_name) else {
                continue;
            };
            writeln!(f, "pub type {rust_name} = cef_sys::{enum_name};")?;
        }

        writeln!(f, "\n// Global function wrappers")?;
        for global_fn in &self.globals {
            writeln!(f, "{global_fn};")?;
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
                syn::Item::Struct(item_struct) => match &item_struct.fields {
                    syn::Fields::Named(fields) => {
                        let mut struct_decl = StructDeclaration::default();
                        struct_decl.name = item_struct.ident.to_string();
                        struct_decl.rust_name = make_rust_type_name(&struct_decl.name);
                        for field in fields.named.iter() {
                            if let Ok(field_decl) = MethodDeclaration::try_from(field) {
                                struct_decl.methods.push(field_decl);
                            } else if let Ok(field_decl) = FieldDeclaration::try_from(field) {
                                struct_decl.fields.push(field_decl);
                            }
                        }
                        tree.structs.push(struct_decl);
                    }
                    syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        tree.enums.push(item_struct.ident.to_string());
                    }
                    _ => {}
                },
                syn::Item::Enum(syn::ItemEnum { ident, .. }) => {
                    tree.enums.push(ident.to_string());
                }
                syn::Item::ForeignMod(syn::ItemForeignMod {
                    unsafety: Some(_),
                    abi:
                        syn::Abi {
                            name: Some(abi), ..
                        },
                    items,
                    ..
                }) if abi.value() == "C" => {
                    for item in items {
                        if let syn::ForeignItem::Fn(item_fn) = item {
                            let original_name = item_fn.sig.ident.to_string();
                            static PATTERN: OnceLock<Regex> = OnceLock::new();
                            let pattern =
                                PATTERN.get_or_init(|| Regex::new(r"^cef_(\w+)$").unwrap());
                            let name = pattern
                                .captures(&original_name)
                                .and_then(|captures| captures.get(1))
                                .map(|name| name.as_str().to_string());
                            let (name, original_name) = match name {
                                Some(name) => (name, Some(original_name)),
                                None => (original_name, None),
                            };
                            let args = item_fn
                                .sig
                                .inputs
                                .iter()
                                .filter_map(|arg| {
                                    let syn::FnArg::Typed(syn::PatType { pat, ty, .. }) = arg
                                    else {
                                        return None;
                                    };

                                    let syn::Pat::Ident(syn::PatIdent { ident, .. }) = pat.as_ref()
                                    else {
                                        return None;
                                    };

                                    let name = ident.to_string();
                                    let rust_name = make_snake_case_value_name(&name);
                                    let ty = type_to_string(ty.as_ref());
                                    Some(MethodArgument {
                                        name,
                                        rust_name,
                                        ty,
                                    })
                                })
                                .collect();
                            let output = match &item_fn.sig.output {
                                syn::ReturnType::Type(_, ty) => Some(type_to_string(ty.as_ref())),
                                _ => None,
                            };
                            tree.globals.push(MethodDeclaration {
                                name,
                                original_name,
                                args,
                                output,
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(tree)
    }
}

pub fn generate_bindings(source_path: &Path) -> crate::Result<PathBuf> {
    let bindings = read_bindings(source_path)?;
    let parsed = syn::parse_file(&bindings)?;
    let parse_tree = ParseTree::try_from(&parsed)?;

    let mut out_file = crate::dirs::get_out_dir();
    out_file.push("bindings.rs");
    let mut bindings = std::fs::File::create(&out_file)?;
    write!(bindings, "{}", parse_tree)?;
    format_bindings(&out_file)?;

    Ok(out_file)
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

fn type_to_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(syn::TypePath { qself: None, path }) => {
            let name = path.to_token_stream().to_string();
            make_rust_type_name(&name).unwrap_or(name)
        }
        syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
            let elems = elems
                .iter()
                .map(|elem| type_to_string(elem))
                .collect::<Vec<_>>()
                .join(", ");
            format!("({elems})")
        }
        syn::Type::Array(syn::TypeArray { elem, len, .. }) => {
            let elem = type_to_string(elem);
            let len = len.to_token_stream().to_string();
            format!("[{elem}; {len}]")
        }
        syn::Type::Slice(syn::TypeSlice { elem, .. }) => {
            let elem = type_to_string(elem);
            format!("[{elem}]")
        }
        syn::Type::Ptr(syn::TypePtr {
            const_token, elem, ..
        }) => {
            let rust_name = match elem.as_ref() {
                syn::Type::Path(syn::TypePath { qself: None, path }) => {
                    let name = path.to_token_stream().to_string();
                    make_rust_type_name(&name)
                }
                _ => None,
            };

            match (rust_name, const_token) {
                (Some(rust_name), _) => rust_name,
                (None, Some(_)) => format!("*const {}", type_to_string(elem.as_ref())),
                (None, None) => format!("*mut {}", type_to_string(elem.as_ref())),
            }
        }
        _ => ty.to_token_stream().to_string(),
    }
}

fn make_rust_type_name(name: &str) -> Option<String> {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = PATTERN.get_or_init(|| Regex::new(r"^_?cef_(\w+)_t$").unwrap());
    pattern
        .captures(name)
        .and_then(|captures| captures.get(1))
        .map(|name| {
            let name = name
                .as_str()
                .from_case(Case::Snake)
                .to_case(Case::UpperCamel);
            if name.starts_with("String") {
                format!("Cef{}", name)
            } else {
                name
            }
        })
}

fn make_snake_case_value_name(name: &str) -> String {
    name.from_case(Case::Camel).to_case(Case::Snake)
}
