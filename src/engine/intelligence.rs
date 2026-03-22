use strsim::jaro_winkler;

/// Interface for all intelligence engines in Axiom
pub trait IntelligenceProvider: Send + Sync {
    fn name(&self) -> &str;
    fn is_relevant(&self, intent: &str, line: &str, threshold: f32) -> bool;
}

/// Lightweight fuzzy matching intelligence (Works everywhere)
pub struct FuzzyIntelligence;

impl IntelligenceProvider for FuzzyIntelligence {
    fn name(&self) -> &str { "fuzzy" }
    
    fn is_relevant(&self, intent: &str, line: &str, threshold: f32) -> bool {
        // Jaro-Winkler is excellent for technical strings/logs
        let score = jaro_winkler(intent, line) as f32;
        score >= threshold
    }
}

/// Future implementation for Neural Embeddings
pub struct NeuralIntelligence;

impl IntelligenceProvider for NeuralIntelligence {
    fn name(&self) -> &str { "neural" }
    
    fn is_relevant(&self, _intent: &str, _line: &str, _threshold: f32) -> bool {
        // Placeholder for when we fix the build issues with ONNX/fastembed
        false
    }
}
