use crate::datasets::Datasets;
use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use serde_json::json;

pub struct CepGenerator;

impl Generator for CepGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "cep".into(),
            name: "CEP".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um Código de Endereçamento Postal (CEP) brasileiro.".into(),
            options: Some(json!({
                "fields": [
                    {
                        "name": "mask",
                        "type": "boolean",
                        "label": "Formatado",
                        "default": true
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let mask = options
            .get("mask")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let cep = Datasets::random_cep();
        let final_cep = if mask { cep } else { cep.replace("-", "") };

        Ok(GenerateResponse {
            text: Some(final_cep),
            metadata: None,
        })
    }
}
