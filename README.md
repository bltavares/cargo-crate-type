# cargo crate-type

Edits the lib crate type to help building cross-platform libs, by modifying Cargo.toml

---

This command `cargo` subcommand is exclusively intended to be used to help with [working arround](https://github.com/rust-lang/rust/issues/51009) how crate types are defined, in order to help with cross-platform builds.

It is currently [not possible](https://github.com/rust-lang/cargo/issues/6160) to define a single `crate-type` override on `cargo build`, which causes libs intended to be used on other languages to compile more than one type of crate.

With this command, the `Cargo.toml` will be rewriten top include only the specific needed crate type, either it being dynamic or static.

```
cargo install cargo-crate-type

cargo crate-type [dynamic|static]
```

**Beware:** This command will alter your `Cargo.toml`

Intended to be used on [crossgen](https://github.com/yoshuawuyts/crossgen)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.