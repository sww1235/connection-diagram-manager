use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

/// `expand_partial_empty` is the actual logic of deriving the `PartialEmpty` trait
#[allow(clippy::module_name_repetitions)]
pub fn expand_partial_empty(input: DeriveInput) -> syn::Result<TokenStream> {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => &fields.named,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "this derive macro only works on structs with named fields",
            ));
        }
    };

    let has_id = fields.iter().any(|f| {
        if let Some(fname) = f.ident.clone() {
            fname == "id"
        } else {
            false
        }
    });

    if !has_id {
        return Err(syn::Error::new_spanned(
            input,
            "this derive macro only works on structs with a field called id",
        ));
    };

    let struct_name = input.ident;

    let mut equality_checks = Vec::new();
    for f in fields {
        let field_name = f.ident.clone();

        // don't want to compare these fields in partial_empty function
        if let Some(fname) = &field_name {
            if fname == "id" {
                continue;
            }
            if fname == "contained_datafile_path" {
                continue;
            }
        }
        if equality_checks.is_empty() {
            equality_checks.push(quote! {
                self.#field_name == tester.#field_name
            });
        } else {
            equality_checks.push(quote! {
               && self.#field_name == tester.#field_name
            });
        }
    }

    Ok(quote! {
        #[automatically_derived]
        impl ::cdm_traits::partial_empty::PartialEmpty for #struct_name {
            fn is_partial_empty(&self) -> bool {
                let tester = Self::new();
                #(#equality_checks)*
            }

        }

    })
}
