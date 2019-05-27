use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;

pub struct TypeInfo<'a> {
    pub any_type: &'a AnyType<'a>,
    pub reference: TypeRef,
}
