use liquesco_schema::any_type::AnyType;

pub(crate) fn type_description(any_type: &AnyType) -> (&'static str, &'static str) {
    match any_type {
            AnyType::Struct(_) => ("structure", "A structure (aka struct) contains 0-n fields. The fields do not need to be of the same type."),
            AnyType::UInt(_) => ("unsigned integer", "Data of an unsigned integer (aka uint or unsigned int) holds a single (positive) integer value (within a defined range)."),
            AnyType::SInt(_) => ("signed integer", "Data of signed integer (aka sint or signed int) holds a single (positive or negative) integer value (within a defined range)."),
            AnyType::Ascii(_) => ("ascii", "Ascii (aka ascii text) is a sequence of characters where each of them is within the ascii range (0-127 inclusive). It can be used to transfer technical text (aka string); it's not to be used to transfer human readable text (use unicode for this case)."),
            AnyType::Bool(_) => ("boolean", "Data of type boolean (aka bool) can hold the value 'true' or the value 'false' (1 or 0; on or off; enabled or disabled). It's like a single bit of information. As an alternative you an also use an enum with 2 variants (it's usually better suited in most cases)"),
            AnyType::Enum(_) => ("enumeration", "An enumeration (aka enum; tagged union; variant record; discriminated union) contains 1-n variants; Each variant can (optionally) have a value."),
            AnyType::Anchors(_) => ("anchors", "Anchors - in combination with references - can be used to describe recursive data. Anchors are essentially a sequence of 1-n items. The items in that sequence can be referenced using references."),
            AnyType::Reference(_) => ("reference", "References - in combination with anchors - can be used to describe recursive data. Technically references are just integers that reference one item in the anchors sequence by index."),
            AnyType::Seq(_) => ("sequence", "A sequence (aka seq; list; vector; array) describes a sequence of 0-n elements. Unlike struct fields, each element in a sequence has to be of the same type."),
            AnyType::Float32(_) => ("float32", "A IEEE 754 32 bit float number. Do not use this to transfer decimal values."),
            AnyType::Float64(_) => ("float64", "A IEEE 754 64 bit float number. Do not use this to transfer decimal values."),
            AnyType::Option(_) => ("option", "Use the option type (aka maybe; optional; nullable) to describe data that can either be there ('present'; 'some') or absent ('missing'; 'empty'). Alternatively you can also use an enum with two variants to achieve the same."),
            AnyType::Unicode(_) => ("unicode", "The unicode type (aka string) can be used to describe arbitrary human readable text."),
            AnyType::Uuid(_) => ("uuid", "16 byte UUID; RFC 4122."),
            AnyType::Range(_) => ("range", "A range (start - end); start/end with configurable inclusion/exclusion."),
        }
}