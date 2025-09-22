use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Generates a wrapper around `Vec<#struct_name>` with smart container adapters.
pub fn generate_parse_vec(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        /// A wrapper around a `Vec<#struct_name>` with conversion methods for common wrappers.
        pub struct ParseVec(Vec<#struct_name>);

        impl ParseVec {
            /// Consumes `self` and returns the internal `Vec`.
            pub fn get(self) -> Vec<#struct_name> {
                self.0
            }

            #[cfg(feature="arc")]
            /// Converts the `Vec` into an `Arc` for shared ownership.
            pub fn arc(self) -> std::sync::Arc<Vec<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            #[cfg(feature="box")]
            /// Converts the `Vec` into a boxed version for heap allocation.
            pub fn boxed(self) -> Box<Vec<#struct_name>> {
                Box::new(self.0)
            }

            #[cfg(feature="refcell")]
            /// Converts the `Vec` into a `RefCell` for interior mutability.
            pub fn ref_cell(self) -> std::cell::RefCell<Vec<#struct_name>> {
                std::cell::RefCell::new(self.0)
            }

            #[cfg(feature="unsafecell")]
            /// Converts the `Vec` into an `UnsafeCell` for unchecked interior mutability.
            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<Vec<#struct_name>> {
                std::cell::UnsafeCell::new(self.0)
            }

            #[cfg(feature="oncecell")]
            /// Converts the `Vec` into a `OnceCell` that can be set once.
            pub fn once_cell(self) -> std::cell::OnceCell<Vec<#struct_name>> {
                let cell = std::cell::OnceCell::new();
                let _ = cell.set(self.0);
                cell
            }
        }
    }
}
