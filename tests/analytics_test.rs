use axiom::persistence::analytics::TokenSavings;

#[test]
fn test_token_savings_calculation() {
    let savings = TokenSavings::new("test command", 1000, 400);
    assert_eq!(savings.raw_bytes, 1000);
    assert_eq!(savings.processed_bytes, 400);
    assert_eq!(savings.savings_percentage(), 60.0);
}

#[test]
fn test_zero_savings() {
    let savings = TokenSavings::new("test command", 0, 0);
    assert_eq!(savings.savings_percentage(), 0.0);
}

#[test]
fn test_no_compression() {
    let savings = TokenSavings::new("test command", 100, 100);
    assert_eq!(savings.savings_percentage(), 0.0);
}
