use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

pub struct PisGenerator;

impl Generator for PisGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "pis".into(),
            name: "PIS/PASEP".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de PIS/PASEP válido.".into(),
            options: Some(serde_json::json!({
                "fields": [
                   {
                        "name": "mask",
                        "type": "boolean",
                        "label": "Formatado (Pontuação)",
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

        let mut rng = rand::thread_rng();
        let mut digits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..10)).collect();

        // Weights: 3, 2, 9, 8, 7, 6, 5, 4, 3, 2
        let weights = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let mut sum = 0;
        for (i, digit) in digits.iter().enumerate() {
            sum += *digit as u32 * weights[i];
        }

        let remainder = sum % 11;
        let dv = if remainder < 2 { 0 } else { 11 - remainder };
        digits.push(dv as u8);

        let pis_str: String = digits.iter().map(|d| d.to_string()).collect();

        let formatted = if mask {
            format!(
                "{}.{}.{}-{}",
                &pis_str[0..3],
                &pis_str[3..8],
                &pis_str[8..10],
                &pis_str[10..11]
            )
        } else {
            pis_str
        };

        Ok(GenerateResponse {
            text: Some(formatted),
            metadata: None,
        })
    }
}
