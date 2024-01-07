# <img src="https://img.shields.io/badge/rs_i18n-0.0.1-orange?style=flat-square&logo=rust&logoColor=%23fff&labelColor=%23DEA584&color=%23DEA584">  <img src="https://img.shields.io/badge/License-MIT-orange?style=flat-square&logoColor=%23fff&labelColor=%2323B898&color=%2323B898">

# rs-i18n

A lightweight and compact I18n library

- author：syf20020816@outlook.com
- createDate：20240106
- updateDate：20240107
- version：0.0.1
- email：syf20020816@outlook.com

## QuickStart

### 1. install dependencies

```bash
cargo add rs-i18n
cargo add lazy_static
```

### 2. write i18n json file

```bash
-- your project
|---- i18n
|       |-- en_US.json
|       |-- zh_CN.json
```

#### en_US.json

```json
{
  "HELLO":"hello",
  "HAPPY":"satisfied",
  "JOY":"good"
}
```

#### zh_CN.json

```json
{
  "HELLO":"你好",
  "HAPPY":"开心",
  "JOY":"欢快"
}
```

### 3. main.rs

```rust
#[macro_use]
extern crate lazy_static;

use rs_i18n::{I18ns, Loader, UseI18n};

// if use absolute path, you should pay attention that the path should write from root
// means :
// It is an absolute path based on the root directory as the standard
lazy_static! {
    static ref LOADER: Loader = Loader::new(Some("./i18n"));
}

fn main() {
    let mut i18n = UseI18n::new(
        &LOADER,
        None,
    );
    // change lang
    // i18n.set_lang(I18ns::ENUS);
    println!("{}", i18n.t("JOY"));
}
```

***

## Use Dev (recommend)

### add strum

```bash
cargo add strum --feature derive
----or----
strum = { version = "0.25", features = ["derive"] }
```

### main.rs

```rust
#[macro_use]
extern crate lazy_static;

use rs_i18n::{I18ns, Loader, UseI18n};
mod i18n_dict;

use i18n_dict::I18nDict;

lazy_static! {
    static ref LOADER: Loader = Loader::new(Some("./i18n"));
}
fn main() {
    // donot use absoulte path
    let mut i18n = UseI18n::new(
        &LOADER,
        Some("E:\\Rust\\try\\rs-i18n-all\\i18n-test\\src\\i18n_dict.rs"),
    );
    println!("{}", i18n.t(&format!("{:?}", I18nDict::HAPPY)));
}

```

## Goals

- [x] translate
- [x] get system lang as default
- [x] dev
- [ ] date format

