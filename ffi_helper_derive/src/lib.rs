use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, LitStr};

#[proc_macro_derive(CType)]
pub fn derive_ctype(input: TokenStream) -> TokenStream {
    let input: DeriveInput = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let c_name = LitStr::new(&format!("{}", input.ident), Span::call_site());

    match &input.data {
        Data::Struct(s) => {
            if s.fields.is_empty() {
                return syn::Error::new(
                    input.span(),
                    "CType: zero-sized types are not C compatible",
                )
                .to_compile_error()
                .into();
            }

            let (fields, field_names) = match &s.fields {
                syn::Fields::Named(fields) => {
                    let field_names: Vec<_> = fields
                        .named
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap().clone())
                        .collect();

                    (&fields.named, field_names)
                }
                syn::Fields::Unnamed(fields) => {
                    let field_names: Vec<_> = (1..=fields.unnamed.len())
                        .map(|field_id| Ident::new(&format!("m{field_id}"), Span::call_site()))
                        .collect();

                    (&fields.unnamed, field_names)
                }
                syn::Fields::Unit => unreachable!("structs with no fields already denied"),
            };

            let field_types = fields.iter().map(|f| &f.ty);

            let cfield_types = fields.iter().map(|f| {
                let ty = &f.ty;
                quote! {& <#ty as ::ffi_helper::CType>::_prefix()}
            });

            quote! {
                unsafe impl #impl_generics ::ffi_helper::CType for #name #ty_generics #where_clause {
                    fn _prefix() -> String {
                        format!("struct {}", #c_name)
                    }
                    fn _definitions(defs: &mut ::ffi_helper::cdefinitions::CDefinitions) {
                        let mut struct_def = format!("typedef struct {} {{\n", #c_name);

                        #({
                            let field_type = #cfield_types;
                            // Completely skip 'void' fields
                            if field_type != "void" {
                                struct_def = struct_def + "\t" + field_type + " " + stringify!(#field_names) + ";\n";
                            }
                        })*

                        struct_def = struct_def + "} " + #c_name + ";";

                        defs.types.insert(::std::any::type_name::<Self>().to_owned(), struct_def);
                        #( defs.extend_once::<#field_types>(); )*
                    }
                }
            }
            .into()
        }
        Data::Enum(_e) => todo!(),
        Data::Union(_u) => todo!(),
    }
}
