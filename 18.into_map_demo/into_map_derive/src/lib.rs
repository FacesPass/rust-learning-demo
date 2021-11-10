extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(IntoMap)]
pub fn into_map_derive(input: TokenStream) -> TokenStream {
    let mut insert_tokens = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    match parsed_input.data {
        Data::Struct(s) => {
            if let Fields::Named(named_fields) = s.fields {
                let a = named_fields.named;
                for i in a {
                    let field = i.ident.unwrap();
                    let insert_token = quote! {
                        map.insert (
                            stringify!(#field).to_string(),
                            self.#field.to_string()
                        );
                    };
                    insert_tokens.push(insert_token);
                }
            }
        }
        other => panic!("IntoMap is not yet implemented for: {:?}", other),
    }

    let tokens = quote! {
     use std::collections::BTreeMap;
     use into_map::IntoMap;
     impl IntoMap for #struct_name {
     // 将给定结构体转换为动态映射
    fn into_map(&self) -> BTreeMap<String, String> {
       let mut map = BTreeMap::new();
       #(#insert_tokens)*
       map
    }}};
    proc_macro::TokenStream::from(tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
