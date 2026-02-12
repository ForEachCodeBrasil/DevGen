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
                    },
                    {
                        "name": "state",
                        "type": "select",
                        "label": "Estado (UF)",
                        "default": "Aleatório",
                        "options": [
                            "Aleatório", "RS", "DF", "GO", "MT", "MS", "TO", "AC", "AM", "AP", "PA", "RO", "RR",
                            "CE", "MA", "PI", "AL", "PB", "PE", "RN", "BA", "SE", "MG", "ES", "RJ", "SP", "PR", "SC"
                        ]
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
        let mask = options
            .get("mask")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let state_opt = options
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("Aleatório");

        let count = options
            .get("count")
            .and_then(|v| v.as_u64())
            .unwrap_or(1)
            .min(100) as usize;

        let mut results = Vec::with_capacity(count);

        for _ in 0..count {
            let mut rng = rand::thread_rng();
            let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();

            // 9th digit defines the region:
            let ninth_digit = match state_opt {
                "RS" => 0,
                "DF" | "GO" | "MT" | "MS" | "TO" => 1,
                "AC" | "AM" | "AP" | "PA" | "RO" | "RR" => 2,
                "CE" | "MA" | "PI" => 3,
                "AL" | "PB" | "PE" | "RN" => 4,
                "BA" | "SE" => 5,
                "MG" => 6,
                "ES" | "RJ" => 7,
                "SP" => 8,
                "PR" | "SC" => 9,
                _ => rng.gen_range(0..10),
            };
            digits.push(ninth_digit as u8);

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

            results.push(formatted);
        }

        Ok(GenerateResponse {
            text: Some(results.join("\n")),
            metadata: None,
            base64_artifact: None,
        })
    }
}
