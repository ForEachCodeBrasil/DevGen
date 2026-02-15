use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use serde_json::json;

pub struct LoremIpsumGenerator;

impl Generator for LoremIpsumGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "lorem_ipsum".into(),
            name: "Lorem Ipsum".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera texto placeholder (Lorem Ipsum).".into(),
            requires_pro: false,
            options: Some(json!({
                "fields": [
                    {
                        "name": "paragraphs",
                        "type": "number",
                        "label": "Parágrafos",
                        "default": 1
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let paragraphs_count = options
            .get("paragraphs")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;

        // Use a simple static lipsum for now.
        // Or reconstruct it.
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

        let mut output = String::new();
        for i in 0..paragraphs_count {
            if i > 0 {
                output.push_str("\n\n");
            }
            output.push_str(text);
        }

        Ok(GenerateResponse {
            text: Some(output),
            metadata: None,
            base64_artifact: None,
        })
    }
}
