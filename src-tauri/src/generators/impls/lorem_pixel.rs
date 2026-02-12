use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, Rgb};
use std::io::Cursor;

pub struct LoremPixelGenerator;

impl Generator for LoremPixelGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "lorem_pixel".into(),
            name: "Lorem Pixel".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera imagens de placeholder coloridas (offline).".into(),
            options: Some(serde_json::json!({
                "fields": [
                    {
                        "name": "width",
                        "type": "number",
                        "label": "Largura",
                        "default": 300
                    },
                    {
                        "name": "height",
                        "type": "number",
                        "label": "Altura",
                        "default": 200
                    },
                    {
                        "name": "color",
                        "type": "select",
                        "label": "Cor Base",
                        "default": "Blue",
                        "options": ["Blue", "Red", "Green", "Grey", "Orange", "Purple"]
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let width = options.get("width").and_then(|v| v.as_u64()).unwrap_or(300) as u32;
        let height = options
            .get("height")
            .and_then(|v| v.as_u64())
            .unwrap_or(200) as u32;
        let color_name = options
            .get("color")
            .and_then(|v| v.as_str())
            .unwrap_or("Blue");

        let rgb = match color_name {
            "Red" => [239, 68, 68],
            "Green" => [34, 197, 94],
            "Grey" => [113, 113, 122],
            "Orange" => [249, 115, 22],
            "Purple" => [168, 85, 247],
            _ => [59, 130, 246], // Blue
        };

        let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_fn(width, height, |_, _| Rgb(rgb));

        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .map_err(|e| GeneratorError::Internal(e.to_string()))?;

        let b64 = general_purpose::STANDARD.encode(bytes);
        let data_url = format!("data:image/png;base64,{}", b64);

        Ok(GenerateResponse {
            text: Some(format!("{}x{} Placeholder ({})", width, height, color_name)),
            metadata: Some(serde_json::json!({
                "width": width,
                "height": height,
                "color": color_name
            })),
            base64_artifact: Some(data_url),
        })
    }
}
