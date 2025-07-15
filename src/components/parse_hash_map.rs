use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_parse_hash_map() -> TokenStream {
    quote! {
        use std::collections::HashMap;
        use std::any::Any;
        use std::sync::Arc;
        use std::cell::{RefCell, UnsafeCell, OnceCell};

        type HashMapResp = HashMap<String, Box<dyn Any + Send + Sync>>;

        pub struct ParseHashMap(HashMapResp);

        impl ParseHashMap {
            pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
                self.0.get(key)
                    .and_then(|value| value.downcast_ref::<T>())
            }

            pub fn arc(self) -> Arc<HashMapResp> {
                Arc::new(self.0)
            }

            pub fn boxed(self) -> Box<HashMapResp> {
                Box::new(self.0)
            }

            pub fn ref_cell(self) -> RefCell<HashMapResp> {
                RefCell::new(self.0)
            }

            pub fn unsafe_cell(self) -> UnsafeCell<HashMapResp> {
                UnsafeCell::new(self.0)
            }

            pub fn once_cell(self) -> OnceCell<HashMapResp> {
                let cell = OnceCell::new();
                cell.set(self.0).ok();
                cell
            }
        }
    }
}
