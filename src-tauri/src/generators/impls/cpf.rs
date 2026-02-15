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

        let count = options
            .get("count")
            .and_then(|v| v.as_u64())
            .unwrap_or(1)
            .min(100) as usize;

        let mut results = Vec::with_capacity(count);

        for _ in 0..count {
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

            results.push(formatted);
        }

        Ok(GenerateResponse {
            text: Some(results.join("\n")),
            metadata: None,
            base64_artifact: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn is_valid_cpf_digits(cpf: &str) -> bool {
        if cpf.len() != 11 || !cpf.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        let digits: Vec<u8> = cpf
            .chars()
            .map(|c| c.to_digit(10).expect("digit") as u8)
            .collect();

        let mut sum_d1: u32 = 0;
        for (i, digit) in digits.iter().take(9).enumerate() {
            sum_d1 += *digit as u32 * (10 - i as u32);
        }
        let rem_d1 = sum_d1 % 11;
        let expected_d1 = if rem_d1 < 2 { 0 } else { 11 - rem_d1 } as u8;

        let mut sum_d2: u32 = 0;
        for (i, digit) in digits.iter().take(10).enumerate() {
            sum_d2 += *digit as u32 * (11 - i as u32);
        }
        let rem_d2 = sum_d2 % 11;
        let expected_d2 = if rem_d2 < 2 { 0 } else { 11 - rem_d2 } as u8;

        digits[9] == expected_d1 && digits[10] == expected_d2
    }

    #[test]
    fn generates_masked_cpf_by_default() {
        let response = CpfGenerator
            .generate(json!({}))
            .expect("cpf generation should succeed");
        let text = response.text.expect("cpf response should include text");

        assert_eq!(text.len(), 14);
        assert_eq!(text.chars().nth(3), Some('.'));
        assert_eq!(text.chars().nth(7), Some('.'));
        assert_eq!(text.chars().nth(11), Some('-'));
    }

    #[test]
    fn generates_multiple_unmasked_valid_cpfs() {
        let response = CpfGenerator
            .generate(json!({ "mask": false, "count": 5 }))
            .expect("cpf generation should succeed");
        let text = response.text.expect("cpf response should include text");

        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 5);
        assert!(lines.iter().all(|cpf| is_valid_cpf_digits(cpf)));
    }

    #[test]
    fn caps_count_at_100() {
        let response = CpfGenerator
            .generate(json!({ "mask": false, "count": 999 }))
            .expect("cpf generation should succeed");
        let text = response.text.expect("cpf response should include text");

        assert_eq!(text.lines().count(), 100);
    }
}
