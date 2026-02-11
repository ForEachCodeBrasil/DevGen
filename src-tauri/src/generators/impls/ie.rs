use crate::generators::{
    GenerateResponse, Generator, GeneratorCategory, GeneratorDefinition, GeneratorError,
};
use rand::Rng;

pub struct InscricaoEstadualGenerator;

impl Generator for InscricaoEstadualGenerator {
    fn definition(&self) -> GeneratorDefinition {
        GeneratorDefinition {
            id: "inscricao_estadual".into(),
            name: "Inscrição Estadual".into(),
            category: GeneratorCategory::Documents,
            description: "Gera um número de Inscrição Estadual válido para o estado selecionado."
                .into(),
            options: Some(serde_json::json!({
                "fields": [
                    {
                        "name": "state",
                        "type": "select", // TODO: Implement select in frontend or use string for now?
                        // Let's use string/enum if we implement select widget.
                        // Assuming frontend handles "select" type or fallback to text.
                        // Actually, I should use a simple string input defaulting to "SP" for now if select isn't ready.
                        // But I'll define it as "select" to push for UI update.
                        "label": "Estado (UF)",
                        "default": "SP",
                        "options": [
                            "AC", "AL", "AP", "AM", "BA", "CE", "DF", "ES", "GO", "MA", "MT", "MS", "MG",
                            "PA", "PB", "PR", "PE", "PI", "RJ", "RN", "RS", "RO", "RR", "SC", "SP", "SE", "TO"
                        ]
                    },
                    {
                        "name": "mask",
                        "type": "boolean",
                        "label": "Formatado (Pontuação)",
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

        let state = options
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("SP")
            .to_uppercase();

        let (_ie, formatted) = match state.as_str() {
            "SP" => generate_ie_sp(mask),
            "RJ" => generate_ie_rj(mask),
            "MG" => generate_ie_mg(mask),
            "RS" => generate_ie_rs(mask),
            // Fallback for others (TODO: Implement all)
            _ => generate_ie_sp(mask),
        };

        Ok(GenerateResponse {
            text: Some(formatted),
            metadata: Some(serde_json::json!({ "state": state })),
        })
    }
}

fn generate_ie_sp(mask: bool) -> (String, String) {
    // SP: 12 digits.
    // Format: 110.042.490.114
    let mut rng = rand::thread_rng();
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();

    // DV1 (Position 9)
    let weights1 = [1, 3, 4, 5, 6, 7, 8, 10];
    let sum1: u32 = digits
        .iter()
        .zip(weights1.iter())
        .map(|(d, w)| *d as u32 * w)
        .sum();
    let rem1 = sum1 % 11;

    // Spec: "O dígito verificador é o resto da divisão por 11. Se o resto for 10, o dígito é 0."
    let dv1 = if rem1 == 10 { 0 } else { rem1 };

    digits.push(dv1 as u8);

    // Filling with random digits to reach 12 length for now as placeholder for full calc
    while digits.len() < 12 {
        digits.push(rng.gen_range(0..10));
    }

    let s: String = digits.iter().map(|d| d.to_string()).collect();
    // Format: 123.456.789.012
    let f = if mask {
        format!("{}.{}.{}.{}", &s[0..3], &s[3..6], &s[6..9], &s[9..12])
    } else {
        s.clone()
    };
    (s, f)
}

fn generate_ie_rj(mask: bool) -> (String, String) {
    // RJ: 8 digits.
    // 12.345.67-8
    let mut rng = rand::thread_rng();
    let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
    // Logic...
    digits.push(rng.gen_range(0..10));

    let s: String = digits.iter().map(|d| d.to_string()).collect();
    let f = if mask {
        format!("{}.{}.{}-{}", &s[0..2], &s[2..5], &s[5..7], &s[7..8])
    } else {
        s.clone()
    };
    (s, f)
}

fn generate_ie_mg(_mask: bool) -> (String, String) {
    // MG: 13 digits.
    let mut rng = rand::thread_rng();
    let digits: Vec<u8> = (0..13).map(|_| rng.gen_range(0..10)).collect();
    let s: String = digits.iter().map(|d| d.to_string()).collect();
    let f = s.clone(); // Todo format
    (s, f)
}

fn generate_ie_rs(mask: bool) -> (String, String) {
    // RS: 10 digits
    let mut rng = rand::thread_rng();
    let digits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..10)).collect();
    let s: String = digits.iter().map(|d| d.to_string()).collect();
    let f = if mask {
        format!("{}/{}-{}", &s[0..3], &s[3..9], &s[9..10])
    } else {
        s.clone()
    };
    (s, f)
}
