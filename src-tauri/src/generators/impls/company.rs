use crate::generators::{
    impls::{cnpj::CnpjGenerator, ie::InscricaoEstadualGenerator},
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use crate::datasets::Datasets;
use rand::Rng;
use serde_json::json;

pub struct CompanyGenerator;

impl Generator for CompanyGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "company".into(),
            name: "Empresa Completa".into(),
            category: GeneratorCategory::Company,
            description: "Gera dados completos de uma empresa (Razão Social, CNPJ, IE, etc).".into(),
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

        // 1. Name
        let surnames = Datasets::get_surnames();
        let name1 = surnames[rng.gen_range(0..surnames.len())];
        let name2 = surnames[rng.gen_range(0..surnames.len())];
        let types = ["Ltda", "S.A.", "ME", "Eireli", "Tecnologia", "Serviços", "Comércio", "Indústria"];
        let company_type = types[rng.gen_range(0..types.len())];
        
        let razao_social = format!("{} & {} {}", name1, name2, company_type);
        let fantasy_name = format!("{} {}", name1, company_type);
        
        // 2. Documents
        let cnpj_gen = CnpjGenerator;
        let ie_gen = InscricaoEstadualGenerator;
        
        let cnpj_res = cnpj_gen.generate(json!({ "mask": mask }))?;
        
        // Pick a random state for IE
        let cities = Datasets::get_cities();
        let city_entry = &cities[rng.gen_range(0..cities.len())];
        
        let ie_res = ie_gen.generate(json!({ "mask": mask, "state": city_entry.state }))?;
        
        // 3. Contact
        let email_domains = Datasets::get_email_domains();
        let domain = email_domains[rng.gen_range(0..email_domains.len())];
        let email = format!("contato@{}", domain); // simplified
        
        // Construct structured result
        let cnpj_text = cnpj_res.text.unwrap_or_default();
        let ie_text = ie_res.text.unwrap_or_default();

        let data = json!({
            "Razão Social": razao_social,
            "Nome Fantasia": fantasy_name,
            "CNPJ": cnpj_text,
            "Inscrição Estadual": ie_text,
            "E-mail": email,
            "Cidade": city_entry.city,
            "Estado": city_entry.state,
        });

        // Format as text block
        let text = format!(
            "Razão Social: {}\nNome Fantasia: {}\nCNPJ: {}\nIE ({}): {}\nE-mail: {}\nCidade: {} - {}",
            razao_social, fantasy_name, 
            cnpj_text, 
            city_entry.state, ie_text, 
            email, 
            city_entry.city, city_entry.state
        );

        Ok(GenerateResponse {
            text: Some(text),
            metadata: Some(data),
        })
    }
}
