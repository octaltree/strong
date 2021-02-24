# STRONG &emsp; [![crates.io](https://img.shields.io/crates/v/strong)](https://crates.io/crates/strong) ![MIT](https://img.shields.io/crates/l/strong)
<p><strong>Strongly typed String for Rust</strong></p>

Rust is a statically and strongly typed systems programming language.  We can create a new type wrapping primitives.
```rust
struct Age(i32);
struct Email(String);
```
There is a problem here, there are two types of strings in Rust, and it is hard to create strong types for both `String` and `&str`.

STRONG provides two types owned `StrongBuf` and unsized `Strong`.
```rust
use strong::{validators::Email, Strong, StrongBuf, Validator};

fn login(email: &Strong<Email>, password: &Strong<Password>) { .. }

let email: StrongBuf<Email> = ..
let password: StrongBuf<Password> = ..
login(&email, &password);
```
`Email` requires `some_validators` feature.

## Getting Started
```rust
use strong::{validators::Email, Strong, StrongBuf, Validator};

enum Password {}
impl Validator for Password {
    type Err = std::convert::Infallible;
}

let email: StrongBuf<Email> = StrongBuf::<Email>::validate("a@example.com".into()).unwrap();
let password: &Strong<Password> = Strong::<Password>::validate("b").unwrap();
```

### Shorthand
With `shorthand` feature, `Str` and `S` are exported and can be substituted for `StrongBuf` and `Strong`.
```rust
let email: StrongBuf<Email> = StrongBuf::validate("foo".to_string()).unwrap();
let email: Str<Email> = Str::validate("foo".to_string()).unwrap();
```

## License
Licensed under MIT license ([LICENSE-MIT](LICENSE) or https://opensource.org/licenses/MIT)

### Contributing
welcome!
