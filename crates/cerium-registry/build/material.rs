use convert_case::{Case, Casing};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::write_file;

pub fn generate() {
    let entries: IndexMap<String, serde_json::Value> =
        serde_json::from_str(include_str!("../data/item.json")).unwrap();

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

    let from_id_arms: TokenStream = variants
        .iter()
        .map(|(index, ident, _)| {
            let index: TokenStream = index.to_string().parse().unwrap();
            quote! {
                #index => Some(Material::#ident),
            }
        })
        .collect();

    let out = quote! {
        #![allow(unused)]

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum Material {
            #enum_variants
        }

        impl Material {
            pub fn from_id(id: i32) -> Option<Material> {
                match id {
                    #from_id_arms
                    _ => None,
                }
            }
        }

    };

    write_file(out, "material.rs");
}
