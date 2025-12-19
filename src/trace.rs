use sha2::{Sha256, Digest};

#[derive(Clone)]
pub struct TraceStep {
    pub step: u64,
    pub obs_hash: [u8; 32],
    pub action_hash: [u8; 32],
}

impl TraceStep {
    pub fn hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.step.to_le_bytes());
        h.update(self.obs_hash);
        h.update(self.action_hash);
        h.finalize().into()
    }
}
