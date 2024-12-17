This crate provides a parser implementation for the [FEF format](https://github.com/jiricekcz/fef-specification)

# Versioning

This crate follows [Semantic Versioning 2.0.0](https://semver.org/). It also mimics the version of the FEF specification it implements.
The major and minor version of this crate will always match the major and minor version of the FEF specification it implements.
The patch version of this crate will be increased for bug fixes and other minor changes (also including implementation of micro versions of the FEF specification).

# Adding inherent items - breaking change

This crate doesn't consider implementing any inherent items as a breaking change - it will not increase the major version.
This can however potentially break backwards compatibility, if downstream crates don't use [disambiguation syntax](https://doc.rust-lang.org/reference/expressions/call-expr.html#disambiguating-function-calls) for calling their own implementations of structs provided by this crate or structs that implement traits provided by this crate.

It is generally considered a good practice to use disambiguation syntax if you need to add an impl to a struct from an upstream crate.

Example of bad usage, that we don't guarantee backwards compatibility for:
```rust
use fef::config::Config;

struct MyConfig;

impl Config for MyConfig {
    // ...
}

impl MyConfig {
    pub fn language(&self) -> &str {
        // ...
        # ""
    }
}

let config = MyConfig;

let language = config.language(); // This will break if we add a language method to the Config trait

let language = MyConfig::language(&config); // This will not break and execute your language method
// let language = <MyConfig as Config>::language(&config); // This would execute the language method from the Config trait

# assert_eq!(language, "");
```