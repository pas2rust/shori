# `shori`

[![Crates.io](https://img.shields.io/crates/v/shori.svg)](https://crates.io/crates/shori)
[![Docs.rs](https://docs.rs/shori/badge.svg)](https://docs.rs/shori)
[![License](https://img.shields.io/crates/l/shori.svg)](https://github.com/pas2rust/shori/blob/main/LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/pas2rust/shori?color=orange&logo=rust&style=flat&logoColor=white)
![GitHub stars](https://img.shields.io/github/stars/pas2rust/shori?color=success&style=flat&logo=github)
![GitHub forks](https://img.shields.io/github/forks/pas2rust/shori?color=orange&logo=Furry%20Network&style=flat&logoColor=white)
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
use kenzu::Builder;
use serde::{Deserialize, Serialize};
use shori::Parser;

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

```

# ❤️ Donate

[![Monero](https://img.shields.io/badge/88NKLkhZf1nTVpaSU6vwG6dwBwb9tFVSM8Lpj3YqdL1PMt8Gm7opV7aUnMYBaAC9Y6a4kfDc3fLGoMVqeSJKNphyLpLdEvC-FF6600?style=flat&logo=monero&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png)
[![Bitcoin](https://img.shields.io/badge/bc1qnlayyh84e9u5pd4m9g9sf4c5zdzswvkmudmdu5-EAB300?style=flat&logo=bitcoin&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png)