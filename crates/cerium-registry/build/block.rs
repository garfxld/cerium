use convert_case::{Case, Casing as _};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::write_file;

pub fn generate() {
    let entries: IndexMap<String, serde_json::Value> =
        serde_json::from_str(include_str!("../data/block.json")).unwrap();

    let variants: Vec<_> = entries
        .keys()
        .enumerate()
        .map(|(index, key)| {
            let ident = format_ident!(
                "{}",
                key.split_once(":")
                    .map_or(key.clone(), |v| v.1.to_owned())
                    .to_case(Case::UpperCamel)
            );
            (index as i32, ident, key.clone())
        })
        .collect();

    let enum_variants: TokenStream = variants
        .iter()
        .map(|(_, ident, _)| {
            quote! {
                #ident,
            }
        })
        .collect();

    let out = quote! {
        #![allow(unused)]
        use std::ops::Deref;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum Block {
            #enum_variants
        }

        impl Block {
            pub fn from_state(id: i32) -> Option<&'static crate::block::BlockState> {
                crate::block::REGISTRY.0.get(&id)
            }
        }

        impl Deref for Block {
            type Target = crate::block::BlockState;

            fn deref(&self) -> &<Block as Deref>::Target {
                let state_id = *crate::block::REGISTRY.1.get(*self as usize).unwrap();
                crate::block::REGISTRY.0.get(&state_id).unwrap()
            }
        }
    };

    write_file(out, "block.rs");
}
