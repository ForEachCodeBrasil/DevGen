use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

pub struct TituloEleitorGenerator;

impl Generator for TituloEleitorGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "titulo_eleitor".into(),
            name: "Título de Eleitor".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de Título de Eleitor válido.".into(),
            options: Some(serde_json::json!({
                "fields": []
            })),
        }
    }

    fn generate(&self, _options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let mut rng = rand::thread_rng();
        // 8 sequential digits
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();

        // 2 digits for UF (01 to 28)
        let uf = rng.gen_range(1..29);
        digits.push((uf / 10) as u8);
        digits.push((uf % 10) as u8);

        // DV1: First 8 digits. Weights 2..9
        let mut sum = 0;
        for (i, digit) in digits.iter().take(8).enumerate() {
            sum += *digit as u32 * (i as u32 + 2);
        }
        let remainder = sum % 11;
        let dv1 = if remainder == 10 { 0 } else { remainder };

        digits.push(dv1 as u8);

        // DV2: UF + DV1 (last 3 chars: d8, d9, d10)
        let d8 = digits[8] as u32;
        let d9 = digits[9] as u32;
        let d10 = digits[10] as u32;

        // Standard rule: d8*7 + d9*8 + d10*9
        let sum = d8 * 7 + d9 * 8 + d10 * 9;
        let remainder = sum % 11;
        let dv2 = if remainder == 10 { 0 } else { remainder };
        digits.push(dv2 as u8);

        let titulo_str: String = digits.iter().map(|d| d.to_string()).collect();

        Ok(GenerateResponse {
            text: Some(titulo_str),
            metadata: None,
            base64_artifact: None,
        })
    }
}
