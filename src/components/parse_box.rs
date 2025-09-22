use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_parse_box(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        /// A wrapper around `Box<#struct_name>` providing helper methods to
        /// convert into various smart pointer containers.
        pub struct ParseBox(Box<#struct_name>);

        //#[cfg_attr(feature = "tracing", mdd::debugger_impl)]
        impl ParseBox {
            /// Returns the inner `Box<#struct_name>`.
            pub fn get(self) -> Box<#struct_name> {
                self.0
            }

            #[cfg(feature="arc")]
            /// Converts the boxed value into an `Arc<Box<#struct_name>>`.
            pub fn arc(self) -> std::sync::Arc<Box<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            #[cfg(feature = "tokio")]
            /// Wraps the boxed value in a Tokio async mutex.
            pub fn tokio_mutex(self) -> tokio::sync::Mutex<Box<#struct_name>> {
                tokio::sync::Mutex::new(self.0)
            }

            #[cfg(feature="mutex")]
            /// Wraps the boxed value in a standard mutex.
            pub fn mutex(self) -> std::sync::Mutex<Box<#struct_name>> {
                std::sync::Mutex::new(self.0)
            }

            #[cfg(feature="refcell")]
            /// Wraps the boxed value in a `RefCell` for interior mutability.
            pub fn ref_cell(self) -> std::cell::RefCell<Box<#struct_name>> {
                std::cell::RefCell::new(self.0)
            }

            #[cfg(feature="unsafecell")]
            /// Wraps the boxed value in an `UnsafeCell`.
            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<Box<#struct_name>> {
                std::cell::UnsafeCell::new(self.0)
            }

            #[cfg(feature="oncecell")]
            /// Wraps the boxed value in a `OnceCell` for one-time initialization.
            pub fn once_cell(self) -> std::cell::OnceCell<Box<#struct_name>> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }
        }
    }
}
