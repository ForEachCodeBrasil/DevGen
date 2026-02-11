use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct PasswordGenerator;

impl Generator for PasswordGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "password".into(),
            name: "Senha".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera senhas seguras com letras, números e símbolos.".into(),
            options: Some(json!({
                "fields": [
                    {
                        "name": "length",
                        "type": "number", // Needs support in UI or fallback to text
                        "label": "Tamanho",
                        "default": 16
                    },
                    {
                        "name": "uppercase",
                        "type": "boolean",
                        "label": "Maiúsculas (A-Z)",
                        "default": true
                    },
                    {
                        "name": "lowercase",
                        "type": "boolean",
                        "label": "Minúsculas (a-z)",
                        "default": true
                    },
                    {
                        "name": "numbers",
                        "type": "boolean",
                        "label": "Números (0-9)",
                        "default": true
                    },
                    {
                        "name": "symbols",
                        "type": "boolean",
                        "label": "Símbolos (!@#...)",
                        "default": true
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let length = options.get("length").and_then(|v| v.as_u64()).unwrap_or(16) as usize;
        let use_upper = options
            .get("uppercase")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let use_lower = options
            .get("lowercase")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let use_numbers = options
            .get("numbers")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let use_symbols = options
            .get("symbols")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        if !use_upper && !use_lower && !use_numbers && !use_symbols {
            return Err(GeneratorError::InvalidOptions(
                "Selecione pelo menos um tipo de caractere.".into(),
            ));
        }

        let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let lower = "abcdefghijklmnopqrstuvwxyz";
        let numbers = "0123456789";
        let symbols = "!@#$%^&*()_+-=[]{}|;:,.<>?";

        let mut charset = String::new();
        if use_upper {
            charset.push_str(upper);
        }
        if use_lower {
            charset.push_str(lower);
        }
        if use_numbers {
            charset.push_str(numbers);
        }
        if use_symbols {
            charset.push_str(symbols);
        }

        let mut rng = rand::thread_rng();
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect();

        Ok(GenerateResponse {
            text: Some(password),
            metadata: None,
        })
    }
}
