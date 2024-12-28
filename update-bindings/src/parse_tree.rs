use convert_case::{Case, Casing};
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::{self, Debug, Display, Formatter},
    fs,
    io::Write,
    iter::{self, Iterator},
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

pub fn generate_bindings(source_path: &Path) -> crate::Result<PathBuf> {
    let bindings = crate::read_bindings(source_path)?;
    let parsed = syn::parse_file(&bindings)?;
    let parse_tree = ParseTree::from(&parsed);

    let mut out_file = crate::dirs::get_out_dir();
    out_file.push("bindings.rs");
    let mut bindings = fs::File::create(&out_file)?;
    write!(bindings, "{}", parse_tree)?;
    format_bindings(&out_file)?;

    Ok(out_file)
}

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

struct TypeAliasRef<'a> {
    name: String,
    ty: &'a syn::Type,
}

struct FieldRef<'a> {
    name: String,
    ty: &'a syn::Type,
}

impl<'a> TryFrom<&'a syn::Field> for FieldRef<'a> {
    type Error = Unrecognized;

    fn try_from(value: &'a syn::Field) -> Result<Self, Self::Error> {
        let name = value
            .ident
            .as_ref()
            .ok_or(Unrecognized::FieldType)?
            .to_string();

        Ok(Self {
            name,
            ty: &value.ty,
        })
    }
}

#[derive(Clone)]
struct FnArgRef<'a> {
    name: String,
    ty: &'a syn::Type,
}

struct SignatureRef<'a> {
    name: String,
    inputs: Vec<FnArgRef<'a>>,
    output: Option<&'a syn::Type>,
}

impl SignatureRef<'_> {
    fn get_rust_args(&self, tree: &ParseTree) -> proc_macro2::TokenStream {
        let mut args = self
            .inputs
            .iter()
            .map(|arg| {
                Some((
                    make_snake_case_value_name(&arg.name),
                    tree.resolve_argument_type(&arg.ty),
                ))
            })
            .collect::<Vec<_>>();
        for i in 1..(args.len()) {
            let replacement = match (&args[i - 1], &args[i]) {
                (Some((count_name, Some(count_ty))), Some((elem_name, Some(elem_ty))))
                    if count_name.as_str() == format!("{elem_name}_count").as_str() =>
                {
                    if count_ty.ty.to_token_stream().to_string().as_str()
                        != format_ident!("usize")
                            .to_token_stream()
                            .to_string()
                            .as_str()
                    {
                        continue;
                    }

                    let elem_ty_string = elem_ty.ty.to_token_stream().to_string();
                    match tree.cef_name_map.get(&elem_ty_string) {
                        Some(NameMapEntry {
                            name,
                            ty: NameMapType::StructDeclaration,
                        }) => {
                            let name = format_ident!("{name}");
                            let slice_ty =
                                match (count_ty.modifiers.as_slice(), elem_ty.modifiers.as_slice())
                                {
                                    ([], [TypeModifier::ConstPtr, TypeModifier::MutPtr]) => {
                                        quote! { &[#name] }
                                    }
                                    (
                                        [TypeModifier::MutPtr],
                                        [TypeModifier::MutPtr, TypeModifier::MutPtr],
                                    ) => quote! { &mut [#name] },
                                    _ => continue,
                                };
                            let Ok(slice_ty) = syn::parse2::<syn::Type>(slice_ty) else {
                                continue;
                            };

                            // Remove the count argument and replace the element pointer argument
                            // with an element slice.
                            Some((
                                elem_name.clone(),
                                Some(ModifiedType {
                                    modifiers: Default::default(),
                                    ty: slice_ty,
                                }),
                            ))
                        }
                        entry => {
                            let name = entry
                                .map(|entry| entry.name.as_str())
                                .unwrap_or(elem_ty_string.as_str());
                            let name = format_ident!("{name}");
                            let slice_ty =
                                match (count_ty.modifiers.as_slice(), elem_ty.modifiers.as_slice())
                                {
                                    ([], [TypeModifier::ConstPtr]) => {
                                        quote! { &[#name] }
                                    }
                                    ([TypeModifier::MutPtr], [TypeModifier::MutPtr]) => {
                                        quote! { &mut [#name] }
                                    }
                                    _ => continue,
                                };
                            let Ok(slice_ty) = syn::parse2::<syn::Type>(slice_ty) else {
                                continue;
                            };

                            // Remove the count argument and replace the element pointer argument
                            // with an element slice.
                            Some((
                                elem_name.clone(),
                                Some(ModifiedType {
                                    modifiers: Default::default(),
                                    ty: slice_ty,
                                }),
                            ))
                        }
                    }
                }
                (Some((elem_name, Some(elem_ty))), Some((size_name, Some(size_ty))))
                    if size_name.as_str() == format!("{elem_name}_size").as_str() =>
                {
                    if elem_ty.ty.to_token_stream().to_string().as_str()
                        != quote! { ::std::os::raw::c_void }
                            .to_token_stream()
                            .to_string()
                            .as_str()
                        || size_ty.ty.to_token_stream().to_string().as_str()
                            != format_ident!("usize")
                                .to_token_stream()
                                .to_string()
                                .as_str()
                    {
                        continue;
                    }

                    let slice_ty =
                        match (size_ty.modifiers.as_slice(), elem_ty.modifiers.as_slice()) {
                            ([], [TypeModifier::ConstPtr]) => {
                                quote! { &[u8] }
                            }
                            ([], [TypeModifier::MutPtr])
                            | (
                                [TypeModifier::MutPtr],
                                [TypeModifier::MutPtr, TypeModifier::MutPtr],
                            ) => quote! { &mut [u8] },
                            _ => continue,
                        };
                    let Ok(slice_ty) = syn::parse2::<syn::Type>(slice_ty) else {
                        continue;
                    };

                    // Remove the size argument and replace the buffer pointer argument with a
                    // &[u8] slice or &mut Vec<u8>.
                    Some((
                        elem_name.clone(),
                        Some(ModifiedType {
                            modifiers: Default::default(),
                            ty: slice_ty,
                        }),
                    ))
                }
                _ => None,
            };

            if let Some(replacement) = replacement {
                args[i - 1] = Some(replacement);
                args[i] = None;
            }
        }

        let args = args.iter().flatten().filter_map(|(name, arg_ty)| {
            if name.as_str() == "self_" {
                Some(quote! { &self })
            } else {
                let arg_ty = arg_ty.as_ref()?;
                let name = format_ident!("{name}");
                let ty = arg_ty
                    .get_argument_type(tree)
                    .unwrap_or_else(|| arg_ty.ty.to_token_stream());
                Some(quote! { #name: #ty })
            }
        });

        quote! { #(#args),* }
    }

    fn get_rust_output(&self, tree: &ParseTree) -> Option<proc_macro2::TokenStream> {
        self.output.map(|output| {
            let ty = tree
                .resolve_argument_type(output)
                .and_then(|ty| ty.get_output_type(tree))
                .unwrap_or_else(|| {
                    let output = output.to_token_stream();
                    quote! { #output }
                });
            quote! { -> #ty }
        })
    }

    fn get_impl_wrapped_args(&self, tree: &ParseTree) -> proc_macro2::TokenStream {
        let args = self.inputs.iter().skip(1).map(|arg| {
            let name = make_snake_case_value_name(&arg.name);
            let name = format_ident!("{name}");
            let ty = tree.resolve_type_aliases(arg.ty);
            let ty_string = ty.to_string();
            let ty_string = ty_string.trim();
            let normalized_ty = normalize_cef_type(ty_string);

            (tree.root(&normalized_ty) == BASE_REF_COUNTED)
                .then(|| {
                    let arg_ty = tree.resolve_argument_type(arg.ty)?;
                    let entry = tree.cef_name_map.get(normalized_ty.as_ref())?;
                    let ty = match entry {
                        NameMapEntry {
                            name,
                            ty: NameMapType::StructDeclaration,
                        } => {
                            let name = format_ident!("{name}");

                            match arg_ty.modifiers.as_slice() {
                                [TypeModifier::ConstPtr] => Some(quote! { &#name }),
                                [TypeModifier::MutPtr] => Some(quote! { &mut #name }),
                                _ => None,
                            }
                        }
                        NameMapEntry {
                            name,
                            ty: NameMapType::EnumName,
                        } => {
                            let name = format_ident!("{name}");

                            match arg_ty.modifiers.as_slice() {
                                [] => Some(quote! { #name }),
                                [TypeModifier::ConstPtr] => Some(quote! { &[#name] }),
                                [TypeModifier::MutPtr] => Some(quote! { &mut [#name] }),
                                _ => None,
                            }
                        }
                        _ => None,
                    }?;
                    Some(quote! {
                        let #name = #ty(unsafe { RefGuard::from_raw_add_ref(#name) });
                    })
                })
                .flatten()
                .or_else(|| {
                    let arg_ty = tree.resolve_argument_type(arg.ty)?;
                    let ty = tree
                        .cef_name_map
                        .get(normalized_ty.as_ref())
                        .and_then(|entry| syn::parse_str::<syn::Type>(&entry.name).ok());
                    let ty = ty.as_ref().unwrap_or(&arg_ty.ty);
                    match arg_ty.modifiers.as_slice() {
                        [TypeModifier::MutPtr, ..] => Some(quote! {
                            let mut #name = WrapParamRef::<#ty>::from(#name);
                            let #name = #name.as_mut();
                        }),
                        [TypeModifier::ConstPtr, ..] => Some(quote! {
                            let #name = WrapParamRef::<#ty>::from(#name);
                            let #name = #name.as_ref();
                        }),
                        _ => None,
                    }
                })
                .unwrap_or(quote! { let #name = #name.as_raw(); })
        });

        quote! { #(#args)* }
    }

    fn get_signature(&self, tree: &ParseTree) -> proc_macro2::TokenStream {
        let name = &self.name;
        let name = format_ident!("{name}");
        let args = self.get_rust_args(tree);
        let output = self.get_rust_output(tree);
        quote! { fn #name(#args) #output }
    }
}

impl<'a> TryFrom<&'a syn::Field> for SignatureRef<'a> {
    type Error = Unrecognized;

    fn try_from(value: &'a syn::Field) -> Result<Self, Self::Error> {
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

        let inputs = inputs
            .iter()
            .filter_map(|arg| {
                arg.name.as_ref().map(|(ident, _)| FnArgRef {
                    name: ident.to_string(),
                    ty: &arg.ty,
                })
            })
            .collect();
        let output = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(ty.as_ref()),
        };

        Ok(Self {
            name,
            inputs,
            output,
        })
    }
}

const BASE_REF_COUNTED: &str = "_cef_base_ref_counted_t";

struct StructDeclarationRef<'a> {
    name: String,
    fields: Vec<FieldRef<'a>>,
    methods: Vec<SignatureRef<'a>>,
}

#[derive(Clone, Copy, Debug)]
enum NameMapType {
    TypeAlias,
    EnumName,
    StructDeclaration,
}

struct NameMapEntry {
    name: String,
    ty: NameMapType,
}

#[derive(Debug)]
enum TypeModifier {
    MutPtr,
    ConstPtr,
    MutRef,
    ConstRef,
}

struct ModifiedType {
    modifiers: Vec<TypeModifier>,
    ty: syn::Type,
}

impl ModifiedType {
    fn get_argument_type(&self, tree: &ParseTree) -> Option<proc_macro2::TokenStream> {
        let elem = self.ty.to_token_stream();
        match tree.cef_name_map.get(&elem.to_string()) {
            Some(NameMapEntry {
                name,
                ty: NameMapType::StructDeclaration,
            }) => {
                let name = format_ident!("{name}");

                match self.modifiers.as_slice() {
                    [TypeModifier::ConstPtr] => Some(quote! { &#name }),
                    [TypeModifier::MutPtr] => Some(quote! { &mut #name }),
                    [TypeModifier::MutPtr, TypeModifier::MutPtr] => {
                        Some(quote! { &mut Option<#name> })
                    }
                    [TypeModifier::ConstPtr, TypeModifier::MutPtr] => Some(quote! { &mut [#name] }),
                    [TypeModifier::ConstPtr, TypeModifier::ConstPtr] => Some(quote! { &[#name] }),
                    _ => None,
                }
            }
            Some(NameMapEntry {
                name,
                ty: NameMapType::EnumName,
            }) => {
                let name = format_ident!("{name}");

                match self.modifiers.as_slice() {
                    [] => Some(quote! { #name }),
                    [TypeModifier::ConstPtr] => Some(quote! { &[#name] }),
                    [TypeModifier::MutPtr] => Some(quote! { &mut [#name] }),
                    _ => None,
                }
            }
            _ => match self.modifiers.as_slice() {
                [TypeModifier::MutPtr] => Some(quote! { &mut #elem }),
                [TypeModifier::ConstPtr] => Some(quote! { &[#elem] }),
                _ => None,
            },
        }
    }

    fn get_output_type(&self, tree: &ParseTree) -> Option<proc_macro2::TokenStream> {
        let elem = self.ty.to_token_stream();
        tree.cef_name_map
            .get(&elem.to_string())
            .and_then(|entry| match entry {
                NameMapEntry {
                    name,
                    ty: NameMapType::StructDeclaration,
                } => {
                    let name = format_ident!("{name}");

                    match self.modifiers.as_slice() {
                        [] | [TypeModifier::MutPtr] => Some(quote! { #name }),
                        [TypeModifier::ConstPtr] => Some(quote! { &#name }),
                        [TypeModifier::ConstPtr, TypeModifier::MutPtr] => {
                            Some(quote! { &mut [#name>] })
                        }
                        [TypeModifier::ConstPtr, TypeModifier::ConstPtr] => {
                            Some(quote! { &[#name] })
                        }
                        _ => None,
                    }
                }
                NameMapEntry {
                    name,
                    ty: NameMapType::EnumName,
                } => {
                    let name = format_ident!("{name}");

                    match self.modifiers.as_slice() {
                        [] => Some(quote! { #name }),
                        [TypeModifier::ConstPtr] => Some(quote! { &[#name] }),
                        [TypeModifier::MutPtr] => Some(quote! { &mut [#name] }),
                        _ => None,
                    }
                }
                _ => match self.modifiers.as_slice() {
                    [TypeModifier::MutPtr] => Some(quote! { &mut #elem }),
                    [TypeModifier::ConstPtr] => Some(quote! { &[#elem] }),
                    [TypeModifier::MutPtr, TypeModifier::MutPtr] => {
                        Some(quote! { Option<Vec<#elem>> })
                    }
                    _ => None,
                },
            })
    }
}

impl syn::parse::Parse for ModifiedType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut modifiers = vec![];
        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![*]) {
                let _ = input.parse::<syn::Token![*]>()?;
                let lookahead = input.lookahead1();
                if lookahead.peek(syn::Token![const]) {
                    let _ = input.parse::<syn::Token![const]>()?;
                    modifiers.push(TypeModifier::ConstPtr)
                } else {
                    let _ = input.parse::<syn::Token![mut]>()?;
                    modifiers.push(TypeModifier::MutPtr)
                }
            } else if lookahead.peek(syn::Token![&]) {
                let _ = input.parse::<syn::Token![&]>()?;
                let lookahead = input.lookahead1();
                if lookahead.peek(syn::Token![mut]) {
                    let _ = input.parse::<syn::Token![mut]>()?;
                    modifiers.push(TypeModifier::MutRef)
                } else {
                    modifiers.push(TypeModifier::ConstRef)
                }
            } else {
                break;
            }
        }

        let ty = input.parse()?;
        Ok(Self { modifiers, ty })
    }
}

#[derive(Default)]
struct ParseTree<'a> {
    type_aliases: Vec<TypeAliasRef<'a>>,
    enum_names: Vec<String>,
    struct_declarations: Vec<StructDeclarationRef<'a>>,
    global_function_declarations: Vec<SignatureRef<'a>>,

    cef_name_map: BTreeMap<String, NameMapEntry>,
    rust_name_map: BTreeMap<String, NameMapEntry>,

    lookup_type_alias: BTreeMap<String, usize>,
    lookup_enum_name: BTreeMap<String, usize>,
    lookup_struct_declaration: BTreeMap<String, usize>,
    lookup_global_function_declaration: BTreeMap<String, usize>,

    base_types: BTreeMap<String, String>,
}

impl<'a> ParseTree<'a> {
    pub fn write_prelude(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let header = quote! {
            #![allow(dead_code, non_camel_case_types, unused_variables)]
            use crate::{
                rc::{ConvertParam, ConvertReturnValue, RcImpl, RefGuard, WrapParamRef},
                wrapper,
            };
            use cef_sys::*;
        }
        .to_string();
        writeln!(f, "{header}")
    }

    fn resolve_type_aliases(&self, ty: &syn::Type) -> proc_macro2::TokenStream {
        match ty {
            syn::Type::Path(syn::TypePath { qself: None, path }) => {
                let ty = path.to_token_stream().to_string();
                match self.cef_name_map.get(&ty) {
                    Some(NameMapEntry {
                        ty: NameMapType::TypeAlias,
                        ..
                    }) => self
                        .lookup_type_alias
                        .get(&ty)
                        .and_then(|&i| self.type_aliases.get(i))
                        .map(|alias| self.resolve_type_aliases(&alias.ty))
                        .unwrap_or_else(|| path.to_token_stream()),
                    _ => path.to_token_stream(),
                }
            }
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                let elems = elems.iter().map(|elem| self.resolve_type_aliases(elem));
                quote! { #(#elems),* }
            }
            syn::Type::Array(syn::TypeArray { elem, len, .. }) => {
                let elem = self.resolve_type_aliases(elem);
                let len = len.to_token_stream();
                quote! { [#elem; #len] }
            }
            syn::Type::Slice(syn::TypeSlice { elem, .. }) => {
                let elem = self.resolve_type_aliases(elem);
                quote! { [#elem] }
            }
            syn::Type::Ptr(syn::TypePtr {
                const_token, elem, ..
            }) => {
                let elem = self.resolve_type_aliases(elem.as_ref());
                if const_token.is_some() {
                    quote! { *const #elem }
                } else {
                    quote! { *mut #elem }
                }
            }
            _ => ty.to_token_stream(),
        }
    }

    fn resolve_argument_type(&self, ty: &syn::Type) -> Option<ModifiedType> {
        let ty = self.resolve_type_aliases(ty);
        syn::parse2::<ModifiedType>(ty.clone()).ok()
    }

    pub fn write_aliases(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let aliases = self
            .type_aliases
            .iter()
            .filter_map(|TypeAliasRef { name, ty }| {
                let rust_name = make_rust_type_name(name.as_str())?;
                let arg_ty = self.resolve_argument_type(ty)?;
                let ty = arg_ty.ty.to_token_stream().to_string();
                let ty = make_rust_type_name(&ty).unwrap_or(ty);
                (rust_name.as_str() != ty.as_str()).then(|| {
                    let name = format_ident!("{rust_name}");
                    let modifiers = arg_ty.modifiers.iter().map(|modifier| match modifier {
                        TypeModifier::MutPtr => quote! { *mut },
                        TypeModifier::ConstPtr => quote! { *const },
                        TypeModifier::MutRef => quote! { &mut },
                        TypeModifier::ConstRef => quote! { & },
                    });
                    let ty = format_ident!("{ty}");
                    quote! {
                        pub type #name = #(#modifiers)* #ty;
                    }
                })
            });

        let aliases = quote! {
            #(#aliases)*
        }
        .to_string();

        writeln!(f, "\n// Type aliases")?;
        writeln!(f, "{aliases}")
    }

    fn base(&self, name: &str) -> Option<&str> {
        self.base_types.get(name).map(String::as_str)
    }

    fn root<'b: 'c, 'c>(&'b self, name: &'c str) -> &'c str {
        self.base(name).map(|base| self.root(base)).unwrap_or(name)
    }

    pub fn write_structs(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let declarations = self.struct_declarations.iter().filter_map(|s| {
            let Some(NameMapEntry {
                name: rust_name,
                ty: NameMapType::StructDeclaration,
            }) = self.cef_name_map.get(&s.name)
            else {
                return None;
            };
            let rust_name = format_ident!("{rust_name}");

            let name = s.name.as_str();
            let name_ident = format_ident!("{name}");
            let comment = format!(r#"See [{name}] for more documentation."#);
            let root = self.root(&s.name);

            let wrapper = if root == BASE_REF_COUNTED && root != s.name {
                let methods = s.methods.iter().map(|m| {
                    let sig = m.get_signature(self);
                    quote! {
                        #sig;
                    }
                });

                let base_name = self.base(name);
                let impl_trait = format_ident!("Impl{rust_name}");
                let impl_base_name = base_name
                    .filter(|base| *base != BASE_REF_COUNTED)
                    .and_then(|base| self.cef_name_map.get(base))
                    .map(|entry| {
                        let base = &entry.name;
                        format_ident!("Impl{base}")
                    })
                    .unwrap_or(format_ident!("Sized"));
                let impl_methods = s.methods.iter().map(|m| {
                    let sig = m.get_signature(self);
                    quote! {
                        #sig {
                            unsafe { std::mem::zeroed() }
                        }
                    }
                });

                let mut base_name = base_name;
                let mut base_structs = vec![];
                while let Some(next_base) = base_name
                    .filter(|base| *base != root)
                    .and_then(|base| self.lookup_struct_declaration.get(base))
                    .and_then(|&i| self.struct_declarations.get(i))
                {
                    base_name = self.base(&next_base.name);
                    base_structs.push(next_base);
                }

                let init_bases = base_structs
                    .into_iter()
                    .enumerate()
                    .map(|(i, base_struct)| {
                        let name = &base_struct.name;
                        let name = format_ident!("{name}");
                        let impl_mod = format_ident!("impl{name}");
                        let bases = iter::repeat_n(format_ident!("base"), i + 1);
                        quote! {
                            #impl_mod::init_methods::<Self>(&mut object.#(#bases).*);
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev();

                let name = &s.name;
                let impl_mod = format_ident!("impl{name}");
                let init_methods = s.methods.iter().map(|m| {
                    let name = &m.name;
                    let name = format_ident!("{name}");
                    quote! {
                        object.#name = Some(#name::<I>);
                    }
                });

                let wrapped_methods = s.methods.iter().map(|m| {
                    let name = &m.name;
                    let name = format_ident!("{name}");
                    let args = m.inputs.iter().map(|arg| {
                        let name = make_snake_case_value_name(&arg.name);
                        let name = format_ident!("{name}");
                        let ty = self.resolve_type_aliases(arg.ty);
                        quote! { #name: #ty }
                    });
                    let wrapped_args = m.get_impl_wrapped_args(self);
                    let forward_args = m.inputs.iter().skip(1).map(|arg| {
                        let name = make_snake_case_value_name(&arg.name);
                        let name = format_ident!("{name}");
                        quote! { #name }
                    });
                    let original_output = m.output.map(|ty| self.resolve_type_aliases(ty));
                    let output = original_output.as_ref().map(|output| {
                        quote! { -> #output }
                    });
                    let forward_output = original_output.map(|_output| quote! { .into() });

                    quote! {
                        extern "C" fn #name<I: #impl_trait>(#(#args),*) #output {
                            let obj: &RcImpl<_, I> = RcImpl::get(self_);
                            #wrapped_args
                            obj.interface.#name(#(#forward_args),*)#forward_output
                        }
                    }
                });

                quote! {
                    wrapper!(
                        #[doc = #comment]
                        #[derive(Clone)]
                        pub struct #rust_name(#name_ident);
                        #(#methods)*
                    );

                    pub trait #impl_trait : #impl_base_name {
                        #(#impl_methods)*

                        fn into_raw(self) -> *mut #name_ident {
                            let mut object: #name_ident = unsafe { std::mem::zeroed() };
                            #(#init_bases)*
                            #impl_mod::init_methods::<Self>(&mut object);
                            RcImpl::new(object, self) as *mut _
                        }
                    }

                    mod #impl_mod {
                        use super::*;

                        pub fn init_methods<I: #impl_trait>(object: &mut #name_ident) {
                            #(#init_methods)*
                        }

                        #(#wrapped_methods)*
                    }
                }
            } else if !s.methods.is_empty()
                || s.fields.is_empty()
                || s.fields.iter().map(|f| f.name.as_str()).eq(["_unused"])
            {
                quote! {
                    #[doc = #comment]
                    #[repr(transparent)]
                    pub struct #rust_name(#name_ident);

                    impl From<#name_ident> for #rust_name {
                        fn from(value: #name_ident) -> Self {
                            Self(value)
                        }
                    }

                    impl Into<*const #name_ident> for &#rust_name {
                        fn into(self) -> *const #name_ident {
                            self.as_ref() as *const #name_ident
                        }
                    }

                    impl Into<*mut #name_ident> for &mut #rust_name {
                        fn into(self) -> *mut #name_ident {
                            self.as_mut() as *mut #name_ident
                        }
                    }

                    impl Into<#name_ident> for #rust_name {
                        fn into(self) -> #name_ident {
                            self.0
                        }
                    }

                    impl AsRef<#name_ident> for #rust_name {
                        fn as_ref(&self) -> &#name_ident {
                            &self.0
                        }
                    }

                    impl AsMut<#name_ident> for #rust_name {
                        fn as_mut(&mut self) -> &mut #name_ident {
                            &mut self.0
                        }
                    }

                    impl Default for #rust_name {
                        fn default() -> Self {
                            unsafe { std::mem::zeroed() }
                        }
                    }
                }
            } else {
                let fields = s
                    .fields
                    .iter()
                    .map(|f| {
                        let name = &f.name;
                        let rust_name = make_snake_case_value_name(name);
                        let rust_name = format_ident!("{rust_name}");
                        let name = format_ident!("{name}");
                        let ty = self.resolve_type_aliases(f.ty);
                        let ty_string = ty.to_string();
                        let ty = match self.cef_name_map.get(&ty_string) {
                            Some(NameMapEntry { name, .. }) => {
                                let name = format_ident!("{name}");
                                quote! { #name }
                            }
                            _ => ty,
                        };
                        (rust_name, name.clone(), ty)
                    })
                    .collect::<Vec<_>>();
                let fields_decl = fields.iter().map(|(rust_name, _, ty)| {
                    quote! { #rust_name: #ty, }
                });
                let from_fields = fields.iter().map(|(rust_name, name, _)| {
                    quote! { #rust_name: value.#name.into(), }
                });
                let into_fields = fields.iter().map(|(rust_name, name, _)| {
                    quote! { #name: self.#rust_name.into(), }
                });

                quote! {
                    #[doc = #comment]
                    pub struct #rust_name {
                        #(#fields_decl)*
                    }

                    impl From<#name_ident> for #rust_name {
                        fn from(value: #name_ident) -> Self {
                            Self {
                                #(#from_fields)*
                            }
                        }
                    }

                    impl Into<#name_ident> for #rust_name {
                        fn into(self) -> #name_ident {
                            #name_ident {
                                #(#into_fields)*
                            }
                        }
                    }

                    impl Default for #rust_name {
                        fn default() -> Self {
                            unsafe { std::mem::zeroed() }
                        }
                    }
                }
            };

            Some(wrapper)
        });

        let declarations = quote! {
            #(#declarations)*
        }
        .to_string();

        writeln!(f, "\n// Struct wrappers")?;
        writeln!(f, "{declarations}")
    }

    pub fn write_enums(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let enum_names = self
            .enum_names
            .iter()
            .filter_map(|name| make_rust_type_name(name).map(|rust_name| (rust_name, name)));
        let declarations = enum_names.map(|(rust_name, name)| {
            let comment = format!(r#"See [{name}] for more documentation."#);
            let name = format_ident!("{name}");
            let rust_name = format_ident!("{rust_name}");
            quote! {
                #[doc = #comment]
                #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
                pub struct #rust_name(#name);

                impl AsRef<#name> for #rust_name {
                    fn as_ref(&self) -> &#name {
                        &self.0
                    }
                }

                impl AsMut<#name> for #rust_name {
                    fn as_mut(&mut self) -> &mut #name {
                        &mut self.0
                    }
                }

                impl From<#name> for #rust_name {
                    fn from(value: #name) -> Self {
                        Self(value)
                    }
                }

                impl Into<#name> for #rust_name {
                    fn into(self) -> #name {
                        self.0
                    }
                }

                impl Default for #rust_name {
                    fn default() -> Self {
                        unsafe { std::mem::zeroed() }
                    }
                }
            }
        });

        let declarations = quote! {
            #(#declarations)*
        };

        writeln!(f, "\n// Enum aliases")?;
        writeln!(f, "{declarations}")
    }

    pub fn write_globals(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let declarations = self.global_function_declarations.iter().map(|f| {
            let original_name = f.name.as_str();
            static PATTERN: OnceLock<Regex> = OnceLock::new();
            let pattern = PATTERN.get_or_init(|| Regex::new(r"^cef_(\w+)$").unwrap());
            let name = pattern
                .captures(&original_name)
                .and_then(|captures| captures.get(1))
                .map(|name| name.as_str())
                .unwrap_or(original_name);
            let name = format_ident!("{name}");
            let original_name = format_ident!("{original_name}");
            let args = f.get_rust_args(self);
            let output = f.get_rust_output(self);
            let inputs = f
                .inputs
                .iter()
                .map(|arg| {
                    let rust_name = make_snake_case_value_name(&arg.name);
                    let rust_name = format_ident!("{rust_name}");
                    let ty = self.resolve_type_aliases(arg.ty);
                    (rust_name, ty)
                })
                .collect::<Vec<_>>();
            let forward_args = inputs.iter().map(|(rust_name, _)| {
                quote! { #rust_name.as_raw() }
            });
            let original_output = f.output.map(|ty| self.resolve_type_aliases(ty));
            let forward_output = original_output.map(|_output| quote! { .as_wrapper() });

            quote! {
                pub fn #name(#args) #output {
                    unsafe { #original_name(#(#forward_args),*)#forward_output }
                }
            }
        });

        let declarations = quote! {
            #(#declarations)*
        }
        .to_string();

        writeln!(f, "\n// Global function wrappers")?;
        writeln!(f, "{declarations}")
    }
}

impl<'a> Display for ParseTree<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_prelude(f)?;
        self.write_aliases(f)?;
        self.write_structs(f)?;
        self.write_enums(f)?;
        self.write_globals(f)
    }
}

impl<'a> From<&'a syn::File> for ParseTree<'a> {
    fn from(value: &'a syn::File) -> Self {
        let mut tree = Self::default();

        tree.type_aliases = value
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Type(item_type) => Some(TypeAliasRef {
                    name: item_type.ident.to_string(),
                    ty: item_type.ty.as_ref(),
                }),
                _ => None,
            })
            .collect();

        tree.enum_names = value
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Enum(syn::ItemEnum { ident, .. }) => Some(ident.to_string()),
                syn::Item::Struct(item_struct) => match &item_struct.fields {
                    syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        Some(item_struct.ident.to_string())
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect();

        tree.struct_declarations = value
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Struct(item_struct) => match &item_struct.fields {
                    syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
                        let mut fields = vec![];
                        let mut methods = vec![];

                        for member in named.iter() {
                            if let Ok(method) = SignatureRef::try_from(member) {
                                methods.push(method);
                            } else if let Ok(field) = FieldRef::try_from(member) {
                                fields.push(field);
                            }
                        }

                        Some(StructDeclarationRef {
                            name: item_struct.ident.to_string(),
                            fields,
                            methods,
                        })
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect();

        tree.global_function_declarations = value
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::ForeignMod(syn::ItemForeignMod {
                    unsafety: Some(_),
                    abi:
                        syn::Abi {
                            name: Some(abi), ..
                        },
                    items,
                    ..
                }) if abi.value() == "C" => Some(items),
                _ => None,
            })
            .flat_map(|items| {
                items.iter().filter_map(|item| match item {
                    syn::ForeignItem::Fn(syn::ForeignItemFn {
                        sig:
                            syn::Signature {
                                ident,
                                inputs,
                                output,
                                ..
                            },
                        ..
                    }) => Some(SignatureRef {
                        name: ident.to_string(),
                        inputs: inputs
                            .iter()
                            .map(|arg| match arg {
                                syn::FnArg::Receiver(_) => {
                                    unreachable!("unexpected function receiver")
                                }
                                syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                                    match pat.as_ref() {
                                        syn::Pat::Ident(syn::PatIdent { ident, .. }) => FnArgRef {
                                            name: ident.to_string(),
                                            ty: ty.as_ref(),
                                        },
                                        _ => unreachable!("unexpected argument name type"),
                                    }
                                }
                            })
                            .collect(),
                        output: match output {
                            syn::ReturnType::Default => None,
                            syn::ReturnType::Type(_, ty) => Some(ty.as_ref()),
                        },
                    }),
                    _ => None,
                })
            })
            .collect();

        tree.cef_name_map = tree
            .type_aliases
            .iter()
            .map(|alias| alias.name.as_str())
            .map(|cef_name| (cef_name, NameMapType::TypeAlias))
            .chain(
                tree.enum_names
                    .iter()
                    .map(String::as_str)
                    .map(|cef_name| (cef_name, NameMapType::EnumName)),
            )
            .chain(
                tree.struct_declarations
                    .iter()
                    .map(|s| s.name.as_str())
                    .map(|cef_name| (cef_name, NameMapType::StructDeclaration)),
            )
            .filter_map(|(cef_name, ty)| {
                make_rust_type_name(cef_name).map(|rust_name| (cef_name, (rust_name, ty)))
            })
            .filter_map(|(cef_name, (rust_name, ty))| {
                if cef_name == rust_name.as_str() {
                    None
                } else {
                    Some((
                        cef_name.to_owned(),
                        NameMapEntry {
                            name: rust_name,
                            ty,
                        },
                    ))
                }
            })
            .collect();
        tree.rust_name_map = tree
            .cef_name_map
            .iter()
            .map(|(a, NameMapEntry { name: b, ty })| {
                (
                    b.clone(),
                    NameMapEntry {
                        name: a.clone(),
                        ty: *ty,
                    },
                )
            })
            .collect();

        tree.lookup_type_alias = tree
            .type_aliases
            .iter()
            .enumerate()
            .map(|(index, alias)| (alias.name.clone(), index))
            .collect();
        tree.lookup_enum_name = tree
            .enum_names
            .iter()
            .enumerate()
            .map(|(index, name)| (name.clone(), index))
            .collect();
        tree.lookup_struct_declaration = tree
            .struct_declarations
            .iter()
            .enumerate()
            .map(|(index, s)| (s.name.clone(), index))
            .collect();
        tree.lookup_global_function_declaration = tree
            .global_function_declarations
            .iter()
            .enumerate()
            .map(|(index, f)| (f.name.clone(), index))
            .collect();

        tree.base_types = tree
            .struct_declarations
            .iter()
            .filter_map(|s| match s.fields.as_slice() {
                [FieldRef { name, ty }] if name.as_str() == "base" => {
                    Some((s.name.clone(), tree.resolve_type_aliases(ty).to_string()))
                }
                _ => None,
            })
            .collect();

        tree
    }
}

fn format_bindings(source_path: &Path) -> crate::Result<()> {
    let mut cmd = Command::new("rustfmt");
    cmd.arg(source_path);
    cmd.output()?;
    Ok(())
}

fn normalize_cef_type(name: &str) -> Cow<'_, str> {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = PATTERN.get_or_init(|| Regex::new(r"^(\s*\*\s*(const|mut)\s+)*").unwrap());
    pattern.replace(name, "")
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
                format!("Cef{name}")
            } else {
                name
            }
        })
}

fn make_snake_case_value_name(name: &str) -> String {
    name.from_case(Case::Camel).to_case(Case::Snake)
}
