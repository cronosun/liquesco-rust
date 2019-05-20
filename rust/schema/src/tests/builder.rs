use crate::any_type::AnyType;
use crate::core::Type;
use crate::core::TypeContainer;
use crate::core::TypeRef;
use crate::schema::DefaultSchema;

pub struct Builder<'a> {
    types: Vec<AnyType<'a>>,
}

pub fn builder<'a>() -> Builder<'a> {
    Builder::default()
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Self { types: Vec::new() }
    }
}

pub struct Container<'a> {
    types: Vec<AnyType<'a>>,
}

impl<'a> TypeContainer<'a> for Container<'a> {
    fn maybe_type(&self, reference: TypeRef) -> Option<&AnyType<'a>> {
        let len = self.types.len();
        let ref_usize = reference.0 as usize;
        if ref_usize >= len {
            Option::None
        } else {
            Option::Some(&self.types[ref_usize])
        }
    }
}

impl<'a> Builder<'a> {
    pub fn add<T: Into<AnyType<'a>>>(&mut self, r#type: T) -> TypeRef
    where
        AnyType<'a>: std::convert::From<T>,
    {
        let reference = TypeRef(self.types.len() as u32);
        self.types.push(AnyType::from(r#type));
        reference
    }

    pub fn finish<T: Type>(mut self, r#type: T) -> DefaultSchema<'a, Container<'a>>
    where
        AnyType<'a>: std::convert::From<T>,
    {
        let reference = self.add(r#type);
        let container = Container { types: self.types };
        DefaultSchema::new(container, reference)
    }
}
