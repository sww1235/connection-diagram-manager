use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

pub fn expand_merge(input: DeriveInput) -> syn::Result<TokenStream> {
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

    // First check for equality between each field, and
    let mut equality_checks = Vec::new();
    for f in fields.iter() {
        let field_name = f.ident.clone();
        if let Some(fname) = &field_name {
            if fname == "id" {
                continue;
            }
        }
        // use colons to set field values dummy
        equality_checks.push(quote! {
            let field = ::cdm_traits::merge::StructField {
                name: {stringify!(#field_name).to_string()},
                // use debug formatting for ease
                self_string: {format!{"{:#?}", self.#field_name}},
                other_string: {format!{"{:#?}", other.#field_name}},
                equality: {self.#field_name == other.#field_name},
                use_other: false,
            };
            equality_results.fields.push(field);
        });
    }
    // then use results from merge to process
    let mut merge_ops = Vec::new();
    for f in fields.iter() {
        let field_name = f.ident.clone();
        merge_ops.push(quote! {
            if val.name == stringify!(#field_name).to_string() && val.use_other {
               self.#field_name = other.#field_name.clone();
            }
        });
    }

    Ok(quote! {
        #[automatically_derived]
        impl ::cdm_traits::merge::Merge for #struct_name {
            fn merge_prompt(
                &mut self,
                other: &Self,
                prompt_fn: fn(::cdm_traits::merge::ComparedStruct)
                -> ::cdm_traits::merge::ComparedStruct
            ){
                //TODO: maybe check for partial_empty/empty here on other
                if self.id != other.id {
                    //return Err(::cdm_errors::(
                    //input,
                    //"attempting to merge structs with different IDs. This shouldn't have happened.",
                    //));
                    //TODO: return an error within scope
                    todo!();
                }
                let mut equality_results = ::cdm_traits::merge::ComparedStruct::new();
                equality_results.struct_name = stringify!(#struct_name).to_string();
                #(#equality_checks)*
                let prompt_results = prompt_fn(equality_results);
                for val in prompt_results.fields{
                    #(#merge_ops)*
                }
            }

        }

    })
}
