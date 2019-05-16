use crate::schema::core::Type;
use crate::schema::core::TypeContainer;
use crate::schema::core::TypeRef;
use crate::schema::schema::DefaultSchema;
use crate::schema::any_type::AnyType;

pub struct Builder<'a> {
    types: Vec<AnyType<'a>>,
}

pub fn builder<'a>() -> Builder<'a> {
    Builder::default()
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Self {
            types: Vec::new(),
        }
    }
}

pub struct Container<'a> {
    types: Vec<AnyType<'a>>,
}

impl<'a> TypeContainer<'a> for Container<'a> {
    fn maybe_type(&self, reference: TypeRef) -> Option<&AnyType<'a>> {
        let len = self.types.len();
        if reference.0 >= len {
            Option::None
        } else {
            Option::Some(&self.types[reference.0])
        }
    }
}

impl<'a> Builder<'a> {
    pub fn add<T: Type>(&mut self, r#type: T) -> TypeRef
    where AnyType<'a>: std::convert::From<T> {
        let reference = TypeRef(self.types.len());
        self.types.push(AnyType::from(r#type));
        reference
    }

    pub fn finish<T: Type>(
        mut self,
        r#type: T,
    ) -> DefaultSchema<'a, Container<'a>>
    where AnyType<'a>: std::convert::From<T> {
        let reference = self.add(r#type);
        let container = Container {
            types: self.types,
        };
        DefaultSchema::new(container, reference)
    }
}
