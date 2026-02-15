use crate::datasets::Datasets;
use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use serde_json::json;

pub struct NickGenerator;

impl Generator for NickGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "nick".into(),
            name: "Nick / Apelido".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera apelidos / nicknames criativos.".into(),
            requires_pro: false,
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

        let mut nicks = Vec::with_capacity(count);
        for _ in 0..count {
            nicks.push(Datasets::random_nick());
        }

        let text = nicks.join(
            "
",
        );

        Ok(GenerateResponse {
            text: Some(text),
            metadata: Some(json!({ "nicks": nicks })),
            base64_artifact: None,
        })
    }
}
