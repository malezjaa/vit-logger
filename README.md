# `vit_logger`

Easy to use logger for your rust projects.

## Installation

```shell
cargo add vit_logger
```

## Usage

```rust
fn main() {
    std::env::set_var("RUST_LOG", "trace");
    VitLogger::new().init(Config::builder());
    log::info!("Hello, world!");
}
```

## License

[MIT](LICENSE) © [malezjaa](https://github.com/malezjaa)