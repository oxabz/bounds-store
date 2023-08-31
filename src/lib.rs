/*!
# bounds-store

This crate provides a proc macro to store bounds for later use. It avoid repeating bounds in generic functions.

## Example
```rust, no_run
#![allow(dead_code, type_alias_bounds)]
use bounds_store::{bounds, bound_alias};

trait Float {
    fn foo();
}

type Point<F: Float> = (F, F);

trait Polygon<'a, F: Float>: 'a + IntoIterator<Item = &'a Point<F>> where F: 'a {
    fn bar(&self);
}

bounds! {
    Polygon => <'a, F: 'a + Float, P: Polygon<'a, F>>
}

#[bound_alias(Polygon)]
fn area(polygon: P) -> F {
    F::foo();
    polygon.bar();
    unimplemented!()
}
 */

use std::sync::OnceLock;
use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::ToTokens;

static STORE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

struct Bound{
    name: syn::Ident,
    _eq_token: syn::Token![=],
    _gt_token: syn::Token![>],
    generics: syn::Generics,
}

impl syn::parse::Parse for Bound {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let eq_token = input.parse()?;
        let gt_token = input.parse()?;
        let generics = input.parse()?;
        Ok(Bound {
            name,
            _eq_token:eq_token,
            _gt_token:gt_token,
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
        let names = bound.name.to_string();
        let generics = bound.generics;
        let generics = generics.into_token_stream();
        let generics = generics.to_string();        
        store.insert(&*names.leak(), &*generics.leak());
    }

    STORE.set(store).expect("Could not set store");
    TokenStream::new()
}

struct BoundAliasParams(Vec<(syn::Ident, Option<syn::Token!(,)>)>);

impl syn::parse::Parse for BoundAliasParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut params = Vec::new();
        while !input.is_empty() {
            let param = input.parse()?;
            let comma = input.parse()?;
            params.push((param, comma));
        }
        Ok(BoundAliasParams(params))
    }
}

#[proc_macro_attribute]
pub fn bound_alias(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item as syn::ItemFn);
    let params = syn::parse_macro_input!(attr as BoundAliasParams);
    
    let store = STORE.get().expect("bounds_alias! must be called before bounds attribute");

    let generics = &mut item.sig.generics;

    for (alias, _ ) in params.0{
        let name = alias.to_string();
        let bound_alias = store.get(&*name).expect(&format!("Could not find bound {}", name));
        let bound_alias: syn::Generics = syn::parse_str(bound_alias).expect("Could not parse bound");
        let alias_params = bound_alias.params;
        
        for param in alias_params {
            match param {
                syn::GenericParam::Lifetime(_) | syn::GenericParam::Const(_) => {
                    generics.params.push(param);
                },
                syn::GenericParam::Type(ty) => {
                    let existant = generics.params.iter_mut().find(|p| {
                        if let syn::GenericParam::Type(existant) = p {
                            existant.ident == ty.ident
                        } else {
                            false
                        }
                    });
                    if let Some(existant) = existant {
                        let existant = match existant {
                            syn::GenericParam::Type(existant) => existant,
                            _ => unreachable!(),
                        };
                        let mut existant_bounds = existant.bounds.clone();
                        existant_bounds.extend(ty.bounds);
                        existant.bounds = existant_bounds;
                    } else {
                        generics.params.push(syn::GenericParam::Type(ty));
                    }
                },
            }
        }
    }

    TokenStream::from(quote::quote!(#item))
}

#[cfg(test)]
mod test{
    use trybuild::TestCases;

    #[test]
    fn test(){
        let t = TestCases::new();
        t.pass("tests/polygon.rs");
        t.pass("tests/polygon_other_gen.rs");
        t.pass("tests/polygon_merge_bounds.rs");
        t.compile_fail("tests/bounds_dup.rs");
        t.compile_fail("tests/undefined_bound.rs");
    }
}