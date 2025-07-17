use kenzu::Builder;
use serde::{Deserialize, Serialize};
use shori::macros::Parser;

#[derive(
    Builder,
    Default,
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
    Parser,
)]
pub struct User {
    pub id: String,
    #[set(value = "name")]
    pub name: String,
    pub password: String,
    #[set(value = "email@example.com")]
    pub email: String,
    #[set(value = 18)]
    pub age: u8,
    pub gender: String,
}

#[test]
fn parse_toml() {
    let user_parse = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("123")
        .email("john@example.com")
        .age(25)
        .gender("M")
        .build()
        .unwrap()
        .parse()
        .toml()
        .unwrap();

    let toml_val = user_parse.get();
    let recovered = user_parse.from_value(&toml_val).unwrap();

    assert_eq!(recovered.name, "John Doe");

    let from = user_parse.from().unwrap();
    assert_eq!(from.email, "john@example.com");
}

#[test]
fn parse_box() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(25)
        .gender("F")
        .build()
        .unwrap()
        .parse()
        .boxed()
        .get();

    assert_eq!(user.name, "John Doe");
}