extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Ident, Variant};

#[derive(Clone)]
struct EnvOptVariant {
    ident: Ident,
    name: String,
    default: Option<String>,
}

#[proc_macro_derive(EnvOpt, attributes(envopt))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = ast.ident;

    if let Data::Enum(DataEnum { variants, .. }) = ast.data {
        let envopt_variants: Vec<_> = variants
            .into_iter()
            .map(|variant| {
                let ident = variant.ident.clone();
                let name = get_name_attr(&variant).expect("No name found");
                let default = get_default_attr(&variant);
                EnvOptVariant {
                    ident,
                    name,
                    default,
                }
            })
            .collect();

        let validate_matches: Vec<_> = envopt_variants
            .clone()
            .into_iter()
            .filter_map(|variant| {
                let name = variant.name;
                if variant.default.is_none() {
                    Some(quote! {
                        match std::env::var(#name) {
                            Ok(_) => {}
                            Err(_) => {
                                errors.push(concat!("Environment variable ", stringify!(#name), " not set").into());
                            }
                        }
                    })
                } else {
                    None
                }
            })
            .collect();

        let value_matches: Vec<_> = envopt_variants
            .into_iter()
            .map(|variant| {
                let ident = variant.ident;
                let name = variant.name;

                match variant.default {
                    Some(default) => quote! {
                        #enum_name::#ident => match std::env::var(#name) {
                            Ok(val) => Ok(val),
                            Err(_) => Ok(#default.into()),
                        }
                    },
                    None => quote! {
                        #enum_name::#ident => match std::env::var(#name) {
                            Ok(val) => Ok(val),
                            Err(_) => Err(concat!("Environment variable ", stringify!(#name), " not set").into()),
                        }
                    },
                }
            })
            .collect();

        let impl_env_opt = quote! {
            impl envopt::EnvOpt for #enum_name {
                fn validate() -> Result<(), Vec<String>> {
                    let mut errors = Vec::new();
                    #(#validate_matches)*

                    if errors.is_empty() {
                        Ok(())
                    } else {
                        Err(errors)
                    }
                }

                fn value(&self) -> Result<String, String> {
                    match self {
                        #(#value_matches),*
                    }
                }
            }
        };

        TokenStream::from(impl_env_opt)
    } else {
        TokenStream::new()
    }
}

fn get_attr(name: String, variant: &Variant) -> Option<String> {
    for attr in variant.attrs.clone() {
        if attr.path.segments.first().unwrap().ident == "envopt" {
            for token in attr.tokens {
                if let TokenTree::Group(group) = token.clone() {
                    let mut token_stream_iter = group.stream().into_iter();
                    while let Some(x) = token_stream_iter.next() {
                        if let TokenTree::Ident(i) = x {
                            if &i == &Ident::new(&name, i.span().clone()) {
                                token_stream_iter.next();
                                let mut s = token_stream_iter.next().unwrap().to_string();
                                // FIXME this gets rid of the escaped quots.  There must be a more
                                // idiomatic way...
                                s.remove(0);
                                s.pop();
                                return Some(s);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn get_name_attr(variant: &Variant) -> Option<String> {
    dbg!(get_attr("name".into(), variant))
}

fn get_default_attr(variant: &Variant) -> Option<String> {
    dbg!(get_attr("default".into(), variant))
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test() {}
}
