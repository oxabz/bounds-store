use std::sync::OnceLock;
use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Token, TypeParamBound};

static STORE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

struct Bound{
    names: syn::punctuated::Punctuated<syn::Ident, syn::Token!(,)>,
    eq_token: syn::Token![=],
    gt_token: syn::Token![>],
    generics: syn::Generics,
}

impl syn::parse::Parse for Bound {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let names = input.parse()?;
        let eq_token = input.parse()?;
        let gt_token = input.parse()?;
        let generics = input.parse()?;
        Ok(Bound {
            names,
            eq_token,
            gt_token,
            generics,
        })
    }
}

struct Bounds(Vec<(Bound, Option<syn::token::Comma>)>);

impl syn::parse::Parse for Bounds {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut bounds = Vec::new();
        while !input.is_empty() {
            let bound = input.parse()?;
            let comma = input.parse()?;
            bounds.push((bound, comma));
        }
        Ok(Bounds(bounds))
    }
}

#[proc_macro]
pub fn bounds(input: TokenStream) -> TokenStream {
    let bounds = syn::parse_macro_input!(input as Bounds);
    if let Some(_) = STORE.get() {
        panic!("bounds! can only be called once");
    }
    let mut store = HashMap::new();
    for (bound, _) in bounds.0 {
        let names = bound.names.to_token_stream().to_string();
        let generics = bound.generics;
        let generics = generics.into_token_stream();
        let generics = generics.to_string();        
        store.insert(&*names.leak(), &*generics.leak());
    }

    STORE.set(store).expect("Could not set store");
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn bound(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item as syn::ItemFn);
    let store = STORE.get().expect("bounds! must be called before bounds attribute");

    let mut params = &item.sig.generics.params;
    let stored_bounds = params.iter().filter_map(|param| {
        let param = if let syn::GenericParam::Type(param) = param {
            param
        } else {
            return None
        };
        if param.bounds.len() != 1 {
            return None;
        }

        let bound = &param.bounds[0];
        if let TypeParamBound::Verbatim(t) = bound{
            if t.to_string() != "_" {
                return None;
            }
        }

        let name = param.ident.to_string();
        Some(name)
    }).collect::<Vec<_>>();
    
    TokenStream::from(quote::quote!(#item))
}