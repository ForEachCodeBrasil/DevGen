use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

pub struct CnpjGenerator;

/// Characters allowed in the first 12 positions of an alphanumeric CNPJ.
/// RFB systems must accept A–Z and 0–9; DV uses ASCII code − 48.
const ALPHANUMERIC_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const WEIGHTS_D1: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
const WEIGHTS_D2: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

/// Maps a CNPJ base character to its DV contribution value (ASCII − 48).
/// Digits stay 0–9; letters map A=17 … Z=42.
pub fn char_value(c: u8) -> u32 {
    (c as u32).saturating_sub(48)
}

/// Computes both check digits for a 12-character CNPJ base (numeric or alphanumeric).
/// Base must be uppercase A–Z / 0–9 only.
pub fn calculate_check_digits(base: &[u8]) -> Result<(u8, u8), GeneratorError> {
    if base.len() != 12 {
        return Err(GeneratorError::InvalidOptions(
            "CNPJ base must have 12 characters".into(),
        ));
    }
    for &c in base {
        if !(c.is_ascii_digit() || c.is_ascii_uppercase()) {
            return Err(GeneratorError::InvalidOptions(format!(
                "Invalid CNPJ character: {}",
                c as char
            )));
        }
    }

    let d1 = mod11_check_digit(base, &WEIGHTS_D1);
    let mut with_d1 = base.to_vec();
    with_d1.push(b'0' + d1);
    let d2 = mod11_check_digit(&with_d1, &WEIGHTS_D2);
    Ok((d1, d2))
}

fn mod11_check_digit(chars: &[u8], weights: &[u32]) -> u8 {
    let mut sum: u32 = 0;
    for (i, &c) in chars.iter().enumerate() {
        sum += char_value(c) * weights[i];
    }
    let remainder = sum % 11;
    if remainder < 2 {
        0
    } else {
        (11 - remainder) as u8
    }
}

fn format_cnpj(body: &str, mask: bool) -> String {
    if mask && body.len() == 14 {
        format!(
            "{}.{}.{}/{}-{}",
            &body[0..2],
            &body[2..5],
            &body[5..8],
            &body[8..12],
            &body[12..14]
        )
    } else {
        body.to_string()
    }
}

fn random_numeric_base(rng: &mut impl Rng) -> Vec<u8> {
    let mut base: Vec<u8> = (0..8).map(|_| b'0' + rng.gen_range(0..10)).collect();
    // Default branch / establishment order: 0001 (matriz)
    base.extend_from_slice(b"0001");
    base
}

fn random_alphanumeric_base(rng: &mut impl Rng) -> Vec<u8> {
    let mut base: Vec<u8> = (0..12)
        .map(|_| ALPHANUMERIC_CHARS[rng.gen_range(0..ALPHANUMERIC_CHARS.len())])
        .collect();

    // Guarantee at least one letter so the sample is clearly alphanumeric
    // (useful for testing systems that still only accept digits).
    if base.iter().all(|c| c.is_ascii_digit()) {
        let pos = rng.gen_range(0..12);
        // Pick from A–Z only (skip digits at indices 0..10)
        base[pos] = ALPHANUMERIC_CHARS[10 + rng.gen_range(0..26)];
    }
    base
}

fn build_cnpj(base: Vec<u8>) -> Result<String, GeneratorError> {
    let (d1, d2) = calculate_check_digits(&base)?;
    let mut out = String::with_capacity(14);
    for c in base {
        out.push(c as char);
    }
    out.push(char::from(b'0' + d1));
    out.push(char::from(b'0' + d2));
    Ok(out)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CnpjFormat {
    Numeric,
    Alphanumeric,
    Random,
}

impl CnpjFormat {
    fn from_option(value: Option<&str>) -> Self {
        match value.map(|s| s.to_ascii_lowercase()).as_deref() {
            Some("numeric") | Some("numerico") | Some("numérico") => Self::Numeric,
            Some("alphanumeric") | Some("alfanumerico") | Some("alfanumérico") => {
                Self::Alphanumeric
            }
            Some("random") | Some("aleatorio") | Some("aleatório") => Self::Random,
            _ => Self::Alphanumeric,
        }
    }
}

impl Generator for CnpjGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "cnpj".into(),
            name: "CNPJ".into(),
            category: GeneratorCategory::Documents,
            description: "Gera CNPJ válido (numérico legado ou alfanumérico RFB 2026), com dígitos verificadores corretos.".into(),
            requires_pro: false,
            options: Some(serde_json::json!({
                "fields": [
                    {
                        "name": "format",
                        "type": "select",
                        "label": "Formato",
                        "options": ["alphanumeric", "numeric", "random"],
                        "default": "alphanumeric"
                    },
                    {
                        "name": "mask",
                        "type": "boolean",
                        "label": "Formatado (Pontuação)",
                        "default": true
                    },
                    {
                        "name": "count",
                        "type": "number",
                        "label": "Quantidade",
                        "default": 1
                    }
                ]
            })),
        }
    }

    fn generate(&self, options: serde_json::Value) -> Result<GenerateResponse, GeneratorError> {
        let mask = options
            .get("mask")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let format = CnpjFormat::from_option(options.get("format").and_then(|v| v.as_str()));

        let count = options
            .get("count")
            .and_then(|v| v.as_u64())
            .unwrap_or(1)
            .min(100) as usize;

        let mut rng = rand::thread_rng();
        let mut results = Vec::with_capacity(count);

        for _ in 0..count {
            let use_alpha = match format {
                CnpjFormat::Alphanumeric => true,
                CnpjFormat::Numeric => false,
                CnpjFormat::Random => rng.gen_bool(0.5),
            };

            let base = if use_alpha {
                random_alphanumeric_base(&mut rng)
            } else {
                random_numeric_base(&mut rng)
            };

            let body = build_cnpj(base)?;
            results.push(format_cnpj(&body, mask));
        }

        Ok(GenerateResponse {
            text: Some(results.join("\n")),
            metadata: None,
            base64_artifact: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn strip_mask(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
            .to_ascii_uppercase()
    }

    fn is_valid_cnpj(raw: &str) -> bool {
        let body = strip_mask(raw);
        if body.len() != 14 {
            return false;
        }
        let bytes = body.as_bytes();
        // Last two must be digits
        if !bytes[12].is_ascii_digit() || !bytes[13].is_ascii_digit() {
            return false;
        }
        let Ok((d1, d2)) = calculate_check_digits(&bytes[..12]) else {
            return false;
        };
        bytes[12] == b'0' + d1 && bytes[13] == b'0' + d2
    }

    #[test]
    fn char_value_maps_digits_and_letters() {
        assert_eq!(char_value(b'0'), 0);
        assert_eq!(char_value(b'9'), 9);
        assert_eq!(char_value(b'A'), 17);
        assert_eq!(char_value(b'B'), 18);
        assert_eq!(char_value(b'K'), 27); // 75 - 48
        assert_eq!(char_value(b'Z'), 42);
    }

    #[test]
    fn validates_sefaz_rs_homologation_sample() {
        // CNPJ fictício de homologação SEFAZ-RS (formato alfanumérico)
        assert!(is_valid_cnpj("PC3D315K000193"));
        assert!(is_valid_cnpj("PC.3D3.15K/0001-93"));
    }

    #[test]
    fn validates_known_numeric_cnpj() {
        // Classic numeric: base 112223330001 + DVs
        let base = b"112223330001";
        let (d1, d2) = calculate_check_digits(base).unwrap();
        let body = format!("112223330001{}{}", d1, d2);
        assert!(is_valid_cnpj(&body));
        // Cross-check old-style expectation for this well-known sample
        assert_eq!(body, "11222333000181");
    }

    #[test]
    fn generates_masked_alphanumeric_by_default() {
        let response = CnpjGenerator
            .generate(json!({}))
            .expect("cnpj generation should succeed");
        let text = response.text.expect("text");
        let body = strip_mask(&text);

        assert_eq!(body.len(), 14);
        assert!(text.contains('.'));
        assert!(text.contains('/'));
        assert!(text.contains('-'));
        assert!(is_valid_cnpj(&text));
        // Default is alphanumeric → expect at least one letter in base
        assert!(body[..12].chars().any(|c| c.is_ascii_alphabetic()));
    }

    #[test]
    fn generates_valid_numeric_cnpj() {
        let response = CnpjGenerator
            .generate(json!({ "format": "numeric", "mask": false, "count": 10 }))
            .expect("ok");
        let text = response.text.expect("text");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 10);
        for line in lines {
            assert!(line.chars().all(|c| c.is_ascii_digit()));
            assert!(is_valid_cnpj(line));
            // Numeric samples use branch order 0001 (matriz)
            assert_eq!(&line[8..12], "0001");
        }
    }

    #[test]
    fn generates_valid_alphanumeric_cnpj() {
        let response = CnpjGenerator
            .generate(json!({ "format": "alphanumeric", "mask": false, "count": 20 }))
            .expect("ok");
        let text = response.text.expect("text");
        for line in text.lines() {
            assert_eq!(line.len(), 14);
            assert!(line[..12]
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()));
            assert!(line[12..].chars().all(|c| c.is_ascii_digit()));
            assert!(is_valid_cnpj(line));
            assert!(line[..12].chars().any(|c| c.is_ascii_alphabetic()));
        }
    }

    #[test]
    fn caps_count_at_100() {
        let response = CnpjGenerator
            .generate(json!({ "format": "numeric", "mask": false, "count": 999 }))
            .expect("ok");
        let text = response.text.expect("text");
        assert_eq!(text.lines().count(), 100);
    }

    #[test]
    fn numeric_dv_compatible_with_legacy_algorithm() {
        // ASCII−48 keeps digit values identical, so pure-numeric CNPJs
        // remain valid under the new DV routine.
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let base = random_numeric_base(&mut rng);
            let body = build_cnpj(base).unwrap();
            assert!(is_valid_cnpj(&body));
        }
    }
}
