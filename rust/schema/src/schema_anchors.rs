
use crate::schema_builder::SchemaBuilder;
use crate::core::TypeRef;
use crate::any_type::AnyType;
use crate::anchors::TAnchors;
use crate::doc_type::DocType;
use crate::schema_builder::BuildsOwnSchema;

/// This is the base of a liquesco schema.
pub struct SchemaAnchors;

impl BuildsOwnSchema for SchemaAnchors {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder,
    {
        let any_type = AnyType::build_schema(builder);
        builder.add(
            DocType::from(TAnchors::new(any_type, any_type))
                .with_name_unwrap("schema_anchors")
                .with_description(
                    "This anchors is the base of any liquesco schema. It \
                     contains at least one type. It can contain more types that can be referenced \
                     to create recursive types.",
                ),
        )
    }
}