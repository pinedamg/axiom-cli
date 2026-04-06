mod common;
use axiom::IntentContext;
use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use axiom::gateway::core::TerminalEvent;
use std::fs;

#[test]
fn test_learning_lifecycle_and_persistence() {
    // 1. Setup: Usar una base de datos única para este test
    let test_id = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    let db_path = format!("test_learning_{}.db", test_id);
    
    let mut config = AxiomConfig::default();
    config.db_path = std::path::PathBuf::from(db_path.clone());
    config.discovery_threshold = 5;
    
    let command = "custom-miner-tool";
    let context = IntentContext {
        last_message: "monitor mining".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    // --- SESIÓN 1: APRENDIZAJE ---
    {
        let mut session = AxiomSession::new(config.clone()).unwrap();
        
        // Enviamos 10 líneas idénticas. 
        // Las primeras 5 deberían ser "Keep" (Axiom está aprendiendo).
        // Las siguientes 5 deberían ser "None" (Axiom empezó a colapsar).
        let mut kept_count = 0;
        for i in 0..10 {
            let line = format!("Status: Processing hash 0x{:x}...", i);
            if session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context).is_some() {
                kept_count += 1;
            }
        }
        
        assert!(kept_count >= 5, "Session 1 should show initial lines while learning. Got: {}", kept_count);
        
        // Finalizar sesión para persistir en DB
        session.finalize(command, 100, 50).unwrap();
    }

    // --- VERIFICACIÓN DE DB ---
    {
        let session = AxiomSession::new(config.clone()).unwrap();
        let templates = session.engine.get_learned_templates();
        assert!(templates.iter().any(|(t, _)| t.contains("Status: Processing hash <HEX>...")), 
                "Pattern should be persisted in DB");
    }

    // --- SESIÓN 2: SABIDURÍA ---
    {
        let mut session = AxiomSession::new(config.clone()).unwrap();
        
        // Enviamos la misma línea. Ahora debería colapsar DESDE LA PRIMERA.
        let line = "Status: Processing hash 0xabc123...";
        let result = session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context);
        
        assert!(result.is_none(), "Session 2 should instantly collapse known patterns from line 1");
        
        // --- SESIÓN 3: SEGURIDAD (Outlier) ---
        // Aunque el patrón es conocido, si hay un secreto, NO debe colapsar.
        let secret_line = "Status: Processing hash 0xabc123... Secret: AKIAIOSFODNN7EXAMPLE"; // axiom-scan:ignore
        let result_security = session.engine.process_line(TerminalEvent::StaticLine(secret_line.to_string()), command, &context);
        
        assert!(result_security.is_some(), "Known patterns must NOT collapse if they contain secrets (V3 Priority)");
        assert!(result_security.unwrap().contains("[REDACTED_SECRET]"));
    }

    // Cleanup
    let _ = fs::remove_file(db_path);
}
