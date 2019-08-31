# envopt

[![Crates.io](https://img.shields.io/crates/v/envopt.svg)](https://crates.io/crates/envopt)
[![Documentation](https://docs.rs/envopt/badge.svg)](https://docs.rs/envopt/)
![License](https://img.shields.io/crates/l/envopt.svg)

Parse environment variables by defining a struct.

### Example
```rust
use envopt::EnvOpt;

#[derive(EnvOpt)]
pub enum EnvOpts {
    #[envopt(name = "FOO")]
    Foo,
    #[envopt(name = "BAR", default = "default-bar")]
    Bar,
}

pub fn main() {
    EnvOpts::validate_or_exit();

    println!("FOO: {}", EnvOpts::Foo.value_or_exit());
    println!("BAR: {}", EnvOpts::Bar.value_or_exit());
}
```
