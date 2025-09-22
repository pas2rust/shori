#![cfg(all(feature = "serde", feature = "bincode", not(feature = "full")))]

use kenzu::Builder;
use serde::{Deserialize, Serialize};
use shori::Parser;

#[derive(
    Builder,
    PartialEq,
    Default,
    Parser,
    Debug,
    Clone,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
pub struct User {
    pub id: String,
    #[set(value = "name")]
    pub name: String,
    pub password: String,
    #[build(
        pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
        err = "err"
    )]
    #[set(value = "email@example.com")]
    pub email: String,
    #[set(value = 18)]
    pub age: u8,
    pub gender: String,
}

#[test]
fn parse_bin() {
    let user_parse_bin = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(25)
        .gender("F")
        .build()
        .unwrap()
        .parse()
        .bin()
        .unwrap();

    let user_bin = user_parse_bin.get();
    let user_bin_from_bytes = user_parse_bin.from_bytes(&user_bin);
    assert!(user_bin_from_bytes.is_ok());

    let user_hex = user_parse_bin.hex();
    let user_from_hex = user_parse_bin.from_hex(&user_hex);

    assert!(user_from_hex.is_ok());
    let user = user_from_hex.unwrap();

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");

    let user = user_bin_from_bytes.unwrap();
    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");

    let user_from = user_parse_bin.from();
    assert!(user_from.is_ok());

    let user = user_from.unwrap();
    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");
}
