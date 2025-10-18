use mokuya::components::prelude::{get_fields, get_struct_name, is_string};
use quote::{format_ident, quote};
use syn::DeriveInput;

pub fn generate_parse_by_field(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = get_struct_name(input);
    let struct_name_str = struct_ident.to_string();
    let fields = get_fields(input).expect("fields must be provided");

    let mut inner_wrappers = Vec::new();
    let mut outer_wrappers = Vec::new();
    let mut selector_methods = Vec::new();
    let mut per_field_feature_wrappers = Vec::new();
    let mut per_field_feature_methods = Vec::new();

    for field in fields.iter() {
        let field_ident = field.ident.as_ref().expect("named fields only");
        let field_ty = &field.ty;

        let field_name_str = field_ident.to_string();
        let mut chars = field_name_str.chars();
        let capitalized = match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => field_name_str.clone(),
        };

        let inner_ident = format_ident!("Parse{}{}", struct_name_str, capitalized);
        let outer_ident = format_ident!("Field{}", capitalized);
        let outer_ident_str = outer_ident.to_string();

        inner_wrappers.push(quote! {
            #[derive(Debug)]
            pub struct #inner_ident(pub #field_ty);
        });

        outer_wrappers.push(quote! {
            #[derive(Debug)]
            pub struct #outer_ident(pub #inner_ident);
        });

        selector_methods.push(quote! {
            pub fn #field_ident(self) -> #outer_ident {
                #outer_ident(#inner_ident(self.0.0.#field_ident))
            }
        });

        let arc_ident = format_ident!("{}Arc", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "arc")]
            #[derive(Debug)]
            pub struct #arc_ident(pub std::sync::Arc<#field_ty>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "arc")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn arc(self) -> #arc_ident {
                    #arc_ident(std::sync::Arc::new(self.0.0))
                }
            }
        });

        let tokio_mutex_ident = format_ident!("{}TokioMutex", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "tokio")]
            #[derive(Debug)]
            pub struct #tokio_mutex_ident(pub tokio::sync::Mutex<#field_ty>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "tokio")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn tokio_mutex(self) -> #tokio_mutex_ident {
                    #tokio_mutex_ident(tokio::sync::Mutex::new(self.0.0))
                }
            }
        });

        let mutex_ident = format_ident!("{}Mutex", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "mutex")]
            #[derive(Debug)]
            pub struct #mutex_ident(pub std::sync::Mutex<#field_ty>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "mutex")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn mutex(self) -> #mutex_ident {
                    #mutex_ident(std::sync::Mutex::new(self.0.0))
                }
            }
        });

        let box_ident = format_ident!("{}Box", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "box")]
            #[derive(Debug)]
            pub struct #box_ident(pub Box<#field_ty>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "box")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn boxed(self) -> #box_ident {
                    #box_ident(Box::new(self.0.0))
                }
            }
        });

        let refcell_ident = format_ident!("{}RefCell", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "refcell")]
            #[derive(Debug)]
            pub struct #refcell_ident(pub std::cell::RefCell<#field_ty>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "refcell")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn ref_cell(self) -> #refcell_ident {
                    #refcell_ident(std::cell::RefCell::new(self.0.0))
                }
            }
        });

        let unsafecell_ident = format_ident!("{}UnsafeCell", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "unsafecell")]
            #[derive(Debug)]
            pub struct #unsafecell_ident(pub std::cell::UnsafeCell<#field_ty>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "unsafecell")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn unsafe_cell(self) -> #unsafecell_ident {
                    #unsafecell_ident(std::cell::UnsafeCell::new(self.0.0))
                }
            }
        });

        let oncecell_ident = format_ident!("{}OnceCell", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "oncecell")]
            #[derive(Debug)]
            pub struct #oncecell_ident(pub std::cell::OnceCell<#field_ty>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "oncecell")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn once_cell(self) -> #oncecell_ident {
                    let cell = std::cell::OnceCell::new();
                    let _ = cell.set(self.0.0);
                    #oncecell_ident(cell)
                }
            }
        });

        let vec_ident = format_ident!("{}Vec", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "vec")]
            #[derive(Debug)]
            pub struct #vec_ident(pub Vec<#field_ty>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "vec")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn vec(self) -> #vec_ident {
                    #vec_ident(vec![self.0.0])
                }
            }
        });

        let bin_ident = format_ident!("{}Bin", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "bincode")]
            #[derive(Debug)]
            pub struct #bin_ident(pub Vec<u8>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "bincode")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn bin(self) -> Result<#bin_ident, Box<dyn std::error::Error>> {
                    let config = bincode::config::standard();
                    let serialized = bincode::encode_to_vec(&self.0.0, config)?;
                    Ok(#bin_ident(serialized))
                }
            }
        });

        let json_ident = format_ident!("{}Json", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "serde_json")]
            #[derive(Debug)]
            pub struct #json_ident(pub serde_json::Value);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "serde_json")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn json(self) -> Result<#json_ident, Box<dyn std::error::Error>> {
                    let json = serde_json::to_value(self.0.0)?;
                    Ok(#json_ident(json))
                }
            }
        });

        let hashmap_ident = format_ident!("{}HashMap", outer_ident_str);
        per_field_feature_wrappers.push(quote! {
            #[cfg(feature = "hashmap")]
            #[derive(Debug)]
            pub struct #hashmap_ident(pub std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>);
        });
        per_field_feature_methods.push(quote! {
            #[cfg(feature = "hashmap")]
            impl #outer_ident {
                #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                pub fn hashmap(self) -> #hashmap_ident {
                    let mut map = std::collections::HashMap::new();
                    map.insert(
                        stringify!(#field_ident).to_string(),
                        Box::new(self.0.0) as Box<dyn std::any::Any + Send + Sync>
                    );
                    #hashmap_ident(map)
                }
            }
        });

        if is_string(field_ty) {
            let argon_ident = format_ident!("{}Argon2", outer_ident_str);

            per_field_feature_wrappers.push(quote! {
                #[cfg(feature = "argon2")]
                #[derive(Debug)]
                pub struct #argon_ident(pub String);
            });

            per_field_feature_methods.push(quote! {
                #[cfg(feature = "argon2")]
                impl #outer_ident {
                    #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                    pub fn argon2_hash(self) -> Result<#argon_ident, Box<dyn std::error::Error + Send + Sync>> {
                        use argon2::{
                            Argon2, Algorithm, Version, Params,
                            password_hash::{SaltString, PasswordHasher, rand_core::OsRng},
                        };
                        use std::io::{Error as IoError, ErrorKind};

                        let m_cost_kib: u32 = 65_536;
                        let t_cost: u32 = 3;
                        let p_cost: u32 = match std::thread::available_parallelism() {
                            Ok(nz) => {
                                let v = nz.get() as u32;
                                if v == 0 { 1 } else { std::cmp::min(v, 4) }
                            }
                            Err(_) => 1,
                        };

                        let params = Params::new(m_cost_kib, t_cost, p_cost, Some(32))
                            .map_err(|e| Box::new(IoError::new(ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error + Send + Sync>)?;
                        let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

                        let salt = SaltString::generate(&mut OsRng);
                        let phc = argon
                            .hash_password(self.0 .0.as_bytes(), &salt)
                            .map_err(|e| Box::new(IoError::new(ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error + Send + Sync>)?;

                        Ok(#argon_ident(phc.to_string()))
                    }
                }

            #[cfg(feature = "argon2")]
            impl #argon_ident {
                    #[cfg_attr(feature = "nekotracing", nekotracing::nekotracing)]
                    pub fn verify(&self, candidate: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
                        use argon2::{Argon2, password_hash::PasswordHash, password_hash::PasswordVerifier};
                        use std::io::{Error as IoError, ErrorKind};

                        let parsed = PasswordHash::new(&self.0)
                            .map_err(|e| Box::new(IoError::new(ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error + Send + Sync>)?;

                        let argon = Argon2::default();

                        match argon.verify_password(candidate.as_bytes(), &parsed) {
                            Ok(()) => Ok(true),
                            Err(_) => Ok(false),
                        }
                    }
                }
            });
        }
    }

    quote! {
        #[derive(Debug)]
        pub struct FieldSelector(pub Parse);
        #(#inner_wrappers)*
        #(#outer_wrappers)*
        #(#per_field_feature_wrappers)*
        impl FieldSelector {
            #(#selector_methods)*
        }
        #(#per_field_feature_methods)*
    }
}
