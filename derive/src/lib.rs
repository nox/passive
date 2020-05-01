// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Passive (derive)
//!
//! Proc macro crate for the [`passive`](https://docs.rs/passive) traits.

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use synstructure::Structure;

macro_rules! derive_traits {
    ($($proc_name:ident: $trait_name:ident,)*) => {$(
        fn $proc_name(s: Structure) -> TokenStream {
            derive_trait(s, quote!(passive::$trait_name))
        }
        synstructure::decl_derive!([$trait_name] => $proc_name);
    )*}
}

derive_traits! {
    always_aligned: AlwaysAligned,
    always_valid: AlwaysValid,
    immutable: Immutable,
}

fn derive_trait(s: Structure, trait_path: TokenStream) -> TokenStream {
    let impl_ = s.unsafe_bound_impl(&trait_path, quote!());

    let soundness_check = soundness_check(&s, trait_path);

    quote! { #impl_ #soundness_check }
}

fn soundness_check(s: &Structure, trait_path: TokenStream) -> TokenStream {
    let (impl_generics, _, where_clause) = s.ast().generics.split_for_impl();

    let check = s
        .variants()
        .iter()
        .flat_map(|v| v.bindings())
        .map(|b| {
            let ty = &b.ast().ty;
            let span = ty.span();
            quote_spanned! { span => is::<#ty>() }
        });

    quote! {
        const _: () = {
            fn is<T: #trait_path>() {}
            fn checks #impl_generics () #where_clause {
                #(#check;)*
            }
        };
    }
}
