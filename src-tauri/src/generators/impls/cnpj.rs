use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

pub struct CnpjGenerator;

impl Generator for CnpjGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "cnpj".into(),
            name: "CNPJ".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de Cadastro Nacional da Pessoa Jurídica válido.".into(),
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
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();

        // Add 0001 (filial default)
        digits.extend_from_slice(&[0, 0, 0, 1]);

        // Calculate first verifier digit
        let mut sum: u32 = 0;
        let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        for (i, digit) in digits.iter().enumerate() {
            sum += *digit as u32 * weights1[i];
        }
        let remainder = sum % 11;
        let d1 = if remainder < 2 { 0 } else { 11 - remainder };
        digits.push(d1 as u8);

        // Calculate second verifier digit
        let mut sum: u32 = 0;
        let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        for (i, digit) in digits.iter().enumerate() {
            sum += *digit as u32 * weights2[i];
        }
        let remainder = sum % 11;
        let d2 = if remainder < 2 { 0 } else { 11 - remainder };
        digits.push(d2 as u8);

        let cnpj_str: String = digits.iter().map(|d| d.to_string()).collect();

        let formatted = if mask {
            format!(
                "{}.{}.{}/{}-{}",
                &cnpj_str[0..2],
                &cnpj_str[2..5],
                &cnpj_str[5..8],
                &cnpj_str[8..12],
                &cnpj_str[12..14]
            )
        } else {
            cnpj_str
        };

        Ok(GenerateResponse {
            text: Some(formatted),
            metadata: None,
            base64_artifact: None,
        })
    }
}
