use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct RandomNumberGenerator;

impl Generator for RandomNumberGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "random_number".into(),
            name: "Número Aleatório".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera um número aleatório com tamanho configurável.".into(),
            requires_pro: false,
            options: Some(json!({
                "fields": [
                    {
                        "name": "length",
                        "type": "number",
                        "label": "Tamanho",
                        "default": 10
                    },
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
        let length = options
            .get("length")
            .and_then(|v| v.as_u64())
            .unwrap_or(10)
            .min(100) as usize;

        let count = options
            .get("count")
            .and_then(|v| v.as_u64())
            .unwrap_or(1)
            .min(100) as usize;

        let mut rng = rand::thread_rng();
        let mut results = Vec::with_capacity(count);

        for _ in 0..count {
            let number: String = (0..length)
                .map(|_| rng.gen_range(0..10).to_string())
                .collect();
            results.push(number);
        }

        Ok(GenerateResponse {
            text: Some(results.join("\n")),
            metadata: None,
            base64_artifact: None,
        })
    }
}
