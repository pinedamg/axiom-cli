use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeRegisterReq {
    pub hardware_hash: String,
    pub os: String,
    pub arch: String,
    pub pow_nonce: u64,
    pub funnel_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeRegisterRes {
    pub node_id: String,
    pub node_token: String,
}

pub struct Handshake;

impl Handshake {
    const REGISTER_URL: &'static str = "https://axiom-pulse-api.mpineda.com.ar/v1/node/register";

    /// Generates a unique hardware fingerprint (one-way hash)
    pub fn get_hardware_hash() -> String {
        let uid = machine_uid::get().unwrap_or_else(|_| "unknown-hardware".to_string());
        let mut hasher = Sha256::new();
        hasher.update(uid);
        hex::encode(hasher.finalize())
    }

    /// Solves the Proof of Work required by the Pulse protocol
    pub fn solve_pow(hardware_hash: &str) -> u64 {
        let mut nonce = 0;
        loop {
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}", hardware_hash, nonce));
            let result = hex::encode(hasher.finalize());
            if result.starts_with("0000") {
                return nonce;
            }
            nonce += 1;
            // Safety break to prevent infinite loops in extreme cases
            if nonce > 10_000_000 { return 0; }
        }
    }

    /// Registers or re-identifies the node with the Pulse backend
    pub fn register_node() -> anyhow::Result<(String, String)> {
        let hash = Self::get_hardware_hash();
        let nonce = Self::solve_pow(&hash);
        let funnel_id = std::env::var("AXIOM_FUNNEL_ID").ok().and_then(|s| Uuid::parse_str(&s).ok());

        let req = NodeRegisterReq {
            hardware_hash: hash,
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            pow_nonce: nonce,
            funnel_id,
        };

        let response: NodeRegisterRes = ureq::post(Self::REGISTER_URL)
            .timeout(std::time::Duration::from_secs(5))
            .send_json(req)?
            .into_json()?;

        Ok((response.node_id, response.node_token))
    }
}
