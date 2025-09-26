#![cfg(feature = "full")]

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

#[test]
fn parse_arc_mutex() {
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

    let locked_user = user.lock().unwrap();

    assert_eq!(locked_user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(locked_user.name, "John Doe");
    assert_eq!(locked_user.password, "password123");
    assert_eq!(locked_user.email, "johndoe@example.com");
    assert_eq!(locked_user.age, 25);
    assert_eq!(locked_user.gender, "F");
}

#[test]
fn parse_arc() {
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
        .arc()
        .get();

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");
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

#[test]
fn parse_mutex() {
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
        .get();

    let user = user.lock().unwrap();

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");
}

#[test]
fn parse_once_cell() {
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
        .once_cell();

    assert_eq!(user.get().unwrap().name, "John Doe");
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

#[tokio::test]
async fn parse_tokio_mutex() {
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
        .tokio_mutex()
        .get();

    let user = user.lock().await;

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");
}

#[test]
fn parse_toml() {
    let user_parse_toml = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(25)
        .gender("F")
        .build()
        .unwrap()
        .parse()
        .toml()
        .unwrap();

    let user_toml_value = user_parse_toml.get();
    let from_value_user = user_parse_toml.from_value(&user_toml_value);
    assert!(from_value_user.is_ok());

    let user = from_value_user.unwrap();
    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");

    let user_from = user_parse_toml.from();
    assert!(user_from.is_ok());

    let user = user_from.unwrap();
    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");
}

#[test]
fn parse_tuple() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(25)
        .gender("F")
        .build()
        .unwrap()
        .parse();

    let (id, name, password, email, age, gender) = user.tuple();

    assert_eq!(id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(name, "John Doe");
    assert_eq!(password, "password123");
    assert_eq!(email, "johndoe@example.com");
    assert_eq!(age, &25);
    assert_eq!(gender, "F");
}

#[test]
fn parse_unsafe_cell() {
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
        .unsafe_cell()
        .get();

    let user_ptr = user.get();
    unsafe {
        (*user_ptr).name = "Jane Doe".into();
    }

    unsafe {
        assert_eq!((*user_ptr).name, "Jane Doe");
    }
}

#[test]
fn parse_vec() {
    let users = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(25)
        .gender("F")
        .build()
        .unwrap()
        .parse()
        .vec()
        .get();

    assert_eq!(users.len(), 1);

    let user = &users[0];

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");
}
