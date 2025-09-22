use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

/// Generates a wrapper around `std::cell::UnsafeCell` containing the target struct,
/// with utility methods to wrap it in `Arc` or `Box`.
pub fn generate_parse_unsafe_cell(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        /// A wrapper around `std::cell::UnsafeCell<#struct_name>` with container conversion helpers.
        pub struct ParseUnsafeCell(std::cell::UnsafeCell<#struct_name>);

        impl ParseUnsafeCell {
            /// Consumes `self` and returns the inner `UnsafeCell`.
            pub fn get(self) -> std::cell::UnsafeCell<#struct_name> {
                self.0
            }

            #[cfg(feature="arc")]
            /// Wraps the `UnsafeCell` in an `Arc` for shared ownership across threads.
            pub fn arc(self) -> std::sync::Arc<std::cell::UnsafeCell<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            #[cfg(feature="box")]
            /// Boxes the `UnsafeCell`.
            pub fn boxed(self) -> Box<std::cell::UnsafeCell<#struct_name>> {
                Box::new(self.0)
            }
        }
    }
}
