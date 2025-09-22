#![cfg(all(feature = "refcell", not(feature = "full")))]

use kenzu::Builder;
use shori::Parser;

#[derive(Builder, PartialEq, Default, Parser, Debug, Clone)]
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
fn parse_ref_cell() {
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
        .ref_cell()
        .get();

    {
        let mut user_borrow = user.borrow_mut();
        user_borrow.name = "Jane Doe".into();
    }

    assert_eq!(user.borrow().name, "Jane Doe");
}
