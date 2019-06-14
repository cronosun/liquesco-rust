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
 * **Balanced 1**: It's maybe not the format that generates the smallest binaries - but still does not waste much space. It's not a zero-copy deserialization format (like Cap’n Proto) - but still quite efficient; on the other hand de-/serializers are easy to implement.
 * **Balanced 2**: It's not as verbose as JSON (for example struct keys are not encoded); on the other hand you don't need a schema to deserialize the data (like Protocol Buffers); it's possible to decode the data without a schema and most things are human readable (useful for debugging when you don't have the schema).
 * **Tagged Unions**: Unlike most other serialization formats liquesco supports tagged unions (aka. enums, discriminated unions, case classes, variant records).
 * **Extendable (versioning)**: It's possible to add more fields to structs or tagged unions and still be able to decode the data using an old decoder.

### Other features / information

 * **Language independent**: Rust is used for the reference implementation - there's however nothing in the format that's Rust specific.
 * **Machine independent**: It uses little endian to decode data.
 * **Minimal heap memory allocations**: It's possible to serialize any data using no heap memory allocation. For deserialization you might (depending on how you use the data) need heap memory allocation for destination lists and maps.

## Schema

The schema can be used to validate data and to generate source code (generate data classes for example).

The most important feature: It's very cheap to validate data using a given schema. The rust reference implementation proves this:

 * Almost no heap allocations are required to validate data.
 * Maps always have to be sorted by keys; sequences with unique items (aka. sets) always have to be sorted too. This makes validation for uniqueness quite cheap.
 * No regex: There are many regex standards with slight incompatibilities. Regex is expensive to validate.

Other features:

 * **Tagged Unions**: Tagged unions allow you to write powerful schemas.
 * **Canonical form**: Schemas have a canonical form: documentation removed and serialized using the liquesco serialization format.
 * **Documented**: Each type can be documented. Each type can have 0-n GUIDs - using this it's possible to find compatible types across companies or even worldwide.
 * **Recursion**: It's possible to have typed recursive data.
 * **Text format**: Schemas usually are serialized using the liquesco serialization format but it's also possible to deserialize from a human readable text format.

### The liquesco schema itself is using liquesco

Naming:
 
 * **The** liquesco schema: Means the schema you write a schema in. E.g. like the protobuf language.
 * **A** liquesco schema: Means a schema you write - e.g. a protobuf schema written using the protobuf language.

Serialization and validation:

 * The liquesco schema itself can be serialized using liquesco.
 * The liquesco schema (**A**) itself can be validated using liquesco (aginst the built-in schema).
 * A liquesco schema can be serialized using liquesco.
 * A liquesco schema (**B**) can be validated using liquesco (against schema **A**).
 * Data can be serialized using liquesco.
 * Data can be validated using liquesco (against schema **B**).

## Comparison

### Protobuf

**Serialization**: Quite similar, except that the liquesco serialization contains more information when de-serializing without schema. It's still possible to understand most when deserializing without schema (for example for debugging). In this respect liquesco serialization resembles json (you still can read json data even when the json schema is missing).

**Schema**: Compared to the protobuf language, the liquesco schema describes data much more precisely, some examples:

 * Integers can be constrained (min, max value).
 * Length of sequences can be constrained.
 * Liquesco knows about sorting & equality of data: So it's possible to define sorted sequences. Sequences with unique items (aka. "set"). Map keys are always sorted (so it's easy to validate uniqueness).
 * The schema does not contain language specific information (like java packages). Since liquesco tries to be as canonical as possible and the liquesco schema itself is written using liquesco this results in quite canonical schemas (so when two schemas result in the same binary data they are equal).
 * Contains some types that are missing in proto buf: Decimal type, ASCII type (with defined character ranges), sets, recursive data structures, option type, UUID type.

### CBOR

**Serialization**: Quite similar except that CBOR is not canonical and has no enum data type.

**Schema**: Missing in CBOR.

### Json + Json-Schema

**Serialization**: Json is a text format, liquesco is a binary format. Except from that both are quite similar except that liquesco has some more types like binary data and enums. Data serialized using liquesco is canonical.

**Schema**

 * Json-Schema is not canonical.
 * Json-Schema contains validation rules that are hard to validate in a non-web environment (like the ECMA 262 regular expression dialect).
 * Json-schema contains some other things (like "Resource identifiers" or host names) that are quite web or internet specific while liquesco is independent of the web.
 * Json-schema cannot contain arbitrary types as map keys.