use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(AnError)]
pub fn derive_anerror(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let variant_from_impls = match &input.data {
        syn::Data::Enum(data_enum) => data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            let inner_type = match &variant.fields {
                syn::Fields::Unnamed(fields_unnamed) => match fields_unnamed.unnamed.first() {
                    Some(field) => &field.ty,
                    None => todo!(),
                },
                _ => todo!(),
            };

            quote! {
                impl From<#inner_type> for #name {
                    fn from(value: #inner_type) -> Self {
                        Self::#variant_name(value)
                    }
                }
            }
        }),
        _ => todo!(),
    };

    let expanded = quote! {
        #(#variant_from_impls)*
    };

    TokenStream::from(expanded)
}
