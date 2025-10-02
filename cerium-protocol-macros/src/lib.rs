use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemStruct, LitInt, LitStr, Result, Token, parse::Parse, parse::ParseStream, parse_macro_input,
};

struct PacketArgs {
    name: LitStr,
    id: LitInt,
}

impl Parse for PacketArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let id: LitInt = input.parse()?;

        Ok(PacketArgs { name, id })
    }
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as PacketArgs);
    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_ident = &input_struct.ident;

    let packet_name = &args.name;
    let packet_id = &args.id;

    let expanded = quote! {
        #input_struct

        impl crate::protocol::packet::Packet for #struct_ident {
            const ID: i32 = #packet_id;
            const RESOURCE_ID: &'static str = #packet_name;
        }
    };

    expanded.into()
}
