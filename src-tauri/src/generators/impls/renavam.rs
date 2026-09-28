use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

pub struct RenavamGenerator;

impl Generator for RenavamGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "renavam".into(),
            name: "Renavam".into(),
            category: GeneratorCategory::Vehicle,
            description: "Gera um código Renavam válido.".into(),
            options: Some(serde_json::json!({
                "fields": []
            })),
        }
    }

    fn generate(&self, _options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let mut rng = rand::thread_rng();
        let mut digits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..10)).collect();

        // Reverse weights 2..3
        // Actually Renavam weights are 2,3,4,5,6,7,8,9,2,3 for first 10 digits
        // Then mod 11.

        // Correct algo (11 digits):
        // First 10 random.
        // Reverse digits.
        // Multiply by 2, 3, 4...
        // Sum mod 11.

        let mut sum = 0;
        // Processing from right to left (excluding check digit placeholder)
        // Digit 0 (leftmost) * 3
        // Digit 1 * 2
        // ... wait, standard is:
        // d0*3 + d1*2 + d2*9 + d3*8 ... ?

        // Let's use the most standard renavam algo:
        // Weights: 3,2,9,8,7,6,5,4,3,2
        let weights = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        for (i, digit) in digits.iter().enumerate() {
            sum += *digit as u32 * weights[i];
        }

        let remainder = sum % 11;
        let dv = if remainder < 2 { 0 } else { 11 - remainder };
        digits.push(dv as u8);

        let renavam_str: String = digits.iter().map(|d| d.to_string()).collect();

        Ok(GenerateResponse {
            text: Some(renavam_str),
            metadata: None,
            base64_artifact: None,
        })
    }
}
