use crate::generators::{
    impls::person::PersonGenerator, GenerateResponse, Generator, GeneratorCategory,
    GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct CurriculumGenerator;

impl Generator for CurriculumGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "curriculum".into(),
            name: "Gerador de Currículo",
            category: GeneratorCategory::Person,
            description: "Gera um currículo básico em Markdown.".into(),
            options: Some(json!({
                "fields": [
                    {
                        "name": "language",
                        "type": "select",
                        "label": "Idioma",
                        "default": "Português",
                        "options": ["Português", "English"]
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let lang = options
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("Português");

        let mut rng = rand::thread_rng();
        let person_gen = PersonGenerator;
        let person_res = person_gen.generate(json!({ "mask": true }))?;
        let data = person_res.metadata.unwrap_or(json!({}));

        let name = data["Nome"].as_str().unwrap_or("João Silva");
        let age = data["Idade"].as_u64().unwrap_or(25);
        let email = data["E-mail"].as_str().unwrap_or("joao@exemplo.com");
        let phone = data["Telefone"].as_str().unwrap_or("(11) 99999-9999");
        let city = data["Cidade"].as_str().unwrap_or("São Paulo");
        let state = data["Estado"].as_str().unwrap_or("SP");

        let (obj_title, exp_title, edu_title, skill_title) = if lang == "English" {
            ("Objective", "Experience", "Education", "Skills")
        } else {
            ("Objetivo", "Experiência", "Educação", "Habilidades")
        };

        let objectives = if lang == "English" {
            vec![
                "Seeking a position as a Software Developer to leverage my technical skills.",
                "To obtain a challenging position in a growth-oriented company.",
                "Looking to apply my expertise in a dynamic work environment.",
            ]
        } else {
            vec![
                "Busco uma posição como Desenvolvedor de Software para aplicar meus conhecimentos técnicos.",
                "Obter uma posição desafiadora em uma empresa orientada ao crescimento.",
                "Desejo aplicar minha expertise em um ambiente de trabalho dinâmico.",
            ]
        };

        let roles = if lang == "English" {
            vec![
                "Fullstack Developer",
                "Backend Engineer",
                "Frontend Developer",
                "Analyst",
            ]
        } else {
            vec![
                "Desenvolvedor Fullstack",
                "Engenheiro de Backend",
                "Desenvolvedor Frontend",
                "Analista",
            ]
        };

        let companies = vec![
            "Tech Solutions",
            "Innovate Corp",
            "Soft Systems",
            "Global Dev",
        ];

        let objective = objectives[rng.gen_range(0..objectives.len())];
        let role = roles[rng.gen_range(0..roles.len())];
        let company = companies[rng.gen_range(0..companies.len())];

        let cv = format!(
            "# {}

**{}**: {} years | **{}**
**Email**: {} | **Phone**: {}

## {}
{}

## {}
- **{}** at {}
  - Responsibilities: Developed scalable systems and maintained code quality.

## {}
- Bachelor in Computer Science - University of Technology

## {}
- Rust, TypeScript, React, PostgreSQL, Docker",
            name,
            if lang == "English" { "Age" } else { "Idade" },
            age,
            format!("{} - {}", city, state),
            email,
            phone,
            obj_title,
            objective,
            exp_title,
            role,
            company,
            edu_title,
            skill_title
        );

        Ok(GenerateResponse {
            text: Some(cv),
            metadata: Some(json!({ "name": name, "language": lang })),
            base64_artifact: None,
        })
    }
}
