use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

pub(crate) fn field_names(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;

    let fields = if let Data::Struct(data) = input.data {
        data.fields
    } else {
        panic!("FieldNames can only be used on structs");
    };

    let nameof_getters = fields.iter().filter_map(|field| {
        if let Some(ident) = &field.ident {
            let field_name = ident.to_string();
            let method_name = Ident::new(&format!("nameof_{}", field_name), ident.span());
            Some(quote! {
                pub fn #method_name() -> &'static str {
                    #field_name
                }
            })
        } else {
            None
        }
    });

    let expanded = quote! {
        impl #struct_name {
            #(#nameof_getters)*
        }
    };

    expanded
}
