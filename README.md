# `shori`

[![Crates.io](https://img.shields.io/crates/v/shori.svg)](https://crates.io/crates/shori)
[![Docs.rs](https://docs.rs/shori/badge.svg)](https://docs.rs/shori)
[![License](https://img.shields.io/crates/l/shori.svg)](https://github.com/pas2rust/shori/blob/main/LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/pas2rust/shori?color=orange&logo=rust&style=flat&logoColor=white)
![GitHub stars](https://img.shields.io/github/stars/pas2rust/shori?color=success&style=flat&logo=github)
![GitHub forks](https://img.shields.io/github/forks/pas2rust/shori?color=orange&logo=Furry%20Network&style=flat&logoColor=white)
![Tests](https://raw.githubusercontent.com/pas2rust/badges/main/shori-tests.svg)
![Crates.io downloads](https://img.shields.io/crates/d/shori.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/pas2rust/shori?color=ff69b4&label=update&logo=git&style=flat&logoColor=white)


**`shori`** is a flexible data transformation and parsing toolkit for Rust.  
It provides the `#[derive(Parser)]` macro, which automatically implements conversions for your structs—supporting formats like JSON, TOML, bincode, and common smart pointers and containers like `Arc`, `Mutex`, and `Box`.

Designed for ergonomic and type-safe data manipulation.

---

## 🔧 Features

### ✨ Auto-Derived Parsing & Conversions

- `#[derive(Parser)]` implements:
  - `.parse().json()`
  - `.toml()`
  - `.bin()`
  - `.map()`
  - `.from()`, `.from_value()`
- Supports conversion from and to:
  - `String`, `Vec<u8>`, `serde_json::Value`, `toml::Value`, `HashMap<String, Value>`
  - Wrappers: `Box`, `Arc`, `Mutex`, `RefCell`, `OnceCell`, `UnsafeCell`, `tokio::sync::Mutex`, `Vec<T>`

---

## 📦 Installation

```bash
cargo add shori
```

## 🚀 Usage
Basic Example

```rust
#![cfg(feature = "full")]

use kenzu::Builder;
use serde::{Deserialize, Serialize};
use shori::Parser;

#[derive(
    Builder,
    PartialEq,
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
    #[opt(default = "name")]
    pub name: String,
    pub password: String,
    #[opt(
        pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
        err = "err",
        default = "email@example.com"
    )]
    pub email: String,
    #[opt(default = 18)]
    pub age: u8,
    pub gender: String,
}

#[test]
fn parse_bin() -> Result<(), String> {
    let user_parse_bin = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse()
        .bin()
        .unwrap();

    let user_bin = user_parse_bin.get();
    let user_bin_from_bytes = user_parse_bin.from_bytes(user_bin);
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

    Ok(())
}

#[test]
fn parse_arc_mutex_concurrent() -> Result<(), String> {
    use std::sync::Arc;
    use std::thread;

    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
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

    Ok(())
}

#[test]
fn parse_arc_mutex() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
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

    Ok(())
}

#[test]
fn parse_arc() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse()
        .arc()
        .get();

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.gender, "F");

    Ok(())
}

#[test]
fn parse_box() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse()
        .boxed()
        .get();

    assert_eq!(user.name, "John Doe");

    Ok(())
}

#[test]
fn parse_hash_map() -> Result<(), String> {
    let id = "123e4567-e89b-12d3-a456-426614174000";
    let name = "John Doe";
    let password = "password123";
    let email = "johndoe@example.com";
    let age = 25;
    let gender = "F";

    let user = User::new()
        .id(UserId::new(id)?)
        .name(UserName::new(name)?)
        .password(UserPassword::new(password)?)
        .email(UserEmail::new(email)?)
        .age(UserAge::new(age)?)
        .gender(UserGender::new(gender)?)
        .parse()
        .hashmap();

    assert_eq!(user.get::<String>("id").unwrap(), &id);
    assert_eq!(user.get::<String>("name").unwrap(), &name);
    assert_eq!(user.get::<String>("password").unwrap(), &password);
    assert_eq!(user.get::<String>("email").unwrap(), &email);
    assert_eq!(user.get::<u8>("age").unwrap(), &age);
    assert_eq!(user.get::<String>("gender").unwrap(), &gender);

    Ok(())
}

#[test]
fn parse_json() -> Result<(), String> {
    let user_parse_json = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse()
        .json()
        .unwrap();

    let user_json = user_parse_json.get();
    let from_value_user = user_parse_json.from_value(user_json);
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

    Ok(())
}

#[test]
fn parse_mutex() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
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

    Ok(())
}

#[test]
fn parse_once_cell() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse()
        .once_cell();

    assert_eq!(user.get().unwrap().name, "John Doe");

    Ok(())
}

#[test]
fn parse_ref_cell() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse()
        .ref_cell()
        .get();

    {
        let mut user_borrow = user.borrow_mut();
        user_borrow.name = "Jane Doe".into();
    }

    assert_eq!(user.borrow().name, "Jane Doe");

    Ok(())
}

#[tokio::test]
async fn parse_tokio_mutex() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
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

    Ok(())
}

#[test]
fn parse_toml() -> Result<(), String> {
    let user_parse_toml = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse()
        .toml()
        .unwrap();

    let user_toml_value = user_parse_toml.get();
    let from_value_user = user_parse_toml.from_value(user_toml_value);
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

    Ok(())
}

#[test]
fn parse_tuple() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse();

    let (id, name, password, email, age, gender) = user.tuple();

    assert_eq!(id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(name, "John Doe");
    assert_eq!(password, "password123");
    assert_eq!(email, "johndoe@example.com");
    assert_eq!(age, &25);
    assert_eq!(gender, "F");

    Ok(())
}

#[test]
fn parse_unsafe_cell() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
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

    Ok(())
}

#[test]
fn parse_vec() -> Result<(), String> {
    let users = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
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

    Ok(())
}

#[tokio::test]
async fn use_field_tokio_mutex_and_tuple() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?)
        .parse();

    let (id, name, password, email, age, gender) = user.tuple();
    assert_eq!(id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(name, "John Doe");
    assert_eq!(password, "password123");
    assert_eq!(email, "johndoe@example.com");
    assert_eq!(age, &25);
    assert_eq!(gender, "F");

    #[cfg(feature = "tokio")]
    {
        let name_tokio = User::new()
            .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
            .name(UserName::new("John Doe")?)
            .password(UserPassword::new("password123")?)
            .email(UserEmail::new("johndoe@example.com")?)
            .age(UserAge::new(25)?)
            .gender(UserGender::new("F")?)
            .parse()
            .field()
            .name()
            .tokio_mutex();

        let mut locked = name_tokio.0.lock().await;
        assert_eq!(locked.as_str(), "John Doe");
        *locked = "Jane Doe".into();
        assert_eq!(locked.as_str(), "Jane Doe");
    }

    Ok(())
}

#[test]
fn use_field_basic_and_wrappers() -> Result<(), String> {
    let base = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .email(UserEmail::new("johndoe@example.com")?)
        .age(UserAge::new(25)?)
        .gender(UserGender::new("F")?);

    let wrapped = base.clone().parse().field().name();
    match wrapped {
        FieldName(ParseUserName(inner)) => {
            assert_eq!(inner, "John Doe");
        }
    }

    let name_arc = base.clone().parse().field().name().arc();
    assert_eq!(&*name_arc.0, "John Doe");

    let name_box = base.clone().parse().field().name().boxed();
    assert_eq!(*name_box.0, "John Doe");

    let name_cell = base.clone().parse().field().name().ref_cell();
    {
        let mut borrow = name_cell.0.borrow_mut();
        *borrow = "Jane Doe".into();
    }
    assert_eq!(name_cell.0.borrow().as_str(), "Jane Doe");

    let name_once = base.clone().parse().field().name().once_cell();
    assert_eq!(name_once.0.get().unwrap(), "John Doe");

    let name_vec = base.clone().parse().field().name().vec();
    assert_eq!(name_vec.0.len(), 1);
    assert_eq!(name_vec.0[0], "John Doe");

    let name_bin = base
        .clone()
        .parse()
        .field()
        .name()
        .bin()
        .map_err(|e| e.to_string())?;
    assert!(!name_bin.0.is_empty());

    let name_json = base
        .clone()
        .parse()
        .field()
        .name()
        .json()
        .map_err(|e| e.to_string())?;
    assert_eq!(name_json.0, "John Doe");

    let hm = base.clone().parse().field().name().hashmap();
    let boxed = hm.0.get("name").expect("expected key 'name'");
    let any_ref: &dyn Any = &**boxed;
    if let Some(s) = any_ref.downcast_ref::<String>() {
        assert_eq!(s.as_str(), "John Doe");
    } else {
        panic!("expected String inside HashMap");
    }

    let uc = base.clone().parse().field().name().unsafe_cell();
    let ptr = uc.0.get();
    unsafe {
        assert_eq!(&*ptr, "John Doe");
    }

    let m = base.clone().parse().field().name().mutex();
    let guard = m.0.lock().unwrap();
    assert_eq!(&*guard, "John Doe");

    Ok(())
}

#[test]
fn serialization_wrappers() -> Result<(), Box<dyn std::error::Error>> {
    let base = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?);

    let name_bin = base.clone().parse().field().name().bin()?;
    assert!(!name_bin.0.is_empty());

    let name_json = base.parse().field().name().json()?;
    assert_eq!(name_json.0, "John Doe");

    Ok(())
}

#[test]
#[cfg(feature = "mutex")]
fn mutex_wrappers() {
    let base = User::new().name(UserName::new("John Doe").unwrap());
    let m = base.parse().field().name().mutex();
    let guard = m.0.lock().unwrap();
    assert_eq!(&*guard, "John Doe");
}

#[test]
#[cfg(feature = "tokio")]
fn tokio_mutex_wrappers() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let base = User::new().name(UserName::new("John Doe").unwrap());
    rt.block_on(async {
        let tm = base.parse().field().name().tokio_mutex();
        let guard = tm.0.lock().await;
        assert_eq!(&*guard, "John Doe");
    });
}

#[test]
#[cfg(feature = "unsafecell")]
fn unsafe_cell_wrappers() {
    let base = User::new().name(UserName::new("John Doe").unwrap());
    let uc = base.parse().field().name().unsafe_cell();
    let ptr = uc.0.get();
    unsafe {
        assert_eq!(&*ptr, "John Doe");
    }
}

```

# ❤️ Donate

[![Monero](https://img.shields.io/badge/88NKLkhZf1nTVpaSU6vwG6dwBwb9tFVSM8Lpj3YqdL1PMt8Gm7opV7aUnMYBaAC9Y6a4kfDc3fLGoMVqeSJKNphyLpLdEvC-FF6600?style=flat&logo=monero&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png)
[![Bitcoin](https://img.shields.io/badge/bc1qnlayyh84e9u5pd4m9g9sf4c5zdzswvkmudmdu5-EAB300?style=flat&logo=bitcoin&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png)