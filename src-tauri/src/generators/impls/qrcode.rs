use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use std::io::Cursor;

pub struct QrCodeGenerator;

impl Generator for QrCodeGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "qrcode".into(),
            name: "QR Code".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera um QR Code a partir de um texto.".into(),
            options: Some(serde_json::json!({
                "fields": [
                    {
                        "name": "content",
                        "type": "string",
                        "label": "Conteúdo",
                        "default": "https://github.com/mateusgalasso/DevGen"
                    },
                    {
                        "name": "size",
                        "type": "number",
                        "label": "Tamanho (px)",
                        "default": 256
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let content = options
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let size = options.get("size").and_then(|v| v.as_u64()).unwrap_or(256) as u32;

        if content.is_empty() {
            return Err(GeneratorError::InvalidOptions(
                "Conteúdo não pode ser vazio".into(),
            ));
        }

        let code =
            QrCode::new(content.as_bytes()).map_err(|e| GeneratorError::Internal(e.to_string()))?;

        // Render to image
        let image = code.render::<Luma<u8>>().build();

        // Resize to requested size (using Nearest to keep QR sharp)
        let img = image::imageops::resize(&image, size, size, image::imageops::FilterType::Nearest);

        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .map_err(|e| GeneratorError::Internal(e.to_string()))?;

        let b64 = general_purpose::STANDARD.encode(bytes);
        let data_url = format!("data:image/png;base64,{}", b64);

        Ok(GenerateResponse {
            text: Some(format!("QR Code: {}", content)),
            metadata: Some(serde_json::json!({
                "content": content,
                "size": size
            })),
            base64_artifact: Some(data_url),
        })
    }
}
