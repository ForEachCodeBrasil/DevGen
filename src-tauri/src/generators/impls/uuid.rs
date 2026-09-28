use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct UuidGenerator;

impl Generator for UuidGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "uuid".into(),
            name: "UUID v4".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera um Identificador Único Universal (versão 4).".into(),
            options: Some(json!({
                "fields": [
                    {
                        "name": "uppercase",
                        "type": "boolean",
                        "label": "Maiúsculo",
                        "default": false
                    },
                     {
                        "name": "hyphens",
                        "type": "boolean",
                        "label": "Com Hífens",
                        "default": true
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let uppercase = options
            .get("uppercase")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let hyphens = options
            .get("hyphens")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 16];
        rng.fill(&mut bytes);

        // Variant and Version bits
        bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant 10xxxxxx

        let mut s = String::with_capacity(36);
        for (i, byte) in bytes.iter().enumerate() {
            if hyphens && (i == 4 || i == 6 || i == 8 || i == 10) {
                s.push('-');
            }
            s.push_str(&format!("{:02x}", byte));
        }

        if uppercase {
            s = s.to_uppercase();
        }

        Ok(GenerateResponse {
            text: Some(s),
            metadata: None,
            base64_artifact: None,
        })
    }
}
