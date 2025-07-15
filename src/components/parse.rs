use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

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

    quote! {
        pub struct Parse(#struct_name);

        //#[cfg_attr(feature = "tracing", mdd::debugger_impl)]
        impl Parse {
            pub fn arc(self) -> ParseArc {
                ParseArc(std::sync::Arc::new(self.0))
            }

            pub fn tokio_mutex(self) -> ParseTokioMutex {
                ParseTokioMutex(tokio::sync::Mutex::new(self.0))
            }

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

            pub fn mutex(self) -> ParseMutex {
                ParseMutex(std::sync::Mutex::new(self.0))
            }

            pub fn boxed(self) -> ParseBox {
                ParseBox(Box::new(self.0))
            }

            pub fn ref_cell(self) -> ParseRefCell {
                ParseRefCell(std::cell::RefCell::new(self.0))
            }

            pub fn bin(self) -> Result<ParseBin, Box<dyn Error>> {
                let config = bincode::config::standard();
                let serialized = bincode::encode_to_vec(&self.0, config)?;
                Ok(ParseBin(serialized))
            }

            pub fn json(self) -> Result<ParseJson, Box<dyn Error>> {
                let json = serde_json::to_value(self.0)?;
                Ok(ParseJson(json))
            }

            pub fn vec(self) -> ParseVec {
                ParseVec(vec![self.0])
            }

            pub fn unsafe_cell(self) -> ParseUnsafeCell {
                ParseUnsafeCell(std::cell::UnsafeCell::new(self.0))
            }

            pub fn once_cell(self) -> ParseOnceCell {
                let cell = std::cell::OnceCell::new();
                cell.set(self.0).ok();
                ParseOnceCell(cell)
            }


            pub fn tuple(&self) -> (#(&#field_types),*) {
                (#(#field_names),*)
            }
        }

        //#[cfg_attr(feature = "tracing", mdd::debugger_impl)]
        impl #impl_block {
            pub fn parse(self) -> Parse {
                Parse(self)
            }
        }
    }
}
