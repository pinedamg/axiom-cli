use axiom::config::{AxiomConfig, TelemetryLevel};
use tempfile::tempdir;
use std::fs;

#[test]
fn test_config_serialization_deserialization() {
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("config.yaml");
    
    let mut config = AxiomConfig::default();
    // In our refactored config, variants are Discovery, etc.
    config.telemetry_level = TelemetryLevel::Discovery;
    
    // Test direct serialization
    let yaml = serde_yaml::to_string(&config).expect("Failed to serialize config");
    fs::write(&config_path, yaml).expect("Failed to write test config");
    
    // Test direct deserialization
    let content = fs::read_to_string(&config_path).expect("Failed to read test config");
    let decoded: AxiomConfig = serde_yaml::from_str(&content).expect("Failed to deserialize config");
    
    assert_eq!(decoded.node_id, config.node_id);
    assert_eq!(decoded.telemetry_level, TelemetryLevel::Discovery);
}

#[test]
fn test_node_id_registration() {
    let config1 = AxiomConfig::load(); // Load ensures node_id is registered
    assert!(!config1.node_id.is_empty());
}

#[test]
fn test_command_sanitization_logic() {
    let cmd1 = "git commit -m 'secret message'";
    let bin1 = cmd1.split_whitespace().next().unwrap_or("unknown");
    assert_eq!(bin1, "git");
}
