use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(AnError)]
pub fn derive_anerror(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
        struct Test {}
    };

    TokenStream::from(expanded)
}
