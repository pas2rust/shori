#![cfg(all(feature = "serde", feature = "serde_json", not(feature = "full")))]

use kenzu::Builder;
use serde::{Deserialize, Serialize};
use shori::Parser;

#[derive(Builder, PartialEq, Default, Parser, Debug, Clone, Serialize, Deserialize)]
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
fn parse_json() {
    let user_parse_json = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(25)
        .gender("F")
        .build()
        .unwrap()
        .parse()
        .json()
        .unwrap();

    let user_json = user_parse_json.get();
    let from_value_user = user_parse_json.from_value(&user_json);
    assert!(from_value_user.is_ok());

    let user = from_value_user.unwrap();
    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");

    let user_from = user_parse_json.from();
    assert!(user_from.is_ok());

    let user = user_from.unwrap();
    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");
}
