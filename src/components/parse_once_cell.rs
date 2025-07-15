use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

pub fn generate_parse_once_cell(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        pub struct ParseOnceCell(std::cell::OnceCell<#struct_name>);

        impl ParseOnceCell {
            pub fn get(&self) -> Option<&#struct_name> {
                self.0.get()
            }

            pub fn arc(self) -> std::sync::Arc<std::cell::OnceCell<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            pub fn boxed(self) -> Box<std::cell::OnceCell<#struct_name>> {
                Box::new(self.0)
            }
        }
    }
}
