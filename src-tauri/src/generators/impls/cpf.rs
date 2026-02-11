use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

pub struct CpfGenerator;

impl Generator for CpfGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "cpf".into(),
            name: "CPF".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de Cadastro de Pessoas Físicas válido.".into(),
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
        let mut digits: Vec<u8> = (0..9).map(|_| rng.gen_range(0..10)).collect();

        // Calculate first verifier digit
        let mut sum: u32 = 0;
        for (i, digit) in digits.iter().enumerate() {
            sum += *digit as u32 * (10 - i as u32);
        }
        let remainder = sum % 11;
        let d1 = if remainder < 2 { 0 } else { 11 - remainder };
        digits.push(d1 as u8);

        // Calculate second verifier digit
        let mut sum: u32 = 0;
        for (i, digit) in digits.iter().enumerate() {
            sum += *digit as u32 * (11 - i as u32);
        }
        let remainder = sum % 11;
        let d2 = if remainder < 2 { 0 } else { 11 - remainder };
        digits.push(d2 as u8);

        let cpf_str: String = digits.iter().map(|d| d.to_string()).collect();

        let formatted = if mask {
            format!(
                "{}.{}.{}-{}",
                &cpf_str[0..3],
                &cpf_str[3..6],
                &cpf_str[6..9],
                &cpf_str[9..11]
            )
        } else {
            cpf_str
        };

        Ok(GenerateResponse {
            text: Some(formatted),
            metadata: None,
        })
    }
}
