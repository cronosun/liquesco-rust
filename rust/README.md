# Rust reference implementation

This contains the Rust reference implementation of Liquesco.

 * **core/serialization**: Data de-/serialization.
 * **core/serde**: Serde (https://serde.rs/) support for data de-/serialization.
 * **core/schema**: The Liquesco schema.
 * **text**: Parse data from a textual representation (like json or xml) given a schema.

# Rust version

Should work with Rust 1.34+ (Rust stable as of 2019-05-09).

# Tests

Currently has 100+ tests covering data de-/serialization, Serde support and schema validation.

# State

It's not yet ready to be used in production. It has many missing things and many things to do.
