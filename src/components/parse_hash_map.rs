use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_parse_hash_map() -> TokenStream {
    quote! {
        use std::collections::HashMap;
        use std::any::Any;
        use std::sync::Arc;
        use std::cell::{RefCell, UnsafeCell, OnceCell};

        /// Alias for the HashMap with String keys and boxed dynamic values
        /// that implement Send + Sync.
        type HashMapResp = HashMap<String, Box<dyn Any + Send + Sync>>;

        /// Wrapper around a HashMap storing heterogeneous values.
        pub struct ParseHashMap(HashMapResp);

        impl ParseHashMap {
            /// Attempts to retrieve a reference to a value of type `T` associated
            /// with the given key. Returns `None` if the key is not found or
            /// if the stored value is of a different type.
            pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
                self.0.get(key)
                    .and_then(|value| value.downcast_ref::<T>())
            }

            /// Converts the internal HashMap into an Arc for shared ownership.
            pub fn arc(self) -> Arc<HashMapResp> {
                Arc::new(self.0)
            }

            /// Boxes the internal HashMap.
            pub fn boxed(self) -> Box<HashMapResp> {
                Box::new(self.0)
            }

            /// Wraps the internal HashMap in a RefCell for interior mutability.
            pub fn ref_cell(self) -> RefCell<HashMapResp> {
                RefCell::new(self.0)
            }

            /// Wraps the internal HashMap in an UnsafeCell.
            pub fn unsafe_cell(self) -> UnsafeCell<HashMapResp> {
                UnsafeCell::new(self.0)
            }

            /// Wraps the internal HashMap in a OnceCell for one-time initialization.
            pub fn once_cell(self) -> OnceCell<HashMapResp> {
                let cell = OnceCell::new();
                cell.set(self.0).ok();
                cell
            }
        }
    }
}
