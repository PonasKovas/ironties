use proc_macro2::TokenStream;
use quote::quote;
use syn::{FieldsNamed, FieldsUnnamed};

pub fn impl_named_fields(fields: &FieldsNamed) -> TokenStream {
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

pub fn impl_unnamed_fields(fields: &FieldsUnnamed) -> TokenStream {
    let field_types = fields.unnamed.iter().map(|f| &f.ty);

    quote! {
        #(
            let FullLayout { layout, mut defined_types } = <#field_types as _TypeInfoImpl>::_layout_impl(defined_types);
            fields.push(layout);
        )*
    }
}
