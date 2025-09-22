use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

pub fn generate_parse_once_cell(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        pub struct ParseOnceCell(std::cell::OnceCell<#struct_name>);

        impl ParseOnceCell {
            /// Returns an `Option` with a reference to the contained value if it has been initialized.
            pub fn get(&self) -> Option<&#struct_name> {
                self.0.get()
            }

            #[cfg(feature="arc")]
            /// Consumes self and returns an `Arc` wrapping the `OnceCell` for thread-safe shared ownership.
            pub fn arc(self) -> std::sync::Arc<std::cell::OnceCell<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            #[cfg(feature="box")]
            /// Consumes self and returns a boxed `OnceCell`.
            pub fn boxed(self) -> Box<std::cell::OnceCell<#struct_name>> {
                Box::new(self.0)
            }
        }
    }
}
