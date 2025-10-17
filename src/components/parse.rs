use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::generate_parse_by_field;

pub fn generate_parse(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);
    let impl_block = get_impl(input);
    let fields = get_fields(input).expect("fields must be provided");
    let field_names: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            quote! { &self.0.#field_name }
        })
        .collect();
    let field_types: Vec<_> = fields.iter().map(|field| &field.ty).collect();
    let field_idents: Vec<_> = fields.iter().map(|field| &field.ident).collect();
    let parse_by_field = generate_parse_by_field(input);

    quote! {
        #[derive(Debug)]
        /// Wrapper type that provides parsing and transformation utilities
        /// for the underlying struct.
        pub struct Parse(#struct_name);
        #parse_by_field

        impl Parse {
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            pub fn field(self) -> FieldSelector {
                FieldSelector(self)
            }
            #[cfg(feature="arc")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Converts the inner struct into an `Arc`, wrapped in `ParseArc`.
            ///
            /// Useful for thread-safe shared ownership.
            pub fn arc(self) -> ParseArc {
                ParseArc(std::sync::Arc::new(self.0))
            }

            #[cfg(feature = "tokio")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Converts into a `tokio::sync::Mutex`, wrapped in `ParseTokioMutex`.
            ///
            /// Useful for safe mutation across async tasks.
            pub fn tokio_mutex(self) -> ParseTokioMutex {
                ParseTokioMutex(tokio::sync::Mutex::new(self.0))
            }

            #[cfg(feature="hashmap")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Converts the struct fields into a `HashMap<String, Box<dyn Any + Send + Sync>>`.
            ///
            /// This enables dynamic access to fields by their name.
            pub fn hashmap(self) -> ParseHashMap {
                let mut map = std::collections::HashMap::new();
                #(
                    map.insert(
                        stringify!(#field_idents).to_string(),
                        Box::new(self.0.#field_idents) as Box<dyn std::any::Any + Send + Sync>
                    );
                )*
                ParseHashMap(map)
            }

            #[cfg(feature="mutex")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Converts into a `std::sync::Mutex` wrapped in `ParseMutex`.
            ///
            /// Use this for interior mutability in synchronous code.
            pub fn mutex(self) -> ParseMutex {
                ParseMutex(std::sync::Mutex::new(self.0))
            }

            #[cfg(feature="box")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Boxes the inner struct into `Box<T>`, wrapped in `ParseBox`.
            pub fn boxed(self) -> ParseBox {
                ParseBox(Box::new(self.0))
            }

            #[cfg(feature="refcell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the struct in a `RefCell`, allowing interior mutability in single-threaded contexts.
            pub fn ref_cell(self) -> ParseRefCell {
                ParseRefCell(std::cell::RefCell::new(self.0))
            }

            #[cfg(feature = "bincode")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Serializes the struct into binary using `bincode`, wrapped in `ParseBin`.
            ///
            /// # Errors
            /// Returns an error if serialization fails.
            pub fn bin(self) -> Result<ParseBin, Box<dyn std::error::Error>> {
                let config = bincode::config::standard();
                let serialized = bincode::encode_to_vec(&self.0, config)?;
                Ok(ParseBin(serialized))
            }

            #[cfg(feature = "toml")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Serializes the struct into a `toml::Value`, wrapped in `ParseToml`.
            ///
            /// # Errors
            /// Returns an error if TOML serialization or parsing fails.
            pub fn toml(self) -> Result<ParseToml, Box<dyn std::error::Error>> {
                let toml_string = toml::to_string(&self.0)?;
                let value = toml::from_str::<toml::Value>(&toml_string)?;
                Ok(ParseToml(value))
            }

            #[cfg(feature = "serde_json")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Serializes the struct into a `serde_json::Value`, wrapped in `ParseJson`.
            ///
            /// # Errors
            /// Returns an error if JSON serialization fails.
            pub fn json(self) -> Result<ParseJson, Box<dyn std::error::Error>> {
                let json = serde_json::to_value(self.0)?;
                Ok(ParseJson(json))
            }

            #[cfg(feature="vec")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the struct into a `Vec<T>`, containing a single element.
            ///
            /// Useful for APIs that expect list input.
            pub fn vec(self) -> ParseVec {
                ParseVec(vec![self.0])
            }

            #[cfg(feature="unsafecell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the struct in an `UnsafeCell`.
            ///
            /// Allows unchecked interior mutability.
            pub fn unsafe_cell(self) -> ParseUnsafeCell {
                ParseUnsafeCell(std::cell::UnsafeCell::new(self.0))
            }

            #[cfg(feature="oncecell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Initializes a `OnceCell` with the struct.
            ///
            /// The value is set once and subsequent attempts are ignored.
            pub fn once_cell(self) -> ParseOnceCell {
                let cell = std::cell::OnceCell::new();
                let _ = cell.set(self.0);
                ParseOnceCell(cell)
            }

            #[cfg(feature="tuple")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Returns a tuple of references to all the struct's fields.
            ///
            /// Useful for destructuring or pattern matching.
            pub fn tuple(&self) -> (#(&#field_types),*) {
                (#(#field_names),*)
            }
        }

        impl #impl_block {
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Converts the original struct into a [`Parse`] wrapper.
            ///
            /// Enables access to parsing utilities and conversions.
            pub fn parse(self) -> Parse {
                Parse(self)
            }
        }
    }
}
