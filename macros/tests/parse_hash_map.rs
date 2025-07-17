use kenzu::Builder;
use serde::{Deserialize, Serialize};
use macros::Parser;

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
fn parse_hash_map() {
    let id = "123e4567-e89b-12d3-a456-426614174000".to_string();
    let name = "John Doe".to_string();
    let password = "password123".to_string();
    let email = "johndoe@example.com".to_string();
    let age = 25;
    let gender = "F".to_string();

    let user = User::new()
        .id(&id)
        .name(&name)
        .password(&password)
        .email(&email)
        .age(age)
        .gender(&gender)
        .build()
        .unwrap()
        .parse()
        .hashmap();

    assert_eq!(user.get::<String>("id").unwrap(), &id);
    assert_eq!(user.get::<String>("name").unwrap(), &name);
    assert_eq!(user.get::<String>("password").unwrap(), &password);
    assert_eq!(user.get::<String>("email").unwrap(), &email);
    assert_eq!(user.get::<u8>("age").unwrap(), &age);
    assert_eq!(user.get::<String>("gender").unwrap(), &gender);
}
