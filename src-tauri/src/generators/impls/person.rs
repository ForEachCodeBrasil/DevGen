use crate::datasets::Datasets;
use crate::generators::{
    impls::{cpf::CpfGenerator, rg::RgGenerator},
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct PersonGenerator;

impl Generator for PersonGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "person".into(),
            name: "Pessoa Completa".into(),
            category: GeneratorCategory::Person,
            description: "Gera dados completos de uma pessoa (Nome, CPF, RG, Endereço, etc)."
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
        let mask = options
            .get("mask")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let mut rng = rand::thread_rng();

        // 1. Location (Select first for consistency)
        let cities = Datasets::get_cities();
        let city_entry = &cities[rng.gen_range(0..cities.len())];
        let state = &city_entry.state;

        // 2. Name
        let name = Datasets::random_name();

        // 3. Documents
        let cpf_gen = CpfGenerator;
        let rg_gen = RgGenerator;

        let cpf_res = cpf_gen.generate(json!({ "mask": mask, "state": state }))?;
        let rg_res = rg_gen.generate(json!({ "mask": mask, "state": state }))?;

        // 4. Demographics
        let age = rng.gen_range(18..80);

        // 5. Contact
        let email_domains = Datasets::get_email_domains();
        let domain = email_domains[rng.gen_range(0..email_domains.len())];
        let const_part = name.to_lowercase().replace(" ", ".");
        let email = format!("{}@{}", const_part, domain);

        // 6. Address
        let address = Datasets::random_address();
        let cep = Datasets::random_cep();

        // 6. Phone
        let phone = format!(
            "({:02}) 9{:04}-{:04}",
            rng.gen_range(11..99),
            rng.gen_range(0..9999),
            rng.gen_range(0..9999)
        );

        // Construct structured result
        let cpf_text = cpf_res.text.unwrap_or_default();
        let rg_text = rg_res.text.unwrap_or_default();

        let data = json!({
            "Nome": name,
            "Idade": age,
            "CPF": cpf_text,
            "RG": rg_text,
            "E-mail": email,
            "Telefone": phone,
            "CEP": cep,
            "Endereço": address,
            "Cidade": city_entry.city,
            "Estado": city_entry.state,
        });

        // Format as text block for current UI
        let text = format!(
            "Nome: {}\nIdade: {}\nCPF: {}\nRG: {}\nE-mail: {}\nTelefone: {}\n\nCEP: {}\nEndereço: {}\nCidade: {} - {}",
            name, age, cpf_text, rg_text, email, phone, cep, address, city_entry.city, city_entry.state
        );

        Ok(GenerateResponse {
            text: Some(text),
            metadata: Some(data),
            base64_artifact: None,
        })
    }
}
