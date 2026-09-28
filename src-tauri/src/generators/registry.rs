use super::{GenerateResponse, Generator, GeneratorDefinition, GeneratorError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct GeneratorRegistry {
    generators: Arc<RwLock<HashMap<String, Box<dyn Generator>>>>,
}

impl GeneratorRegistry {
    pub fn new() -> Self {
        Self {
            generators: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register<G: Generator + 'static>(&self, generator: G) {
        let def = generator.definition();
        let mut map = self.generators.write().unwrap();
        map.insert(def.id, Box::new(generator));
    }

    pub fn list(&self) -> Vec<GeneratorDefinition> {
        let map = self.generators.read().unwrap();
        let mut list: Vec<GeneratorDefinition> = map.values().map(|g| g.definition()).collect();
        // Sort by category then name for consistent ordering
        list.sort_by(|a, b| {
            // cheap cheat: debug format of enum works for sorting if distinct
            let cat_cmp = format!("{:?}", a.category).cmp(&format!("{:?}", b.category));
            if cat_cmp == std::cmp::Ordering::Equal {
                a.name.cmp(&b.name)
            } else {
                cat_cmp
            }
        });
        list
    }

    pub fn generate(
        &self,
        id: &str,
        options: serde_json::Value,
    ) -> Result<GenerateResponse, GeneratorError> {
        let map = self.generators.read().unwrap();
        if let Some(gen) = map.get(id) {
            gen.generate(options)
        } else {
            Err(GeneratorError::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::{GeneratorCategory, GeneratorDefinition};
    use serde_json::json;

    struct StubGenerator {
        id: &'static str,
        name: &'static str,
        category: GeneratorCategory,
    }

    impl Generator for StubGenerator {
        fn definition(&self) -> GeneratorDefinition {
            GeneratorDefinition {
                id: self.id.into(),
                name: self.name.into(),
                category: self.category,
                description: "stub".into(),
                options: None,
            }
        }

        fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
            Ok(GenerateResponse {
                text: Some(format!("{}:{}", self.id, options)),
                metadata: None,
                base64_artifact: None,
            })
        }
    }

    #[test]
    fn list_is_sorted_by_category_then_name() {
        let registry = GeneratorRegistry::new();
        registry.register(StubGenerator {
            id: "person_b",
            name: "B",
            category: GeneratorCategory::Person,
        });
        registry.register(StubGenerator {
            id: "documents_a",
            name: "A",
            category: GeneratorCategory::Documents,
        });
        registry.register(StubGenerator {
            id: "person_a",
            name: "A",
            category: GeneratorCategory::Person,
        });

        let ids: Vec<String> = registry.list().into_iter().map(|d| d.id).collect();
        assert_eq!(ids, vec!["documents_a", "person_a", "person_b"]);
    }

    #[test]
    fn generate_returns_not_found_for_unknown_id() {
        let registry = GeneratorRegistry::new();
        let err = registry.generate("missing", json!({})).unwrap_err();
        assert!(matches!(err, GeneratorError::NotFound));
    }

    #[test]
    fn generate_uses_registered_generator() {
        let registry = GeneratorRegistry::new();
        registry.register(StubGenerator {
            id: "echo",
            name: "Echo",
            category: GeneratorCategory::Utilities,
        });

        let response = registry
            .generate("echo", json!({ "mask": true }))
            .expect("registered generator should execute");

        assert_eq!(response.text.as_deref(), Some("echo:{\"mask\":true}"));
    }
}
