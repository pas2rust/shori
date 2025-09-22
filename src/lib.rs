use components::prelude::*;
use mokuya::components::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, parse_macro_input};

mod components;

#[proc_macro_derive(
    Parser,
    //attributes(parser)
)]
pub fn parser(input: TokenStream) -> TokenStream {
    let mut expanded = TokenStream2::new();
    let mut input = parse_macro_input!(input as DeriveInput);
    add_traits_to_generics(&mut input);
    for_extend_token_stream(
        &mut expanded,
        vec![
            generate_parse(&input),
            #[cfg(feature = "mutex")]
            generate_parse_mutex(&input),
            #[cfg(feature = "arc")]
            generate_parse_arc(&input),
            #[cfg(feature = "box")]
            generate_parse_box(&input),
            #[cfg(feature = "oncecell")]
            generate_parse_once_cell(&input),
            #[cfg(feature = "refcell")]
            generate_parse_ref_cell(&input),
            #[cfg(feature = "unsafecell")]
            generate_parse_unsafe_cell(&input),
            #[cfg(feature = "vec")]
            generate_parse_vec(&input),
            #[cfg(feature = "hashmap")]
            generate_parse_hash_map(),
            #[cfg(feature = "tokio")]
            generate_parse_tokio_mutex(&input),
            #[cfg(feature = "serde_json")]
            generate_parse_json(&input),
            #[cfg(feature = "bincode")]
            generate_parse_bin(&input),
            #[cfg(feature = "toml")]
            generate_parse_toml(&input),
        ],
    );
    expanded.into()
}
