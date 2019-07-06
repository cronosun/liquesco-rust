Note, this document is about the rust implementation, read [Liquesco](README-LIQUESCO.md) if you want to know more about liquesco itself.

[![Build Status](https://travis-ci.org/cronosun/liquesco-rust.svg?branch=master)](https://travis-ci.org/cronosun/liquesco-rust)

# Rust reference implementation

This contains the Rust reference implementation of liquesco.

 * **serialization**: Data de-/serialization without Serde (https://serde.rs/) and with Serde; 
 * **schema**: The liquesco schema. Schema validation.
 * **parsing**: Parse data from a textual representation (currently yaml) given a schema.
 * **gen-doc**: Generates documentation. Example documentation (for the schema schema): [Example Schema Doc](https://cronosun.github.io/liquesco-rust/doc/SCHEMA.html)
 
# Rust version

Should work with Rust 1.34+ (Rust stable as of 2019-05-09).

# Tests

Currently has 150+ tests covering data de-/serialization, Serde support and schema validation.

# Notable tests

## `parsing/tests/schemas`

 * Parses a custom schema to binary. Validates that custom schema against the liquesco schema.
 * Then parses data that conforms to the given custom schema (to binary). Then validates that data against the given custom schema.

## `schema/tests/self_schema`

This validates the built-in liquesco schema against itself.

## `gen-doc/src/tests`

Takes the liquesco schema and prints the generated ascii doc to std out. Use

```shell
cargo test -- --nocapture
```

to see the generated HTML in the console. Note: With slight modifications it's possible to generate documentation for your custom schema.

# State

It's not yet ready to be used in production. It has many missing things and many things to do.
