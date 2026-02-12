use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct VehiclePlateGenerator;

impl Generator for VehiclePlateGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "vehicle_plate".into(),
            name: "Placa de Veículo".into(),
            category: GeneratorCategory::Vehicle,
            description: "Gera placas de veículos nos formatos Antigo e Mercosul.".into(),
            options: Some(json!({
                "fields": [
                    {
                        "name": "model",
                        "type": "select",
                        "label": "Modelo",
                        "default": "Mercosul",
                        "options": ["Mercosul", "Antigo"]
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let model = options
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("Mercosul");

        let mut rng = rand::thread_rng();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let numbers = "0123456789";

        let l1 = letters.chars().nth(rng.gen_range(0..26)).unwrap();
        let l2 = letters.chars().nth(rng.gen_range(0..26)).unwrap();
        let l3 = letters.chars().nth(rng.gen_range(0..26)).unwrap();

        let plate = if model == "Antigo" {
            let n1 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
            let n2 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
            let n3 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
            let n4 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
            format!("{}{}{}-{}{}{}{}", l1, l2, l3, n1, n2, n3, n4)
        } else {
            let n1 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
            let l4 = letters.chars().nth(rng.gen_range(0..26)).unwrap();
            let n2 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
            let n3 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
            format!("{}{}{}{}{}{}{}", l1, l2, l3, n1, l4, n2, n3)
        };

        Ok(GenerateResponse {
            text: Some(plate),
            metadata: None,
            base64_artifact: None,
        })
    }
}
