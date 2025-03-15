use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn full_ctor(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;

    let data = match input.data {
        Data::Struct(data) => data,
        _ => abort_call_site!("FullCtor can only be used on structs"),
    };

    let field_names: Vec<_> = data.fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = data.fields.iter().map(|f| &f.ty).collect();

    let expanded = quote! {
        impl #struct_name {
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    expanded
}
