use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};
/// the `Compare` function outputs a `Option<String>` representation of one type compared with
/// another. Mainly used for deriving on structs
pub fn expand_compare(input: DeriveInput) -> syn::Result<TokenStream> {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "this derive macro only works on structs with named fields",
            ));
        }
    };

    let struct_name = input.ident;

    let mut compares = Vec::new();

    for f in fields.iter() {
        let field_name = f.ident.clone();

        compares.push(quote! {
        let cmp = self.#field_name.compare();
        if cmp.is_some() {
            field_vars.push(::cdm_traits::compare::CompareResultField{
               field_name: stringify!(#field_name),
               self_value: self.#field_name.to_string(),
               other_value: other.#field_name.to_string(),
            });
        }

        });
    }

    let output = quote! {
        #[automatically_derived]
        impl ::cdm_traits::compare::Compare for #struct_name {
            fn compare(&self, other: &Self, compare_result: Option<::cdm_traits::compare::CompareResult>) -> Option<::cdm_traits::compare::CompareResult> {
                if self == other {
                    return None;
                } else {
                    let mut field_vars = Vec::new();

                    #(#compares)*
                    //TODO: this doesn't concatenate the structs results
                    Some(::cdm_traits::compare::CompareResult{
                        struct_name: Some(stringify!(#struct_name)),
                        base_result: None,
                        field_variations: Some(field_vars),
                    })
                }
            }
        }
    };
    Ok(output)
}
