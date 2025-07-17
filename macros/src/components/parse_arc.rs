use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

pub fn generate_parse_arc(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        /// A wrapper around `Arc<#struct_name>` that provides helper methods to
        /// convert into common smart pointer containers.
        pub struct ParseArc(std::sync::Arc<#struct_name>);

        //#[cfg_attr(feature = "tracing", mdd::debugger_impl)]
        impl ParseArc {
            /// Returns the inner `Arc<#struct_name>` value.
            pub fn get(self) -> std::sync::Arc<#struct_name> {
                self.0
            }

            /// Converts the `Arc<#struct_name>` into a `Box<Arc<#struct_name>>`.
            pub fn boxed(self) -> Box<std::sync::Arc<#struct_name>> {
                Box::new(self.0)
            }

            /// Wraps the `Arc<#struct_name>` inside a `RefCell` for interior mutability.
            pub fn ref_cell(self) -> std::cell::RefCell<std::sync::Arc<#struct_name>> {
                std::cell::RefCell::new(self.0)
            }

            /// Wraps the `Arc<#struct_name>` in a `OnceCell` for one-time initialization.
            pub fn once_cell(self) -> std::cell::OnceCell<std::sync::Arc<#struct_name>> {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                cell
            }

            /// Wraps the `Arc<#struct_name>` inside an `UnsafeCell`, allowing low-level mutability.
            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<std::sync::Arc<#struct_name>> {
                std::cell::UnsafeCell::new(self.0)
            }
        }
    }
}
