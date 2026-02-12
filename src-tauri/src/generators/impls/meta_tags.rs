use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use serde_json::json;

pub struct MetaTagsGenerator;

impl Generator for MetaTagsGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "meta_tags".into(),
            name: "Meta Tags".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera meta tags HTML básicas para SEO.".into(),
            options: Some(json!({
                "fields": [
                    {
                        "name": "title",
                        "type": "text",
                        "label": "Título da Página",
                        "default": "Minha Página Incrível"
                    },
                    {
                        "name": "description",
                        "type": "text",
                        "label": "Descrição",
                        "default": "Esta é uma descrição otimizada para SEO."
                    },
                    {
                        "name": "keywords",
                        "type": "text",
                        "label": "Palavras-chave (sep. vírgula)",
                        "default": "seo, gerador, meta tags"
                    },
                     {
                        "name": "author",
                        "type": "text",
                        "label": "Autor",
                        "default": "Seu Nome"
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let title = options
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Minha Página");
        let description = options
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let keywords = options
            .get("keywords")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let author = options.get("author").and_then(|v| v.as_str()).unwrap_or("");

        let mut output = String::new();
        output.push_str(&format!("<title>{}</title>\n", title));
        output.push_str(&format!(
            "<meta name=\"description\" content=\"{}\">\n",
            description
        ));
        if !keywords.is_empty() {
            output.push_str(&format!(
                "<meta name=\"keywords\" content=\"{}\">\n",
                keywords
            ));
        }
        if !author.is_empty() {
            output.push_str(&format!("<meta name=\"author\" content=\"{}\">\n", author));
        }
        output
            .push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">");

        Ok(GenerateResponse {
            text: Some(output),
            metadata: None,
            base64_artifact: None,
        })
    }
}
