use axiom::persistence::PersistenceManager;
use axiom::config::AxiomConfig;
use tempfile::NamedTempFile;

#[test]
fn test_persistence_global_state() {
    let tmp_db = NamedTempFile::new().unwrap();
    let p = PersistenceManager::new_with_path(tmp_db.path()).unwrap();

    // Default should be enabled
    assert!(p.get_global_enabled().unwrap());

    // Disable
    p.set_global_enabled(false).unwrap();
    assert!(!p.get_global_enabled().unwrap());

    // Enable back
    p.set_global_enabled(true).unwrap();
    assert!(p.get_global_enabled().unwrap());
}

#[test]
fn test_persistence_bypass_countdown() {
    let tmp_db = NamedTempFile::new().unwrap();
    let p = PersistenceManager::new_with_path(tmp_db.path()).unwrap();

    // Default should be 0
    assert_eq!(p.get_bypass_count().unwrap(), 0);

    // Set bypass
    p.set_bypass_count(3).unwrap();
    assert_eq!(p.get_bypass_count().unwrap(), 3);

    // Decrement
    let next = p.decrement_bypass_count().unwrap();
    assert_eq!(next, 2);
    assert_eq!(p.get_bypass_count().unwrap(), 2);

    // Decrement to zero
    p.decrement_bypass_count().unwrap();
    let last = p.decrement_bypass_count().unwrap();
    assert_eq!(last, 0);
    
    // Should stay at 0
    let below_zero = p.decrement_bypass_count().unwrap();
    assert_eq!(below_zero, 0);
}

#[test]
fn test_config_blacklist_persistence() {
    let mut config = AxiomConfig::default();
    
    // Check defaults
    assert!(config.blacklist.contains(&"vi".to_string()));
    assert!(config.blacklist.contains(&"ssh".to_string()));

    // Add new
    config.blacklist.push("my-custom-tool".to_string());
    
    // Note: save_global uses HOME, so we don't test actual file write here 
    // to avoid polluting the runner's home, but we validate the struct logic.
    assert!(config.blacklist.contains(&"my-custom-tool".to_string()));
}

#[test]
fn test_discovery_threshold_logic() {
    use axiom::engine::AxiomEngine;
    use axiom::privacy::PrivacyRedactor;
    use axiom::engine::intelligence::FuzzyIntelligence;
    use axiom::IntentContext;

    let redactor = PrivacyRedactor::default();
    let intelligence = Box::new(FuzzyIntelligence);
    
    // Create engine with a VERY low threshold (1)
    // This means after 1 repeat, it should start collapsing.
    let mut engine = AxiomEngine::new(redactor, vec![], intelligence, 1);
    
    let context = IntentContext::default();

    let line = "Generating noise...";
    
    // Line 1: Normal
    engine.process_line(line, "unknown", &context);
    // Line 2: Should trigger threshold
    let res2 = engine.process_line(line, "unknown", &context);
    
    // res2 should be None (swallowed) because threshold is 1
    assert_eq!(res2, None);
}
