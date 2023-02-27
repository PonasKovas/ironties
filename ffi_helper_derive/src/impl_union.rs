use crate::impl_fields::impl_named_fields;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataUnion, Ident};

pub fn impl_union(name: &Ident, u: &DataUnion) -> TokenStream {
    let fields = impl_named_fields(&u.fields);

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

        defined_types[my_type_id].1.ty = TypeType::Union {
            fields: SVec::from_std(fields),
        };

        FullLayout {
            layout: Layout::DefinedType { id: my_type_id },
            defined_types,
        }
    }
}
