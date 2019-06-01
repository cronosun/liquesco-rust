use crate::type_info::TypeInfo;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;
use liquesco_schema::schema_builder::SchemaBuilder;
use std::collections::HashMap;

pub trait SchemaReader {
    fn master_ref(&self) -> TypeRef;
    fn require(&self, reference: TypeRef) -> &AnyType;

    fn type_info(&self, reference: TypeRef) -> TypeInfo {
        TypeInfo {
            any_type: self.require(reference),
            reference,
        }
    }
}

/// It's at the same time a `SchemaBuilder` and a `SchemaReader`.
pub struct SchemaBuilderReader {
    type_to_ref: HashMap<AnyType<'static>, TypeRef>,
    reference_to_type: HashMap<TypeRef, AnyType<'static>>,
}

impl Default for SchemaBuilderReader {
    fn default() -> Self {
        SchemaBuilderReader {
            type_to_ref: HashMap::new(),
            reference_to_type: HashMap::new(),
        }
    }
}

impl SchemaBuilder for SchemaBuilderReader {
    fn add<T: Into<AnyType<'static>>>(&mut self, item: T) -> TypeRef {
        let any = item.into();
        // store items, this prevents duplicates
        if let Some(reference) = self.type_to_ref.get(&any) {
            *reference
        } else {
            // not yet in map / no duplicate
            let new_reference = TypeRef::new(self.type_to_ref.len() as u32);
            // TODO: Prevent cloning
            self.reference_to_type.insert(new_reference, any.clone());
            self.type_to_ref.insert(any, new_reference);
            new_reference
        }
    }
}

impl SchemaReader for SchemaBuilderReader {
    fn master_ref(&self) -> TypeRef {
        TypeRef::new(self.type_to_ref.len() as u32)
    }

    fn require(&self, reference: TypeRef) -> &AnyType {
        if let Some(any) = self.reference_to_type.get(&reference) {
            any
        } else {
            panic!(format!(
                "Type with reference {:?} not found in schema",
                reference
            ))
        }
    }
}
