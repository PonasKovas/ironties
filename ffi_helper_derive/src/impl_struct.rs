use crate::impl_fields::{impl_named_fields, impl_unnamed_fields};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Ident};

pub fn impl_struct(name: &Ident, s: &DataStruct) -> TokenStream {
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
