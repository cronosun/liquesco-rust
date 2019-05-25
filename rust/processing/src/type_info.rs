use liquesco_schema::core::TypeRef;
use liquesco_schema::any_type::AnyType;

pub struct TypeInfo<'a> {
    pub any_type : &'a AnyType<'a>,
    pub reference : TypeRef,
}