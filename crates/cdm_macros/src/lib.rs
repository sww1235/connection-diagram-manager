//! Procedural Macros for Connection Diagram Manager

/// merge derive
mod merge;
use merge::expand_merge;

/// partial empty derive
mod partial_empty;
use partial_empty::expand_partial_empty;

/// empty derive
mod empty;
use empty::expand_empty;

// https://blog.turbo.fish/proc-macro-simple-derive/
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

/// `derive_merge` is the proc macro function to process the `Merge` derive
#[proc_macro_derive(Merge)]
pub fn derive_merge(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    expand_merge(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// `derive_partial_empty` is the proc macro function to process the `PartialEmpty` derive
#[proc_macro_derive(PartialEmpty)]
pub fn derive_partial_empty(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    expand_partial_empty(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// `derive_empty` is the proc macro function to process the `Empty` derive
#[proc_macro_derive(Empty)]
pub fn derive_empty(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    expand_empty(input).into()
}
