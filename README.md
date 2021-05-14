# irmin-rs

A Rust crate for interfacing with [Irmin](https://github.com/mirage/irmin)

## Features

### bindings

The `bindings` feature provides an interface for call into OCaml from Rust in order to
access irmin stores. This should be used when you'd like to embed irmin in your application.

### client

The `client` feature provides a client implementation compatible with [irmin-server](https://github.com/zshipko/irmin-server).
This should be used when you'd like to run the irmin server in another process or a different machine.


