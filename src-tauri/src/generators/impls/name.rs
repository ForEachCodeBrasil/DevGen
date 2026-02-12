use crate::datasets::Datasets;
use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use serde_json::json;

pub struct NameGenerator;

impl Generator for NameGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "name".into(),
            name: "Nome de Pessoa".into(),
            category: GeneratorCategory::Person,
            description: "Gera nomes completos de pessoas.".into(),
            options: Some(json!({
                "fields": [
                    {
                        "name": "count",
                        "type": "number",
                        "label": "Quantidade",
                        "default": 1
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let count = options
            .get("count")
            .and_then(|v| v.as_u64())
            .unwrap_or(1)
            .min(100) as usize;

        let mut names = Vec::with_capacity(count);
        for _ in 0..count {
            names.push(Datasets::random_name());
        }

        let text = names.join(
            "
",
        );

        Ok(GenerateResponse {
            text: Some(text),
            metadata: Some(json!({ "names": names })),
        })
    }
}
