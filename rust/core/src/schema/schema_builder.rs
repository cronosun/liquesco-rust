use crate::schema::any_type::AnyType;
use crate::schema::core::TypeRef;
use crate::schema::doc_type::DocType;
use crate::schema::structure::TStruct;

pub trait SchemaBuilder {
    fn add<T : Into<AnyType<'static>>>(&mut self, item : T) -> TypeRef;
}

/// Something that can build its own schema.
pub trait BuildsOwnSchema {
    fn build_schema<B>(builder : &mut B) -> TypeRef where B : SchemaBuilder;
}

/// A base type (a single type, not the any type) that can build its own schema. Note:
/// It's always a struct.
pub trait BaseTypeSchemaBuilder {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
        where
            B: SchemaBuilder;
}