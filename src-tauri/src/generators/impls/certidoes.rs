use crate::generators::{
    utils, GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

// Standard Format (Matrícula): CCCCCC AA 55 YYYY T NNNNN LLL DV
// CCCCCC: CNS (6)
// AA: Acervo (2)
// 55: Type (Civil Registry code fixed?) -> standard uses 55 for civil registry
// YYYY: Year (4)
// T: Type (1-Nasc, 2-Cas, 3-Obito, etc?)
// NNNNN: Book (5)
// LLL: Leaf (3)
// DV: Verifier (2)

fn generate_certidao(cert_type: u8, mask: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut digits = Vec::with_capacity(32);

    // CNS (6 digits) - Random
    digits.extend(utils::generate_random_digits(6));

    // Acervo (2 digits) - 01 (Livro)
    digits.extend_from_slice(&[0, 1]);

    // 55 (2 digits) - Civil Registry
    digits.extend_from_slice(&[5, 5]);

    // Year (4 digits) - 1970 to 2025
    let year = rng.gen_range(1970..2026);
    digits.push((year / 1000) as u8);
    digits.push(((year / 100) % 10) as u8);
    digits.push(((year / 10) % 10) as u8);
    digits.push((year % 10) as u8);

    // Type (1 digit)
    digits.push(cert_type);

    // Book (5 digits)
    digits.extend(utils::generate_random_digits(5));

    // Leaf (3 digits)
    digits.extend(utils::generate_random_digits(3));

    // DV calculation:
    // For now using random digits to guarantee valid length/format.
    // Precise Algo TODO.

    let dv1 = rng.gen_range(0..10) as u8;
    let dv2 = rng.gen_range(0..10) as u8;

    digits.push(dv1);
    digits.push(dv2);

    let s: String = digits.iter().map(|d| d.to_string()).collect();

    if mask {
        // Format: CCCCCC AA 55 YYYY T NNNNN LLL DV
        format!(
            "{} {} {} {} {} {} {} {}",
            &s[0..6],
            &s[6..8],
            &s[8..10],
            &s[10..14],
            &s[14..15],
            &s[15..20],
            &s[20..23],
            &s[23..25]
        )
    } else {
        s
    }
}

pub struct CertidaoNascimentoGenerator;
impl Generator for CertidaoNascimentoGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "certidao_nascimento".into(),
            name: "Certidão de Nascimento".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de matrícula de Certidão de Nascimento.".into(),
            options: Some(serde_json::json!({
                "fields": [{ "name": "mask", "type": "boolean", "label": "Formatado", "default": true }]
            })),
        }
    }
    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let mask = options
            .get("mask")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        Ok(GenerateResponse {
            text: Some(generate_certidao(1, mask)),
            metadata: None,
        })
    }
}

pub struct CertidaoCasamentoGenerator;
impl Generator for CertidaoCasamentoGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "certidao_casamento".into(),
            name: "Certidão de Casamento".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de matrícula de Certidão de Casamento.".into(),
            options: Some(serde_json::json!({
                "fields": [{ "name": "mask", "type": "boolean", "label": "Formatado", "default": true }]
            })),
        }
    }
    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let mask = options
            .get("mask")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        Ok(GenerateResponse {
            text: Some(generate_certidao(2, mask)),
            metadata: None,
        })
    }
}

pub struct CertidaoObitoGenerator;
impl Generator for CertidaoObitoGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "certidao_obito".into(),
            name: "Certidão de Óbito".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de matrícula de Certidão de Óbito.".into(),
            options: Some(serde_json::json!({
                "fields": [{ "name": "mask", "type": "boolean", "label": "Formatado", "default": true }]
            })),
        }
    }
    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let mask = options
            .get("mask")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        Ok(GenerateResponse {
            text: Some(generate_certidao(4, mask)),
            metadata: None,
        })
    }
}
