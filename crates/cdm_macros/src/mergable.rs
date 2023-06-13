use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Fields, GenericArgument, Path, PathArguments, Type, TypePath,
};

pub fn expand_mergable(input: DeriveInput) -> TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let has_id = fields.iter().any(|f| {
        if let Some(fname) = f.ident.clone() {
            fname == "id"
        } else {
            false
        }
    });

    if !has_id {
        panic!("this derive macro only works on structs with a field called id");
    };

    let struct_name = input.ident;

    let mut eql_checks = Vec::new();
    for f in fields.iter() {
        // Interpolation only works for variables, not arbitrary expressions.
        // That's why we need to move these fields into local variables first
        //
        // @ is pattern binding
        let field_name = f.ident.clone();

        if let Some(fname) = &field_name {
            if fname == "id" {
                continue;
            }
        }

        match f.ty.clone() {
            // strings (only works for String, not std::string::String)
            Type::Path(TypePath { path, .. }) if path.is_ident("String") => {
                eql_checks.push(quote! {
                    if self.#field_name != other.#field_name {
                        input_map.insert(
                            stringify!(#field_name).to_string(),
                            [self.#field_name.clone(), other.#field_name.clone()],
                        );

                    }
                });
            }
            //vectors
            // Type::Path(TypePath {path, ..}) if path.
            // options (only works for Option, not std::option::Option)
            Type::Path(ty @ TypePath { .. }) => match option_inner_type(&ty.path) {
                Some(ty) => eql_checks.push(quote! {
                    if self.#field_name != other.#field_name {
                        input_map.insert(
                            stringify!(#field_name).to_string(),
                            [
                                {
                                    if let Some(#field_name) = self.#field_name.clone() {
                                        #field_name
                                    } else {
                                       #ty::default()
                                    }
                                },
                                {
                                    if let Some(#field_name) = other.#field_name.clone(){
                                        #field_name
                                    } else {
                                        #ty::default()
                                    }
                                }
                            ]
                        );
                    }
                }),
                None => eql_checks.push(quote! {
                    if self.#field_name != other.#field_name {
                        input_map.insert(
                            stringify!(#field_name).to_string(),
                            [self.#field_name.to_string(), other.#field_name.to_string()],
                        );
                    }
                }),
            },
            _ => eql_checks.push(quote! {
                if self.#field_name != other.#field_name {
                    input_map.insert(
                            stringify!(#field_name).to_string(),
                        [self.#field_name.to_string(), other.#field_name.to_string()],
                    );
                }
            }),
        }
    }

    let mut result_checks = Vec::new();
    for f in fields.iter() {
        let field_name = f.ident.clone();
        if let Some(fname) = &field_name {
            if fname == "id" {
                continue;
            }
        }
        match f.ty.clone() {
            // strings (only works for String, not std::string::String)
            Type::Path(TypePath { path, .. }) if path.is_ident("String") => {
                result_checks.push(quote! {
                    if results[stringify!(#field_name)] {
                            self.#field_name = other.#field_name.clone();
                    }
                })
            }
            // options (only works for Option, not std::option::Option)
            Type::Path(ty @ TypePath { .. }) => match option_inner_type(&ty.path) {
                Some(..) => result_checks.push(quote! {
                    if results[stringify!(#field_name)] {
                        self.#field_name =i other.#field_name.clone();
                    }
                }),
                None => result_checks.push(quote! {
                    if results[stringify!(#field_name)] {
                        self.#field_name = other.#field_name;
                    }
                }),
            },
            _ => result_checks.push(quote! {
                if results[stringify!(#field_name)] {
                    self.#field_name = other.#field_name;
                }
            }),
        }
    }

    quote! {
        #[automatically_derived]
        impl ::cdm_traits::Mergable for #struct_name {
            fn merge_prompt(&mut self, other: &Self, prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, bool>) {
                //TODO: maybe check for partial_empty/empty here on other
                let mut input_map: HashMap<String, [String; 2]> = HashMap::new();
                if self.id != other.id {
                    panic! {"attempting to merge structs with different IDs. This shouldn't have happened."}
                }
                #(#eql_checks)*
                let results = prompt_fn(input_map);
                // false means don't replace value in self struct
                #(#result_checks)*
            }

        }

    }
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
