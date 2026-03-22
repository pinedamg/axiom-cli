/// Calculates the Shannon entropy of a string (bits per character).
/// Secrets usually have a high entropy (> 4.5 bits).
pub fn calculate_entropy(data: &str) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut frequencies = [0usize; 256];
    let len = data.len() as f64;

    for byte in data.as_bytes() {
        frequencies[*byte as usize] += 1;
    }

    frequencies
        .iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_normal_text() {
        let text = "this is a normal sentence with low randomness";
        let entropy = calculate_entropy(text);
        // Normal text usually falls between 3.0 and 4.2
        assert!(entropy < 4.5, "Expected low entropy for normal text, got {}", entropy);
    }

    #[test]
    fn test_entropy_secret_key() {
        let secret = "AKIA5G4H3J2K1L0M9N8P7Q6R5S4T3U2V1W0X";
        let entropy = calculate_entropy(secret);
        // A random secret has high entropy (> 4.5)
        assert!(entropy > 4.5, "Expected high entropy for secret key, got {}", entropy);
    }
}
