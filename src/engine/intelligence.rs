use strsim::jaro_winkler;
use candle_core::{Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

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
        let intent_lower = intent.to_lowercase();
        let line_lower = line.to_lowercase();

        // 1. Direct word inclusion (Fast & precise)
        for word in intent_lower.split_whitespace() {
            if word.len() > 3 && line_lower.contains(word) {
                return true;
            }
        }

        // 2. Fuzzy matching for typos or variations
        let score = jaro_winkler(&intent_lower, &line_lower) as f32;
        score >= threshold
    }
}

/// Neural Intelligence using Candle (Local Embeddings)
pub struct NeuralIntelligence {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl NeuralIntelligence {
    pub fn new() -> anyhow::Result<Self> {
        let device = Device::Cpu;
        let api = Api::new()?;
        let repo = api.repo(Repo::new("sentence-transformers/all-MiniLM-L6-v2".to_string(), RepoType::Model));
        
        let config_filename = repo.get("config.json")?;
        let tokenizer_filename = repo.get("tokenizer.json")?;
        let weights_filename = repo.get("model.safetensors")?;

        let config_str = std::fs::read_to_string(config_filename)?;
        let config: Config = serde_json::from_str(&config_str)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(anyhow::Error::msg)?;
        
        // Correct API for loading safetensors in recent candle-nn
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(&[weights_filename], candle_core::DType::F32, &device)?
        };
        let model = BertModel::load(vb, &config)?;

        Ok(Self { model, tokenizer, device })
    }

    fn get_embedding(&self, text: &str) -> anyhow::Result<Tensor> {
        let tokens = self.tokenizer.encode(text, true).map_err(anyhow::Error::msg)?;
        let token_ids = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?;
        
        let embeddings = self.model.forward(&token_ids, &token_type_ids, None)?;
        
        // Mean pooling
        let (_n_batch, n_tokens, _hidden_size) = embeddings.dims3()?;
        let mean_embedding = (embeddings.sum(1)? / (n_tokens as f64))?;
        
        // Normalize for cosine similarity
        let norm = mean_embedding.sqr()?.sum_all()?.sqrt()?;
        Ok(mean_embedding.broadcast_div(&norm)?)
    }
}

impl IntelligenceProvider for NeuralIntelligence {
    fn name(&self) -> &str { "neural" }
    
    fn is_relevant(&self, intent: &str, line: &str, threshold: f32) -> bool {
        let e1 = match self.get_embedding(intent) {
            Ok(e) => e,
            Err(_) => return false,
        };
        let e2 = match self.get_embedding(line) {
            Ok(e) => e,
            Err(_) => return false,
        };

        // Cosine similarity
        let similarity = match e1 * e2 {
            Ok(t) => t.sum_all().unwrap().to_scalar::<f32>().unwrap(),
            Err(_) => 0.0,
        };
        similarity >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_relevance() {
        let fuzzy = FuzzyIntelligence;
        assert!(fuzzy.is_relevant("error", "The build failed with error 1", 0.7));
        assert!(!fuzzy.is_relevant("apple", "orange", 0.7));
    }

    #[test]
    #[ignore]
    fn test_neural_similarity_logic() {
        let engine = NeuralIntelligence::new().unwrap();
        let intent = "The website is down";
        let line = "Connection refused at port 80";
        let score = engine.is_relevant(intent, line, 0.5);
        assert!(score);
    }
}
