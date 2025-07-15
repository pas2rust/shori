use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

pub fn generate_parse_tokio_mutex(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        pub struct ParseTokioMutex(tokio::sync::Mutex<#struct_name>);

        impl ParseTokioMutex {
            pub fn get(self) -> tokio::sync::Mutex<#struct_name> {
                self.0
            }

            pub fn arc(self) -> std::sync::Arc<tokio::sync::Mutex<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            pub fn boxed(self) -> Box<tokio::sync::Mutex<#struct_name>> {
                Box::new(self.0)
            }

            pub fn ref_cell(self) -> std::cell::RefCell<tokio::sync::Mutex<#struct_name>> {
                std::cell::RefCell::new(self.0)
            }

            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<tokio::sync::Mutex<#struct_name>> {
                std::cell::UnsafeCell::new(self.0)
            }

            pub fn once_cell(self) -> std::cell::OnceCell<tokio::sync::Mutex<#struct_name>> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }
        }
    }
}
