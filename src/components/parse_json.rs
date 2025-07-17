use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_parse_json(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);
    quote! {
        pub struct ParseJson(serde_json::Value);

        impl ParseJson {
            /// Returns a reference to the internal `serde_json::Value`.
            pub fn get(&self) -> &serde_json::Value {
                &self.0
            }

            /// Consumes self and returns an `Arc` wrapped `serde_json::Value`
            /// for thread-safe shared ownership.
            pub fn arc(self) -> std::sync::Arc<serde_json::Value> {
                std::sync::Arc::new(self.0)
            }

            /// Consumes self and returns a Tokio async mutex wrapping the JSON value.
            pub fn tokio_mutex(self) -> tokio::sync::Mutex<serde_json::Value> {
                tokio::sync::Mutex::new(self.0)
            }

            /// Consumes self and returns a standard mutex wrapping the JSON value.
            pub fn mutex(self) -> std::sync::Mutex<serde_json::Value> {
                std::sync::Mutex::new(self.0)
            }

            /// Consumes self and returns a `RefCell` wrapping the JSON value,
            /// allowing interior mutability.
            pub fn ref_cell(self) -> std::cell::RefCell<serde_json::Value> {
                std::cell::RefCell::new(self.0)
            }

            /// Consumes self and returns an `UnsafeCell` wrapping the JSON value.
            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<serde_json::Value> {
                std::cell::UnsafeCell::new(self.0)
            }

            /// Consumes self and returns a `OnceCell` wrapping the JSON value,
            /// allowing one-time initialization.
            pub fn once_cell(self) -> std::cell::OnceCell<serde_json::Value> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }

            /// Consumes self and attempts to deserialize the JSON value into
            /// the original struct type. Returns a `Result` with the deserialized struct or an error.
            pub fn from(self) -> Result<#struct_name, serde_json::Error> {
                serde_json::from_value(self.0)
            }

            /// Attempts to deserialize a given `serde_json::Value` reference into
            /// the original struct type. Returns a `Result` with the deserialized struct or an error.
            pub fn from_value(&self, value: &serde_json::Value) -> Result<#struct_name, serde_json::Error> {
                serde_json::from_value(value.clone())
            }
        }
    }
}
