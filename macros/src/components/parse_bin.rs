use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_parse_bin(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);
    quote! {
        #[derive(Debug)]
        pub struct ParseBin(Vec<u8>);
        use std::error::Error;
        impl ParseBin {
            /// Returns a reference to the inner byte slice.
            pub fn get(&self) -> &[u8] {
                &self.0
            }

            /// Returns a hexadecimal string representation of the bytes.
            pub fn hex(&self) -> String {
                hex::encode(&self.0)
            }

            /// Attempts to decode an instance of the struct from a hex string.
            ///
            /// # Errors
            ///
            /// Returns an error if the hex string is invalid or if decoding fails.
            pub fn from_hex(&self, hex_str: &str) -> Result<#struct_name, Box<dyn std::error::Error>> {
                let bytes = hex::decode(hex_str)?;
                bincode::decode_from_slice(&bytes, bincode::config::standard())
                    .map(|(result, _)| result)
                    .map_err(Into::into)
            }

            /// Converts the inner bytes into an `Arc<Vec<u8>>`.
            pub fn arc(self) -> std::sync::Arc<Vec<u8>> {
                std::sync::Arc::new(self.0)
            }

            /// Wraps the inner bytes in a Tokio `Mutex`.
            pub fn tokio_mutex(self) -> tokio::sync::Mutex<Vec<u8>> {
                tokio::sync::Mutex::new(self.0)
            }

            /// Wraps the inner bytes in a standard `Mutex`.
            pub fn mutex(self) -> std::sync::Mutex<Vec<u8>> {
                std::sync::Mutex::new(self.0)
            }

            /// Wraps the inner bytes in a `RefCell` for interior mutability.
            pub fn ref_cell(self) -> std::cell::RefCell<Vec<u8>> {
                std::cell::RefCell::new(self.0)
            }

            /// Wraps the inner bytes in an `UnsafeCell`.
            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<Vec<u8>> {
                std::cell::UnsafeCell::new(self.0)
            }

            /// Wraps the inner bytes in a `OnceCell` for one-time initialization.
            pub fn once_cell(self) -> std::cell::OnceCell<Vec<u8>> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }

            /// Attempts to decode the struct instance from the inner byte slice.
            ///
            /// # Errors
            ///
            /// Returns a decoding error if the byte slice cannot be decoded.
            pub fn from(&self) -> Result<#struct_name, bincode::error::DecodeError> {
                bincode::decode_from_slice(&self.0, bincode::config::standard())
                    .map(|(result, _)| result)
            }

            /// Attempts to decode the struct instance from the provided byte slice.
            ///
            /// # Errors
            ///
            /// Returns a decoding error if the byte slice cannot be decoded.
            pub fn from_bytes(&self, bytes: &[u8]) -> Result<#struct_name, bincode::error::DecodeError> {
                bincode::decode_from_slice(bytes, bincode::config::standard())
                    .map(|(result, _)| result)
            }
        }
    }
}
