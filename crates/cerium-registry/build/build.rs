use std::{fs::File, io::Write as _, path::Path, process::Command};

use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn main() {
    // Dynamic Registries
    generate("Biome", "biomes.rs", include_str!("../data/biome.json"));
    generate(
        "DimensionType",
        "dimension_types.rs",
        include_str!("../data/dimension_type.json"),
    );
    generate(
        "CatVariant",
        "cat_variants.rs",
        include_str!("../data/cat_variant.json"),
    );
    generate(
        "ChickenVariant",
        "chicken_variants.rs",
        include_str!("../data/chicken_variant.json"),
    );
    generate(
        "CowVariant",
        "cow_variants.rs",
        include_str!("../data/cow_variant.json"),
    );
    generate(
        "DamageType",
        "damage_types.rs",
        include_str!("../data/damage_type.json"),
    );
    generate(
        "FrogVariant",
        "frog_variants.rs",
        include_str!("../data/frog_variant.json"),
    );
    generate(
        "PaintingVariant",
        "painting_variants.rs",
        include_str!("../data/painting_variant.json"),
    );
    generate(
        "PigVariant",
        "pig_variants.rs",
        include_str!("../data/pig_variant.json"),
    );
    generate(
        "WolfSoundVariant",
        "wolf_sound_variants.rs",
        include_str!("../data/wolf_sound_variant.json"),
    );
    generate(
        "WolfVariant",
        "wolf_variants.rs",
        include_str!("../data/wolf_variant.json"),
    );
}

pub fn write_file(content: TokenStream, dst: &str) {
    let path = Path::new("src/generated").join(dst);
    let content = content.to_string();

    let mut file = File::create(&path).unwrap();
    if let Err(e) = file.write_all(content.as_bytes()) {
        println!("cargo::error={e}");
    }

    let _ = Command::new("rustfmt").arg(&path).output();
}

pub(crate) fn generate(strct: &str, dst: &str, content: &str) {
    let entries: IndexMap<String, serde_json::Value> = serde_json::from_str(content).unwrap();

    let keys: TokenStream = entries
        .keys()
        .map(|key| {
            let ident = format_ident!(
                "{}",
                key.split_once(":")
                    .map_or(key.clone(), |v| v.1.to_owned())
                    .to_uppercase()
            );

            quote! {
                key!(#ident, #key);
            }
        })
        .collect();

    let strct = format_ident!("{}", strct);

    let out = quote! {
        #![allow(unused)]

        use std::sync::LazyLock;
        use crate::{RegistryKey, #strct};

        macro_rules! key {
            ($ident:ident, $key:expr) => {
                pub const $ident: LazyLock<RegistryKey<#strct>> = LazyLock::new(|| RegistryKey::of($key));
            };
        }

        impl #strct {
            #keys
        }

    };

    write_file(out, dst);
}
