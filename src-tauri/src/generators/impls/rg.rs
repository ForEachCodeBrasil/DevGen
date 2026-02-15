use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

pub struct RgGenerator;

impl Generator for RgGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "rg".into(),
            name: "RG (Registro Geral)".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de Registro Geral válido (padrão SP).".into(),
            requires_pro: false,
            options: Some(serde_json::json!({
                "fields": [
                    {
                        "name": "mask",
                        "type": "boolean",
                        "label": "Formatado (Pontuação)",
                        "default": true
                    },
                    {
                        "name": "state",
                        "type": "select",
                        "label": "Estado (UF)",
                        "default": "SP",
                        "options": ["SP", "RJ", "MG", "RS", "PR", "SC", "BA", "CE", "PE", "ES", "GO"]
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
        let digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();

        // Calculate verifier digit (Modulo 11) based on SP standard
        // Weights: 2, 3, 4, 5, 6, 7, 8, 9
        let mut sum = 0;
        for (i, digit) in digits.iter().enumerate() {
            sum += *digit as u32 * (i as u32 + 2);
        }

        let remainder = sum % 11;
        let d1_char = match 11 - remainder {
            10 => 'X',
            11 => '0',
            x => char::from_digit(x, 10).unwrap(),
        };

        let mut rg_str: String = digits.iter().map(|d| d.to_string()).collect();
        rg_str.push(d1_char);

        let formatted = if mask {
            format!(
                "{}.{}.{}-{}",
                &rg_str[0..2],
                &rg_str[2..5],
                &rg_str[5..8],
                &rg_str[8..9]
            )
        } else {
            rg_str
        };

        Ok(GenerateResponse {
            text: Some(formatted),
            metadata: None,
            base64_artifact: None,
        })
    }
}
