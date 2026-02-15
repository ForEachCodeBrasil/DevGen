use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, Luma};
use qrcode::{Color, QrCode};
use std::io::Cursor;

pub struct QrCodeGenerator;

impl Generator for QrCodeGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "qrcode".into(),
            name: "QR Code".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera um QR Code a partir de um texto.".into(),
            requires_pro: false,
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

        // Manually render QR matrix to ImageBuffer (qrcode crate's render::Luma has incompatible trait bounds with image 0.24)
        let qr_size = code.width() as u32;
        let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(qr_size, qr_size);
        for y in 0..qr_size {
            for x in 0..qr_size {
                let luma = if code[(x as usize, y as usize)] == Color::Dark {
                    Luma([0u8])
                } else {
                    Luma([255u8])
                };
                img.put_pixel(x, y, luma);
            }
        }

        // Resize to requested size (using Nearest to keep QR sharp)
        let img = image::imageops::resize(&img, size, size, image::imageops::FilterType::Nearest);

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
