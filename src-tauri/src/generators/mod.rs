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
    // Add more fields later
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickGenerateResponse {
    pub text: String,
}

pub trait Generator: Send + Sync {
    fn definition(&self) -> GeneratorDefinition;
    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError>;
}
