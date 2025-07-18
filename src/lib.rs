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
            generate_parse_mutex(&input),
            generate_parse_arc(&input),
            generate_parse_box(&input),
            generate_parse_once_cell(&input),
            generate_parse_ref_cell(&input),
            generate_parse_unsafe_cell(&input),
            generate_parse_vec(&input),
            generate_parse_tokio_mutex(&input),
            generate_parse_hash_map(),
            generate_parse_json(&input),
            generate_parse_bin(&input),
            generate_parse_toml(&input),
        ],
    );
    expanded.into()
}
