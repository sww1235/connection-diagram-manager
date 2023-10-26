use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Fields, GenericArgument, Path, PathArguments, Type, TypePath,
};

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

    //let mut result_checks = Vec::new();
    //for f in fields.iter() {
    //    let field_name = f.ident.clone();
    //    if let Some(fname) = &field_name {
    //        if fname == "id" {
    //            continue;
    //        }
    //    }
    //    match f.ty.clone() {
    //        // strings (only works for String, not std::string::String)
    //        Type::Path(TypePath { path, .. }) if path.is_ident("String") => {
    //            result_checks.push(quote! {
    //                if results[stringify!(#field_name)] {
    //                        self.#field_name = other.#field_name.clone();
    //                }
    //            })
    //        }
    //        // options (only works for Option, not std::option::Option)
    //        Type::Path(ty @ TypePath { .. }) => match option_inner_type(&ty.path) {
    //            Some(..) => result_checks.push(quote! {
    //                if results[stringify!(#field_name)] {
    //                    self.#field_name =i other.#field_name.clone();
    //                }
    //            }),
    //            None => result_checks.push(quote! {
    //                if results[stringify!(#field_name)] {
    //                    self.#field_name = other.#field_name;
    //                }
    //            }),
    //        },
    //        _ => result_checks.push(quote! {
    //            if results[stringify!(#field_name)] {
    //                self.#field_name = other.#field_name;
    //            }
    //        }),
    //    }
    //}

    Ok(quote! {
        #[automatically_derived]
        impl ::cdm_traits::merge::Merge for #struct_name {
            fn merge_prompt(
                &mut self,
                other: &Self,
                prompt_fn: fn(::cdm_traits::compare::CompareResult),
                -> ::cdm_traits::compare::CompareResult)
                 {
                //TODO: maybe check for partial_empty/empty here on other
                if self.id != other.id {
                    return Err(syn::Error::new_spanned(
                    input,
                    "attempting to merge structs with different IDs. This shouldn't have happened.",
                    ));
                }
                let compare = self.compare(other, None);
                if let Some(compare) = compare {
                    let results = prompt_fn(compare);
                } else {
                    return None;
                }
                // false means don't replace value in self struct
                //#(#result_checks)*
            }

        }

    })
}

// only works on types with Option<>, not any other type of option.
// this should be ok, since it is only for my crate
fn option_inner_type(path: &Path) -> Option<&Type> {
    if path.leading_colon.is_some() {
        return None;
    }
    if path.segments.len() != 1 || path.segments[0].ident != "Option" {
        return None;
    }

    let ab = match &path.segments[0].arguments {
        PathArguments::AngleBracketed(ab) => ab,
        _ => return None,
    };
    if ab.args.len() != 1 {
        return None;
    }
    match &ab.args[0] {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}
