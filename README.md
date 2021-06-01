# irmin-rs

A Rust crate for interfacing with [irmin](https://github.com/mirage/irmin)

- Compatibility with [repr](https://github.com/mirage/repr)'s binary encoding using `irmin::Type`
- Embed irmin using direct bindings to the OCaml library
  * Enable the `bindings` feature at compile time
- A client implementation for [irmin-server](https://github.com/zshipko/irmin-server)
  * Enable the `client` feature at compile time


## irmin-server client

The `irmin-server` client provides bindings to communicate directly with `irmin-server` instances,
this allows you to run a single instance of irmin that can be using a mix of Rust and OCaml clients.

## Embedding native irmin bindings in Rust

Embedding irmin bindings allows you to call directly into irmin from your Rust application. For example,
this would be more useful in the case that you would like to open an existing irmin store from Rust
that may have been created by an application written in OCaml.
