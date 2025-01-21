This crate provides a parser implementation for the [FEF format](https://github.com/jiricekcz/fef-specification)

# Versioning

This crate follows [Semantic Versioning 2.0.0](https://semver.org/). To separate versioning from the standard and avoid situations where multiple versions of this crate would have to be installed in a single project to work with different versions of the standard, this crate provides different modules for different major versions of the standard. These can be enabled by feature flags.

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