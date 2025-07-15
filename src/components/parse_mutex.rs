use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

pub fn generate_parse_mutex(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        pub struct ParseMutex(std::sync::Mutex<#struct_name>);

        impl ParseMutex {
            pub fn get(self) -> std::sync::Mutex<#struct_name> {
                self.0
            }

            pub fn arc(self) -> std::sync::Arc<std::sync::Mutex<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            pub fn boxed(self) -> Box<std::sync::Mutex<#struct_name>> {
                Box::new(self.0)
            }

            pub fn ref_cell(self) -> std::cell::RefCell<std::sync::Mutex<#struct_name>> {
                std::cell::RefCell::new(self.0)
            }

            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<std::sync::Mutex<#struct_name>> {
                std::cell::UnsafeCell::new(self.0)
            }

            pub fn once_cell(self) -> std::cell::OnceCell<std::sync::Mutex<#struct_name>> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }
        }
    }
}
