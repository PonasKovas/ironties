use proc_macro::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DataEnum, DataStruct,
    DeriveInput, FieldsNamed, FieldsUnnamed, Ident, TypeParen, Variant,
};

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
        Data::Union(_u) => todo!("union not implemented yet"),
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

fn get_enum_repr(input: &DeriveInput) -> syn::parse::Result<Option<Ident>> {
    for attr in &input.attrs {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "repr" {
                let parsed: TypeParen = syn::parse2(attr.tokens.clone())?;

                return match *parsed.elem {
                    syn::Type::Path(p) => match p.path.get_ident() {
                        Some(ident) => Ok(Some(ident.clone())),
                        None => Err(syn::parse::Error::new(attr.span(), "invalid repr")),
                    },
                    _ => Err(syn::parse::Error::new(attr.span(), "invalid repr")),
                };
            }
        }
    }

    Ok(None)
}

fn impl_enum(
    name: &Ident,
    repr: Option<Ident>,
    variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let variants = {
        let discriminants = variants.iter().scan(quote! { 0 }, |d, v| {
            *d = match &v.discriminant {
                Some((_, d)) => quote! { #d },
                None => quote! { #d },
            };
            let current_d = d.clone();
            *d = quote! {( #d + 1 )};

            Some(current_d)
        });

        variants
            .iter()
            .zip(discriminants)
            .map(|(variant, discriminant)| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    syn::Fields::Named(fields) => {
                        let fields = impl_named_fields(fields);

                        quote! {
                            let mut fields = Vec::new();

                            #fields

                            variants.push(EnumVariant {
                                name: SStr::from_str(stringify!(#variant_name)),
                                ty: EnumVariantType::Struct(SVec::from_std(fields)),
                                discriminant: #discriminant,
                            });
                        }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let fields = impl_unnamed_fields(fields);

                        quote! {
                            let mut fields = Vec::new();

                            #fields

                            variants.push(EnumVariant {
                                name: SStr::from_str(stringify!(#variant_name)),
                                ty: EnumVariantType::Tuple(SVec::from_std(fields)),
                                discriminant: #discriminant,
                            });
                        }
                    }
                    syn::Fields::Unit => quote! {
                        variants.push(EnumVariant {
                            name: SStr::from_str(stringify!(#variant_name)),
                            ty: EnumVariantType::Unit,
                            discriminant: #discriminant,
                        });
                    },
                }
            })
    };

    let repr = match repr {
        Some(r) => quote! { SOption::Some(SStr::from_str(stringify!(#r))) },
        None => quote! { SOption::None },
    };

    quote! {
        defined_types.push((
            Self::_UID,
            DefinedType {
                name: SStr::from_str(stringify!(#name)),
                ty: TypeType::Enum {
                    // Temporary:
                    variants: SVec::from_std(Vec::new()),
                    repr: #repr,
                },
            },
        ));
        let my_type_id = defined_types.len() - 1;

        let mut variants = Vec::new();

        #( #variants )*

        if let TypeType::Enum{ variants: ref mut v, ..} = defined_types[my_type_id].1.ty {
            *v = SVec::from_std(variants);
        }

        FullLayout {
            layout: Layout::DefinedType { id: my_type_id },
            defined_types,
        }
    }
}

fn impl_struct(name: &Ident, s: &DataStruct) -> proc_macro2::TokenStream {
    match &s.fields {
        syn::Fields::Named(fields) => {
            let fields = impl_named_fields(fields);

            quote! {
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

                #fields

                defined_types[my_type_id].1.ty = TypeType::StructNamed {
                    fields: SVec::from_std(fields),
                };

                FullLayout {
                    layout: Layout::DefinedType { id: my_type_id },
                    defined_types,
                }
            }
        }
        syn::Fields::Unnamed(fields) => {
            let fields = impl_unnamed_fields(fields);

            quote! {
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

                #fields

                defined_types[my_type_id].1.ty = TypeType::StructUnnamed {
                    fields: SVec::from_std(fields),
                };

                FullLayout {
                    layout: Layout::DefinedType { id: my_type_id },
                    defined_types,
                }
            }
        }
        syn::Fields::Unit => {
            quote! {
                defined_types.push((
                    Self::_UID,
                    DefinedType {
                        name: SStr::from_str(stringify!(#name)),
                        ty: TypeType::StructUnit,
                    },
                ));
                let my_type_id = defined_types.len() - 1;

                FullLayout {
                    layout: Layout::DefinedType { id: my_type_id },
                    defined_types,
                }
            }
        }
    }
}

fn impl_named_fields(fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
    let field_types = fields.named.iter().map(|f| &f.ty);

    quote! {
        #(
            let FullLayout { layout, mut defined_types } = <#field_types as _TypeInfoImpl>::_layout_impl(defined_types);
            fields.push(NamedField {
                name: SStr::from_str(stringify!(#field_names)),
                layout,
            });
        )*
    }
}

fn impl_unnamed_fields(fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let field_types = fields.unnamed.iter().map(|f| &f.ty);

    quote! {
        #(
            let FullLayout { layout, mut defined_types } = <#field_types as _TypeInfoImpl>::_layout_impl(defined_types);
            fields.push(layout);
        )*
    }
}
