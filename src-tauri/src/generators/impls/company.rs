use crate::datasets::Datasets;
use crate::generators::{
    impls::{cnpj::CnpjGenerator, ie::InscricaoEstadualGenerator},
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct CompanyGenerator;

impl Generator for CompanyGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "company".into(),
            name: "Empresa Completa".into(),
            category: GeneratorCategory::Company,
            description: "Gera dados completos de uma empresa (Razão Social, CNPJ, IE, etc)."
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
        let address = Datasets::random_address();
        let cep = Datasets::random_cep();

        // 2. Name
        let surnames = Datasets::get_surnames();
        let name1 = surnames[rng.gen_range(0..surnames.len())];
        let name2 = surnames[rng.gen_range(0..surnames.len())];
        let types = [
            "Ltda",
            "S.A.",
            "ME",
            "Eireli",
            "Tecnologia",
            "Serviços",
            "Comércio",
            "Indústria",
        ];
        let company_type = types[rng.gen_range(0..types.len())];

        let razao_social = format!("{} & {} {}", name1, name2, company_type);
        let fantasy_name = format!("{} {}", name1, company_type);

        // 3. Documents
        let cnpj_gen = CnpjGenerator;
        let ie_gen = InscricaoEstadualGenerator;

        // Prefer alphanumeric CNPJ (RFB 2026); still valid under ASCII−48 DV rules.
        let cnpj_res = cnpj_gen.generate(json!({ "mask": mask, "format": "alphanumeric" }))?;
        let ie_res = ie_gen.generate(json!({ "mask": mask, "state": state }))?;

        // 4. Contact
        let email_domains = Datasets::get_email_domains();
        let domain = email_domains[rng.gen_range(0..email_domains.len())];
        let clean_name = name1.to_lowercase().replace(" ", "");
        let email = format!("contato@{}.{}", clean_name, domain);
        let phone = format!(
            "({:02}) 3{:03}-{:04}",
            rng.gen_range(11..99),
            rng.gen_range(0..999),
            rng.gen_range(0..9999)
        );

        // Construct structured result
        let cnpj_text = cnpj_res.text.unwrap_or_default();
        let ie_text = ie_res.text.unwrap_or_default();

        let data = json!({
            "Razão Social": razao_social,
            "Nome Fantasia": fantasy_name,
            "CNPJ": cnpj_text,
            "Inscrição Estadual": ie_text,
            "Telefone": phone,
            "E-mail": email,
            "CEP": cep,
            "Endereço": address,
            "Cidade": city_entry.city,
            "Estado": state,
        });

        // Format as text block
        let text = format!(
            "Razão Social: {}\nNome Fantasia: {}\nCNPJ: {}\nIE ({}): {}\nTelefone: {}\nE-mail: {}\n\nCEP: {}\nEndereço: {}\nCidade: {} - {}",
            razao_social,
            fantasy_name,
            cnpj_text,
            state,
            ie_text,
            phone,
            email,
            cep,
            address,
            city_entry.city,
            state
        );

        Ok(GenerateResponse {
            text: Some(text),
            metadata: Some(data),
            base64_artifact: None,
        })
    }
}
