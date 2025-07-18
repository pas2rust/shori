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
fn parse_arc_mutex_concurrent() {
    use std::sync::Arc;
    use std::thread;

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
        .mutex()
        .arc();

    let user_clone = Arc::clone(&user);
    let handle = thread::spawn(move || {
        let mut locked_user = user_clone.lock().unwrap();
        locked_user.name = "Jane Doe".into();
    });

    handle.join().unwrap();

    assert_eq!(user.lock().unwrap().name, "Jane Doe");
}
