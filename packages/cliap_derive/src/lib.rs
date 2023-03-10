use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Arguments)]
pub fn arguments(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // trait cliap::Arguments for #pe {
        //     fn arguments() -> Vec<cliap::Arguments>{
        //         // for each key pair,
        //     }
        // }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
