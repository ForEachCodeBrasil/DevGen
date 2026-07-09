use crate::license::LicenseState;
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
    pub requires_pro: bool,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64_artifact: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneratorError {
    NotFound,
    LicenseRequired,
    InvalidOptions(String),
    Internal(String),
}

// Implement Display for GeneratorError
impl std::fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::NotFound => write!(f, "Gerador não encontrado"),
            GeneratorError::LicenseRequired => write!(f, "Recurso disponível apenas no DevGen Pro"),
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
    #[serde(default)]
    pub license_state: LicenseState,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            locale: "pt-BR".into(),
            // Defaults stay on free-tier utilities so tray works without Pro.
            quick_actions: vec![
                "quick.copy_uuid".into(),
                "quick.copy_password".into(),
                "quick.copy_lorem_ipsum".into(),
                "quick.copy_nick".into(),
                "quick.copy_random_number".into(),
            ],
            history: vec![],
            generator_last_options: std::collections::HashMap::new(),
            license_state: LicenseState::default(),
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
