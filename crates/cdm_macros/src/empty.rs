use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// `expand_empty` is the actual logic of deriving the `Empty` trait
#[allow(clippy::module_name_repetitions)]
pub fn expand_empty(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;

    quote! {
        #[automatically_derived]
        impl ::cdm_traits::empty::Empty for #struct_name {
            fn is_empty(&self) -> bool {
                self == &Self::new()
            }

        }

    }
}
