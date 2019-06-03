use liquesco_schema::core::TypeRef;
use std::collections::HashMap;
use std::collections::HashSet;

/// What a type uses (references); What's using the type?
pub struct Usage {
    is_used_by: HashMap<TypeRef, HashSet<TypeRef>>,
    uses: HashMap<TypeRef, HashSet<TypeRef>>,
}

impl Default for Usage {
    fn default() -> Self {
        Usage {
            is_used_by: HashMap::default(),
            uses: HashMap::default(),
        }
    }
}

lazy_static! {
    static ref EMPTY_SET: HashSet<TypeRef> = { HashSet::with_capacity(0) };
}

impl Usage {
    pub fn set_uses(&mut self, myself: &TypeRef, uses: &TypeRef) {
        // uses
        let mut current = if let Some(current) = self.uses.remove(&myself) {
            current
        } else {
            HashSet::default()
        };
        current.insert(uses.clone());
        self.uses.insert(myself.clone(), current);

        // and the reverse
        let mut used_by = if let Some(used_by) = self.is_used_by.remove(&uses) {
            used_by
        } else {
            HashSet::default()
        };
        used_by.insert(myself.clone());
        self.is_used_by.insert(uses.clone(), used_by);
    }

    pub fn uses(&self, type_ref: &TypeRef) -> &HashSet<TypeRef> {
        if let Some(uses) = self.uses.get(type_ref) {
            uses
        } else {
            &EMPTY_SET
        }
    }

    pub fn is_used_by(&self, type_ref: &TypeRef) -> &HashSet<TypeRef> {
        if let Some(uses) = self.is_used_by.get(type_ref) {
            uses
        } else {
            &EMPTY_SET
        }
    }
}
