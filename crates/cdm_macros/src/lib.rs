mod merge;
use merge::expand_merge;

// https://blog.turbo.fish/proc-macro-simple-derive/
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

// TODO: need to create derive functions for:
// - new()
// - Mergable / merge_prompt()
// - Empty / is_empty()
// - PartialEmpty / is_partial_empty()

#[proc_macro_derive(Merge)]
pub fn derive_merge(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    expand_merge(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
