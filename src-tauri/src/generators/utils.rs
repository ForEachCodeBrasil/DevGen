use rand::Rng;

pub fn mod11(digits: &[u8], weights: &[u32]) -> u8 {
    let mut sum = 0;
    for (i, &digit) in digits.iter().enumerate() {
        if i < weights.len() {
            sum += digit as u32 * weights[i];
        }
    }
    let remainder = sum % 11;
    if remainder < 2 {
        0
    } else {
        (11 - remainder) as u8
    }
}

pub fn generate_random_digits(count: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(0..10)).collect()
}
