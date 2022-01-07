# irmin-rs

[irmin](https://irmin.org) bindings for Rust

This crate enables you to call directly into irmin from your Rust application and
can be used to open an existing irmin store from Rust that may have been created
by an application written in OCaml.

## Building

After installing [libirmin](https://github.com/mirage/irmin) using opam, you can run:

```
$ cargo build
```

And the build script should be able to find the location of the `libirmin` library and header files.

## Testing

Tests must be executed using a single thread:

```
$ cargo test -- --test-threads=1
```

or

```
$ make test
```
