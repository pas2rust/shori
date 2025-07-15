use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_parse_vec(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        pub struct ParseVec(Vec<#struct_name>);

        impl ParseVec {
            pub fn get(self) -> Vec<#struct_name> {
                self.0
            }

            pub fn arc(self) -> std::sync::Arc<Vec<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            pub fn boxed(self) -> Box<Vec<#struct_name>> {
                Box::new(self.0)
            }

            pub fn ref_cell(self) -> std::cell::RefCell<Vec<#struct_name>> {
                std::cell::RefCell::new(self.0)
            }

            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<Vec<#struct_name>> {
                std::cell::UnsafeCell::new(self.0)
            }

            pub fn once_cell(self) -> std::cell::OnceCell<Vec<#struct_name>> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }
        }
    }
}
