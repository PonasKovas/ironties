use std::collections::{HashMap, HashSet};

use crate::CType;

pub struct CDefinitions {
    pub includes: HashSet<String>,
    // type name (from std::any::type_name) -> definition
    pub types: HashMap<String, String>,
}

impl CDefinitions {
    pub fn new() -> Self {
        Self {
            includes: HashSet::new(),
            types: HashMap::new(),
        }
    }
    pub fn render(self) -> String {
        let mut result = String::new();

        for include in self.includes {
            result += &format!("#include <{include}>\n");
        }

        for ty in self.types {
            result += &(ty.1 + "\n");
        }

        result
    }
    pub fn extend_once<T: CType>(&mut self) {
        if !self.types.contains_key(std::any::type_name::<T>()) {
            T::_definitions(self)
        }
    }
}
