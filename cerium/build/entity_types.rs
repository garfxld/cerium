use convert_case::{Case, Casing as _};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::write_file;

pub fn generate() {
    let entries: IndexMap<String, serde_json::Value> =
        serde_json::from_str(include_str!("../data/entity_type.json")).unwrap();

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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum EntityType {
            #enum_variants
        }
    };

    write_file(&out, "entity_types.rs");
}
