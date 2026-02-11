use serde::{Deserialize, Serialize};

pub mod registry;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GeneratorCategory {
    Documents,
    Person,
    Company,
    Vehicle,
    Utils,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorDefinition {
    pub id: String,
    pub name: String,
    pub category: GeneratorCategory,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub generator_id: String,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub text: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorError {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPreferences {
    pub locale: String,
    pub quick_actions: Vec<String>,
    pub history: Vec<String>, // TODO: Define specific HistoryItem struct later
    pub generator_last_options: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            locale: "pt-BR".into(),
            quick_actions: vec![
                "quick.copy_cpf_masked".into(),
                "quick.copy_cnpj_masked".into(),
                "quick.copy_person_full".into(),
                "quick.copy_credit_card".into(),
                "quick.copy_password".into(),
            ],
            history: vec![],
            generator_last_options: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickGenerateResponse {
    pub text: String,
}

pub trait Generator: Send + Sync {
    fn definition(&self) -> GeneratorDefinition;
    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError>;
}
