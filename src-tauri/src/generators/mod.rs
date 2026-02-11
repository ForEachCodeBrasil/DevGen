use serde::{Deserialize, Serialize};

pub mod impls;
pub mod registry;
pub mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorCategory {
    Documents,
    Person,
    Company,
    Vehicle,
    Utilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorDefinition {
    pub id: String,
    pub name: String,
    pub category: GeneratorCategory,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
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
pub enum GeneratorError {
    NotFound,
    InvalidOptions(String),
    Internal(String),
}

// Implement Display for GeneratorError
impl std::fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::NotFound => write!(f, "Gerador não encontrado"),
            GeneratorError::InvalidOptions(msg) => write!(f, "Opções inválidas: {}", msg),
            GeneratorError::Internal(msg) => write!(f, "Erro interno: {}", msg),
        }
    }
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
