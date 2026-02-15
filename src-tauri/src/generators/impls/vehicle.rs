use crate::datasets::Datasets;
use crate::generators::{
    impls::renavam::RenavamGenerator, GenerateResponse, Generator, GeneratorCategory,
    GeneratorDefinition, GeneratorError,
};
use chrono::Datelike;
use rand::Rng;
use serde_json::json;

pub struct VehicleGenerator;

impl Generator for VehicleGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "vehicle".into(),
            name: "Veículo Completo".into(),
            category: GeneratorCategory::Vehicle,
            description: "Gera dados completos de um veículo (Marca, Modelo, Placa, Renavam, etc)."
                .into(),
            requires_pro: false,
            options: Some(json!({
                "fields": [
                    {
                        "name": "mask",
                        "type": "boolean",
                        "label": "Documentos Formatados",
                        "default": true
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let _mask = options
            .get("mask")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let mut rng = rand::thread_rng();

        // 1. Vehicle Brand/Model
        let vehicles = Datasets::get_vehicles();
        let brand_entry = &vehicles[rng.gen_range(0..vehicles.len())];
        let brand = &brand_entry.brand;
        let model = &brand_entry.models[rng.gen_range(0..brand_entry.models.len())];

        // 2. Year (15 years back max)
        let current_year = chrono::Local::now().year();
        let year = rng.gen_range(current_year - 15..=current_year);

        // 3. Color
        let colors = [
            "Branco", "Preto", "Prata", "Cinza", "Vermelho", "Azul", "Bege",
        ];
        let color = colors[rng.gen_range(0..colors.len())];

        // 4. Plate (Mercosul or Old?)
        // Let's generate Mercosul: AAA1A11
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let numbers = "0123456789";

        let l1 = letters.chars().nth(rng.gen_range(0..26)).unwrap();
        let l2 = letters.chars().nth(rng.gen_range(0..26)).unwrap();
        let l3 = letters.chars().nth(rng.gen_range(0..26)).unwrap();
        let n1 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
        let l4 = letters.chars().nth(rng.gen_range(0..26)).unwrap();
        let n2 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();
        let n3 = numbers.chars().nth(rng.gen_range(0..10)).unwrap();

        let plate = format!("{}{}{}{}{}{}{}", l1, l2, l3, n1, l4, n2, n3);

        // 5. Renavam
        let renavam_gen = RenavamGenerator;
        // Renavam usually doesn't mask? Check generator.
        let renavam_res = renavam_gen.generate(json!({}))?;
        let renavam = renavam_res.text.unwrap_or_default();

        // Structured Result
        let data = json!({
            "Marca": brand,
            "Modelo": model,
            "Ano": year,
            "Cor": color,
            "Placa": plate,
            "Renavam": renavam
        });

        // Text format
        let text = format!(
            "Veículo: {} {}\nAno: {}\nCor: {}\nPlaca: {}\nRenavam: {}",
            brand, model, year, color, plate, renavam
        );

        Ok(GenerateResponse {
            text: Some(text),
            metadata: Some(data),
            base64_artifact: None,
        })
    }
}
