use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use mokuya::components::prelude::*;

pub fn generate_parse_ref_cell(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        pub struct ParseRefCell(std::cell::RefCell<#struct_name>);

        impl ParseRefCell {
            /// Consumes self and returns the inner `RefCell` containing the struct.
            pub fn get(self) -> std::cell::RefCell<#struct_name> {
                self.0
            }

            /// Consumes self and returns an `Arc` wrapping the `RefCell` for shared ownership.
            pub fn arc(self) -> std::sync::Arc<std::cell::RefCell<#struct_name>> {
                std::sync::Arc::new(self.0)
            }

            /// Consumes self and returns a boxed `RefCell`.
            pub fn boxed(self) -> Box<std::cell::RefCell<#struct_name>> {
                Box::new(self.0)
            }

            /// Consumes self and returns an `UnsafeCell` wrapping the `RefCell`.
            pub fn unsafe_cell(self) -> std::cell::UnsafeCell<std::cell::RefCell<#struct_name>> {
                std::cell::UnsafeCell::new(self.0)
            }
        }
    }
}
