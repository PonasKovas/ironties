use crate::impl_fields::{impl_named_fields, impl_unnamed_fields};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Ident, Variant};

pub fn impl_enum(name: &Ident, repr: Ident, variants: &Punctuated<Variant, Comma>) -> TokenStream {
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
                                ty: EnumVariantType::Struct(SVec::convert(fields)),
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
                                ty: EnumVariantType::Tuple(SVec::convert(fields)),
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

    quote! {
        defined_types.push((
            Self::_UID,
            DefinedType {
                name: SStr::from_str(stringify!(#name)),
                ty: TypeType::Enum {
                    // Temporary:
                    variants: SVec::convert(Vec::new()),
                    repr: SStr::from_str(stringify!(#repr)),
                },
            },
        ));
        let my_type_id = defined_types.len() - 1;

        let mut variants = Vec::new();

        #( #variants )*

        if let TypeType::Enum{ variants: ref mut v, ..} = defined_types[my_type_id].1.ty {
            *v = SVec::convert(variants);
        }

        FullLayout {
            layout: Layout::DefinedType { id: my_type_id },
            defined_types,
        }
    }
}
