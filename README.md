# Configify

`IDK` is a simple Rust macro to generate configuration structs from environment variables.  
It supports default values, required variables, and uses `once_cell` for lazy static access.

## Usage

Add to `Cargo.toml`:

```toml
idk = "0.1.0"
```
```rust
use configify::create_config_struct;

create_config_struct! {
    #[derive(Debug, Clone)]
    pub struct Config {
        pub port: u16 => "PORT", @default 8000,
        pub db_url: String => "DATABASE_URL", @expect "DATABASE_URL must be set",
        pub secret: String => "SECRET", @default "empty_secret".to_string()
    }
}

fn main() {
    println!("{:#?}", *Config::DEFAULT);
}
```

Features

    Lazy initialization via once_cell::sync::Lazy
    Support for dotenv files
    Default values or required variables with custom error messages



