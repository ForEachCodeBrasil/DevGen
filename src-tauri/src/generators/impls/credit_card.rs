use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;
use serde_json::json;

pub struct CreditCardGenerator;

impl Generator for CreditCardGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "credit_card".into(),
            name: "Cartão de Crédito".into(),
            category: GeneratorCategory::Utilities,
            description: "Gera números de cartão de crédito válidos (Luhn) para testes.".into(),
            requires_pro: false,
            options: Some(json!({
                "fields": [
                    {
                        "name": "brand",
                        "type": "select",
                        "label": "Bandeira",
                        "default": "visa",
                        "options": ["visa", "mastercard", "amex", "discover"]
                    },
                    {
                        "name": "mask",
                        "type": "boolean",
                        "label": "Formatado",
                        "default": true
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
        let brand = options
            .get("brand")
            .and_then(|v| v.as_str())
            .unwrap_or("visa");

        let mut rng = rand::thread_rng();

        let (length, prefix) = match brand {
            "visa" => (16, vec![4]),
            "mastercard" => (16, vec![5, rng.gen_range(1..=5)]),
            "amex" => (15, vec![3, if rng.gen_bool(0.5) { 4 } else { 7 }]),
            "discover" => (16, vec![6, 0, 1, 1]),
            _ => (16, vec![4]),
        };

        let mut digits: Vec<u8> = prefix;
        while digits.len() < length - 1 {
            digits.push(rng.gen_range(0..10));
        }

        // Calculate Luhn check digit
        let mut sum = 0;
        for (i, &digit) in digits.iter().rev().enumerate() {
            let mut val = digit as u32;
            // For odd positions (0-indexed from right in the check loop, which effectively means even indices from right if 1-based,
            // but the check digit itself is pos 0. So the first digit to check is pos 1 (doubled).
            // Actually Luhn: Double every second digit from the right.
            if i % 2 == 0 {
                val *= 2;
                if val > 9 {
                    val -= 9;
                }
            }
            sum += val;
        }

        let rem = sum % 10;
        let check_digit = if rem == 0 { 0 } else { 10 - rem };
        digits.push(check_digit as u8);

        let s: String = digits.iter().map(|d| d.to_string()).collect();

        // Expiration and CVV
        let exp_month = rng.gen_range(1..=12);
        let exp_year = rng.gen_range(2025..2035);
        let cvv = if brand == "amex" {
            rng.gen_range(1000..10000)
        } else {
            rng.gen_range(100..1000)
        };

        let formatted = if mask {
            // Basic chunks of 4
            let mut f = String::new();
            for (i, c) in s.chars().enumerate() {
                if i > 0 && i % 4 == 0 {
                    f.push(' ');
                }
                f.push(c);
            }
            f
        } else {
            s.clone()
        };

        let text = format!(
            "Número: {}\nValidade: {:02}/{}\nCVV: {}",
            formatted, exp_month, exp_year, cvv
        );

        Ok(GenerateResponse {
            text: Some(text),
            metadata: Some(json!({
                "number": s,
                "expiration": format!("{:02}/{}", exp_month, exp_year),
                "cvv": cvv
            })),
            base64_artifact: None,
        })
    }
}
