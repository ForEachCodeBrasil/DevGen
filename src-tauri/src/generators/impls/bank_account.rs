use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct BankAccountGenerator;

impl Generator for BankAccountGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "bank_account".into(),
            name: "Conta Bancária".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera dados de conta bancária de diversos bancos brasileiros.".into(),
            options: Some(json!({
                "fields": [
                    {
                        "name": "bank",
                        "type": "select",
                        "label": "Banco",
                        "default": "Banco do Brasil",
                        "options": ["Banco do Brasil", "Bradesco", "Itaú", "Santander", "Caixa"]
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let bank = options
            .get("bank")
            .and_then(|v| v.as_str())
            .unwrap_or("Banco do Brasil");

        let mut rng = rand::thread_rng();

        let (bank_name, bank_code, agency, account) = match bank {
            "Banco do Brasil" => {
                let ag = format!("{:04}", rng.gen_range(1..9999));
                let acc = format!("{:08}", rng.gen_range(1..99999999));
                // Simplified DV
                let dv = rng.gen_range(0..10).to_string();
                ("Banco do Brasil", "001", ag, format!("{}-{}", acc, dv))
            }
            "Bradesco" => {
                let ag = format!("{:04}", rng.gen_range(1..9999));
                let acc = format!("{:07}", rng.gen_range(1..9999999));
                let dv = rng.gen_range(0..10).to_string();
                ("Bradesco", "237", ag, format!("{}-{}", acc, dv))
            }
            "Itaú" => {
                let ag = format!("{:04}", rng.gen_range(1..9999));
                let acc = format!("{:05}", rng.gen_range(1..99999));
                let dv = rng.gen_range(0..10).to_string();
                ("Itaú", "341", ag, format!("{}-{}", acc, dv))
            }
            "Santander" => {
                let ag = format!("{:04}", rng.gen_range(1..9999));
                let acc = format!("{:08}", rng.gen_range(1..99999999));
                let dv = rng.gen_range(0..10).to_string();
                ("Santander", "033", ag, format!("{}-{}", acc, dv))
            }
            "Caixa" => {
                let ag = format!("{:04}", rng.gen_range(1..9999));
                let op = "001"; // Pessoa física
                let acc = format!("{:08}", rng.gen_range(1..99999999));
                let dv = rng.gen_range(0..10).to_string();
                (
                    "Caixa Econômica",
                    "104",
                    ag,
                    format!("{} {}-{}", op, acc, dv),
                )
            }
            _ => ("Desconhecido", "000", "0000".into(), "000000-0".into()),
        };

        let data = json!({
            "Banco": bank_name,
            "Código": bank_code,
            "Agência": agency,
            "Conta": account
        });

        let text = format!(
            "Banco: {} ({})
Agência: {}
Conta: {}",
            bank_name, bank_code, agency, account
        );

        Ok(GenerateResponse {
            text: Some(text),
            metadata: Some(data),
        })
    }
}
