use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

pub fn generate_parse_unsafe_cell(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        pub struct ParseUnsafeCell(std::cell::UnsafeCell<#struct_name>);

        impl ParseUnsafeCell {
            pub fn get(self) -> std::cell::UnsafeCell<#struct_name> {
                self.0
            }

            pub fn arc(self) -> std::sync::Arc<std::cell::UnsafeCell<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            pub fn boxed(self) -> Box<std::cell::UnsafeCell<#struct_name>> {
                Box::new(self.0)
            }
        }
    }
}
