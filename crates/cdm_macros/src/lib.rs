mod mergable;
use mergable::expand_mergable;

// https://blog.turbo.fish/proc-macro-simple-derive/
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

// TODO: need to create derive functions for:
// - new()
// - Mergable / merge_prompt()
// - Empty / is_empty()
// - PartialEmpty / is_partial_empty()

#[proc_macro_derive(Mergable)]
pub fn derive_mergable(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    expand_mergable(input).into()
}
