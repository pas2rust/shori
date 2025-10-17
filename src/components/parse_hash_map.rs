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
            #[cfg(not(feature = "nekotracing"))]
            pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
                self.0.get(key).and_then(|v| v.downcast_ref::<T>())
            }

            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Attempts to retrieve a reference to a value of type `T` associated
            /// with the given key. Returns `None` if the key is not found or
            /// if the stored value is of a different type.
            pub fn get<T: 'static + std::fmt::Debug>(&self, key: &str) -> Option<&T> {
                self.0.get(key).and_then(|v| v.downcast_ref::<T>())
            }
            #[cfg(feature="arc")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Converts the internal HashMap into an Arc for shared ownership.
            pub fn arc(self) -> Arc<HashMapResp> {
                Arc::new(self.0)
            }

            #[cfg(feature="box")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Boxes the internal HashMap.
            pub fn boxed(self) -> Box<HashMapResp> {
                Box::new(self.0)
            }

            #[cfg(feature="refcell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the internal HashMap in a RefCell for interior mutability.
            pub fn ref_cell(self) -> RefCell<HashMapResp> {
                RefCell::new(self.0)
            }

            #[cfg(feature="unsafecell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the internal HashMap in an UnsafeCell.
            pub fn unsafe_cell(self) -> UnsafeCell<HashMapResp> {
                UnsafeCell::new(self.0)
            }

            #[cfg(feature="oncecell")]
            #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
            /// Wraps the internal HashMap in a OnceCell for one-time initialization.
            pub fn once_cell(self) -> OnceCell<HashMapResp> {
                let cell = OnceCell::new();
                cell.set(self.0).ok();
                cell
            }
        }

        impl std::fmt::Debug for ParseHashMap {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let keys: Vec<&String> = self.0.keys().collect();
                f.debug_struct("ParseHashMap")
                    .field("keys", &keys)
                    .finish()
            }
        }
    }
}
