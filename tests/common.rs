use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use tempfile::tempdir;

#[allow(dead_code)]
pub fn setup_session() -> AxiomSession {
    let mut config = AxiomConfig::default();
    
    // Create a temporary directory that will be leaked during the test run
    // but cleaned up by the OS eventually. This is acceptable for tests 
    // to maintain compatibility with the existing test suite signature.
    let dir = tempdir().expect("Failed to create temp dir");
    let db_path = dir.into_path().join("test_axiom.db");
    
    config.db_path = db_path;
    
    AxiomSession::new(config).expect("Failed to setup session for testing")
}

#[allow(dead_code)]
pub fn setup_test_session() -> (AxiomSession, tempfile::TempDir) {
    let dir = tempdir().expect("Failed to create temp dir");
    let db_path = dir.path().join("test_axiom.db");
    
    let mut config = AxiomConfig::default();
    config.db_path = db_path;
    
    let session = AxiomSession::new(config).expect("Failed to setup session for testing");
    (session, dir)
}
