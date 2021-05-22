# irmin-rs

A Rust crate for interfacing with [irmin](https://github.com/mirage/irmin)

- Compatibility with [repr](https://github.com/mirage/repr)'s binary encoding using `irmin::Type`
- Embed irmin using direct bindings to the OCaml library
  * Enable the `bindings` feature at compile time
- A client implementation for [irmin-server](https://github.com/zshipko/irmin-server)
  * Enable the `client` feature at compile time

