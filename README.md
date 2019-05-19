# Liquesco

Liquesco is:

 * A binary de-/serialization format.
 * A schema language that can be used together with the de-/serialization format (but also work with other formats like JSON).

... and is a Rust implementation:

 * A Rust implementation of the de-/serialization format.
 * An Rust implementation that checks liquesco validity given a schema. 
 * ... and includes Serde (https://serde.rs/) support.

# Details

## De-/serialization format

### Main features

 * **Canonical**: There's only one single possible representation of the data. Useful for a content addressed storage (CAS) / caching data by hash key.
 * **Balanced 1**: It's maybe not the format that generates the smallest binaries - but still does not waiste much space. It's not a zero-copy deserialization format (like Cap’n Proto) - but still quite efficient; on the other hand de-/serializers are easy to implement. 
 * **Balanced 2**: It's not as verbose as JSON (for example struct keys are not encoded); on the other hand you don't need a schema to deserialize the data (like Protocol Buffers); it's possible to decode the data without a schema and most things are human readable (useful for debuging when you don't have the schema).
 * **Tagged Unions**: Unlike most other serialization formats liquesco supports tagged unions (aka. enums, discriminated unions, case classes, variant records).
 * **Extendable (versioning)**: It's possible to add more fields to structs or tagged unions and still be able to decode the data using an old decoder.

### Other features / information

 * **Language independent**: Rust is used for the reference implementation - there's however nothing in the format that's Rust specific.
 * **Machine independent**: It uses little endian to decode data.
 * **Minimal heap memory allocations**: It's possible to serialize any data using no heap memory allocation. For deserialization you might (depending on how you use the data) need heap memory allocation for destination lists and maps.

## Schema

The schema can be used to validate data and to generate source code (generate data classes for example).

 * **Tagged Unions**: Tagged unions allow you to write powerful schemas.
 * **Canonical form**: Schemas have a canonical form: documentation removed and serialized using the liquesco serialization format.
 * **Documented [ToDo]**: Each type can be documented. Each type can have 0-n GUIDs - using this it's possible to find compatible types across companies or even worldwide.
 * **Recursion [ToDo]**: It's possible to have typed recursive data.
 * **Text format [ToDo]**: Schemas usually are serialized using the liquesco serialization format but it's also possible to deserialize from a human readable text format: XML. Why XML, isn't XML dead? I don't think so: Unlike most other formats XML has great editor/IDE support given XSD.



