use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Data, DeriveInput, Field};

/// Automatically derives the `TypeInfo` trait for a type, if all of it's members implement `TypeInfo`
#[proc_macro_derive(TypeInfo)]
pub fn derive_typeinfo(input: TokenStream) -> TokenStream {
    let input: DeriveInput = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match &input.data {
        Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(fields) => {
                    let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
                    let field_types = fields.named.iter().map(|f| &f.ty);

                    quote! {const _: () = {
                        use ::ffi_helper::{_TypeInfoImpl, types::{SVec, SStr}, layout::{Layout, DefinedType, NamedField, FullLayout, DefinedTypes, TypeUid, TypeType, Lifetime}};
                        use ::std::vec::Vec;
                        unsafe impl #impl_generics _TypeInfoImpl for #name #ty_generics #where_clause {
                            const _UID: TypeUid = TypeUid {
                                rustpath: SStr::from_str({ #[allow(non_snake_case)] mod #name { pub const fn path() -> &'static str { ::std::module_path!() } } #name::path() }),
                                file: SStr::from_str(::std::file!()),
                                line: ::std::line!(),
                                column: ::std::column!(),
                            };

                            fn _layout_impl(mut defined_types: DefinedTypes, lifetimes: Vec<Lifetime>) -> FullLayout {
                                match defined_types.iter().position(|t| t.0 == Self::_UID) {
                                    Some(pos) => {
                                        FullLayout {
                                            layout: Layout::DefinedType(pos),
                                            defined_types,
                                            lifetimes,
                                        }
                                    },
                                    None => {
                                        defined_types.push((
                                            Self::_UID,
                                            DefinedType {
                                                name: SStr::from_str(stringify!(#name)),
                                                // Temporary:
                                                ty: TypeType::StructUnit,
                                            },
                                        ));
                                        let my_type_id = defined_types.len() - 1;

                                        let mut fields = Vec::new();

                                        #(
                                            let FullLayout { layout, mut defined_types, mut lifetimes } = <#field_types as _TypeInfoImpl>::_layout_impl(defined_types, lifetimes);
                                            fields.push(NamedField {
                                                name: SStr::from_str(stringify!(#field_names)),
                                                layout,
                                            });
                                        )*

                                        defined_types[my_type_id].1.ty = TypeType::StructNamed {
                                            fields: SVec::from_std(fields),
                                        };

                                        FullLayout {
                                            layout: Layout::DefinedType(my_type_id),
                                            defined_types,
                                            lifetimes,
                                        }
                                    }
                                }
                            }
                        }
                    };}
                    .into()
                }
                syn::Fields::Unnamed(fields) => {
                    let field_types = fields.unnamed.iter().map(|f| &f.ty);

                    quote! {
                        unsafe impl #impl_generics ::ffi_helper::CType for #name #ty_generics #where_clause {
                            const LAYOUT: &'static ::ffi_helper::Layout = &::ffi_helper::Layout::StructUnnamed{
                                name: ::ffi_helper::types::SStr::from_str(stringify!(#name)),
                                fields: ::ffi_helper::types::SSlice::from_slice(&[
                                    #( <#field_types as ::ffi_helper::CType>::LAYOUT ),*
                                ]),
                            };
                        }
                    }
                    .into()
                }
                syn::Fields::Unit => {
                    quote! {
                        unsafe impl #impl_generics ::ffi_helper::CType for #name #ty_generics #where_clause {
                            const LAYOUT: &'static ::ffi_helper::Layout = &::ffi_helper::Layout::StructUnit{
                                name: ::ffi_helper::types::SStr::from_str(stringify!(#name)),
                            };
                        }
                    }
                    .into()
                },
            }
        }
        Data::Enum(_e) => todo!(),
        Data::Union(_u) => todo!(),
    }
}
