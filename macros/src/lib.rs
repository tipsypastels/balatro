use proc_macro::TokenStream;
use syn::{ItemEnum, parse_macro_input};

mod variants;

#[proc_macro_derive(Variants)]
pub fn variants(input: TokenStream) -> TokenStream {
    variants::variants(parse_macro_input!(input as ItemEnum))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
