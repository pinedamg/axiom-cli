use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;

#[allow(dead_code)]
pub fn setup_session() -> AxiomSession {
    let config = AxiomConfig::default();
    AxiomSession::new(config).expect("Failed to setup session for testing")
}
