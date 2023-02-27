use impl_enum::impl_enum;
use impl_struct::impl_struct;
use impl_union::impl_union;
use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DataEnum, DeriveInput, Ident, TypeParen};

mod impl_enum;
mod impl_fields;
mod impl_struct;
mod impl_union;

/// Automatically derives the `TypeInfo` trait for a type, if all of it's members implement `TypeInfo`
#[proc_macro_derive(TypeInfo)]
pub fn derive_typeinfo(input: TokenStream) -> TokenStream {
    let input: DeriveInput = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let layout_impl = match &input.data {
        Data::Struct(s) => impl_struct(name, s),
        Data::Enum(DataEnum { variants, .. }) => {
            let repr = match get_enum_repr(&input) {
                Ok(r) => r,
                Err(err) => return err.to_compile_error().into(),
            };

            impl_enum(name, repr, variants)
        }
        Data::Union(u) => impl_union(name, u),
    };

    quote! {const _: () = {
        use ::ffi_helper::{_TypeInfoImpl, types::{SVec, SStr, SOption}, layout::{EnumVariantType, EnumVariant, Layout, DefinedType, NamedField, FullLayout, DefinedTypes, TypeUid, TypeType}};
        use ::std::vec::Vec;
        unsafe impl #impl_generics _TypeInfoImpl for #name #ty_generics #where_clause {
            const _UID: TypeUid = TypeUid {
                rustpath: SStr::from_str({ #[allow(non_snake_case)] mod #name { pub const fn path() -> &'static str { ::std::module_path!() } } #name::path() }),
                file: SStr::from_str(::std::file!()),
                line: ::std::line!(),
                column: ::std::column!(),
            };

            fn _layout_impl(mut defined_types: DefinedTypes) -> FullLayout {
                match defined_types.iter().position(|t| t.0 == Self::_UID) {
                    Some(pos) => {
                        FullLayout {
                            layout: Layout::DefinedType { id: pos },
                            defined_types,
                        }
                    },
                    None => {
                        #layout_impl
                    }
                }
            }
        }
    };}
    .into()
}

/// Parses and returns the X in `#[repr(X)]`
fn get_enum_repr(input: &DeriveInput) -> syn::parse::Result<Ident> {
    for attr in &input.attrs {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "repr" {
                let parsed: TypeParen = syn::parse2(attr.tokens.clone())?;

                return match *parsed.elem {
                    syn::Type::Path(p) => match p.path.get_ident() {
                        Some(ident) => Ok(ident.clone()),
                        None => Err(syn::parse::Error::new(attr.span(), "invalid repr")),
                    },
                    _ => Err(syn::parse::Error::new(attr.span(), "invalid repr")),
                };
            }
        }
    }

    Err(syn::parse::Error::new(
        input.span(),
        "TypeInfo: #[repr(..)] attribute not found",
    ))
}
