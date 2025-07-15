use mokuya::components::prelude::get_struct_name;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_parse_toml(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);
    quote! {
        pub struct ParseToml(toml::Value);

        impl ParseToml {
            pub fn get(&self) -> &toml::Value {
                &self.0
            }

            pub fn arc(self) -> std::sync::Arc<toml::Value> {
                std::sync::Arc::new(self.0)
            }

            pub fn tokio_mutex(self) -> tokio::sync::Mutex<toml::Value> {
                tokio::sync::Mutex::new(self.0)
            }

            pub fn mutex(self) -> std::sync::Mutex<toml::Value> {
                std::sync::Mutex::new(self.0)
            }

            pub fn ref_cell(self) -> std::cell::RefCell<toml::Value> {
                std::cell::RefCell::new(self.0)
            }

            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<toml::Value> {
                std::cell::UnsafeCell::new(self.0)
            }

            pub fn once_cell(self) -> std::cell::OnceCell<toml::Value> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }

            pub fn from(self) -> Result<#struct_name, toml::de::Error> {
                self.0.try_into()
            }

            pub fn from_value(&self, value: &toml::Value) -> Result<#struct_name, toml::de::Error> {
                value.clone().try_into()
            }
        }
    }
}
