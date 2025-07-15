use kenzu::Builder;
use shori::Parser;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Builder, Parser, Clone, bincode::Encode, bincode::Decode, Serialize, Deserialize,
)]
pub struct User {
    id: i32,
    name: String,
}

fn main() {
    let user = User::new().id(1).name("John Doe").parse().bin().unwrap();
    println!("{:#?}, {:#?}", user, user.from());

    let user = User::new().id(1).name("John Doe").build().unwrap();
    let config = bincode::config::standard();
    let bytes = bincode::encode_to_vec(&user, config).unwrap();
    let (decoded_user, _): (User, usize) =
        bincode::decode_from_slice(&bytes, bincode::config::standard()).unwrap();
    println!("decoded: {:#?}", decoded_user);
}
