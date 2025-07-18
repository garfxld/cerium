extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, item: TokenStream) -> TokenStream {
    let value = parse_macro_input!(attr as LitStr);

    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_ident = &input_struct.ident;

    let expanded = quote! {
        #input_struct

        impl #struct_ident {
            pub const RESOURCE_ID: &'static str = #value;
        }

        impl crate::protocol::packet::Packet for #struct_ident {
        }

    };

    expanded.into()
}
