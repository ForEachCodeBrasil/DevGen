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
            Err(GeneratorError {
                message: format!("Generator not found: {}", id),
            })
        }
    }
}
