use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

/// Generates a wrapper around a `tokio::sync::Mutex` containing the target struct,
/// with various utility methods for wrapping in other containers.
pub fn generate_parse_tokio_mutex(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        #[derive(Debug)]
        /// A wrapper around `tokio::sync::Mutex<#struct_name>` providing utility methods.
        pub struct ParseTokioMutex(tokio::sync::Mutex<#struct_name>);

        impl ParseTokioMutex {
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Consumes `self` and returns the inner `tokio::sync::Mutex`.
            pub fn get(self) -> tokio::sync::Mutex<#struct_name> {
                self.0
            }

            #[cfg(feature="arc")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the mutex in an `Arc` for shared ownership across threads.
            pub fn arc(self) -> std::sync::Arc<tokio::sync::Mutex<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            #[cfg(feature="box")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Boxes the mutex into a `Box`.
            pub fn boxed(self) -> Box<tokio::sync::Mutex<#struct_name>> {
                Box::new(self.0)
            }

            #[cfg(feature="refcell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the mutex in a `RefCell` for interior mutability in single-threaded contexts.
            pub fn ref_cell(self) -> std::cell::RefCell<tokio::sync::Mutex<#struct_name>> {
                std::cell::RefCell::new(self.0)
            }

            #[cfg(feature="unsafecell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the mutex in an `UnsafeCell` for low-level interior mutability.
            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<tokio::sync::Mutex<#struct_name>> {
                std::cell::UnsafeCell::new(self.0)
            }

            #[cfg(feature="oncecell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Places the mutex in a `OnceCell`, allowing single initialization.
            pub fn once_cell(self) -> std::cell::OnceCell<tokio::sync::Mutex<#struct_name>> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }
        }
    }
}
