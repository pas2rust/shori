use mokuya::components::prelude::get_struct_name;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use zipher::components::aes_gcm_siv::*;

pub fn generate_parse_aes_gcm_siv(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);
    quote! {
        pub struct ParseAesGcmSiv(String);;

        impl ParseAesGcmSiv {
            pub fn get(&self) -> String {
                &self.0
            }

            pub fn from(&self) -> #struct_name {
                self.decrypt()
            }
        }
    }
}
