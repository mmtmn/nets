use crate::merkle::MerkleTree;
use serde::{Serialize, Deserialize};

/// A proof that a specific execution step was committed,
/// but is inconsistent with deterministic recomputation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceStepProof {
    /// Index of the step in the execution trace
    pub step_index: u64,

    /// Hash of the observation at this step
    pub obs_hash: [u8; 32],

    /// Hash of the action taken at this step
    pub action_hash: [u8; 32],

    /// Merkle inclusion path proving this step is part of the committed root
    pub merkle_path: Vec<[u8; 32]>,
}

impl TraceStepProof {
    /// Compute the leaf hash corresponding to this step.
    ///
    /// This MUST exactly match the hashing logic used
    /// when constructing the execution trace.
    pub fn leaf_hash(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};

        let mut h = Sha256::new();
        h.update(&self.step_index.to_le_bytes());
        h.update(&self.obs_hash);
        h.update(&self.action_hash);
        h.finalize().into()
    }
}

/// Verify that a step-level fraud proof is included in the committed root.
///
/// This checks ONLY cryptographic inclusion:
/// - leaf hash correctness
/// - Merkle path correctness
///
/// It does NOT check execution correctness.
/// Execution correctness is established by replay elsewhere.
pub fn verify_step_proof(
    committed_root: [u8; 32],
    proof: &TraceStepProof,
) -> bool {
    let leaf = proof.leaf_hash();

    MerkleTree::verify(
        leaf,
        &proof.merkle_path,
        committed_root,
        proof.step_index as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Sha256, Digest};

    fn h(x: u8) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(&[x]);
        h.finalize().into()
    }

    #[test]
    fn valid_proof_passes() {
        let proof = TraceStepProof {
            step_index: 1,
            obs_hash: h(10),
            action_hash: h(11),
            merkle_path: Vec::new(),
        };

        let leaf = proof.leaf_hash();

        let leaves = vec![h(1), leaf, h(3)];
        let tree = MerkleTree::build(leaves);

        let proof = TraceStepProof {
            merkle_path: tree.prove(1),
            ..proof
        };

        assert!(verify_step_proof(tree.root(), &proof));
    }

    #[test]
    fn wrong_step_index_fails() {
        let proof = TraceStepProof {
            step_index: 0,
            obs_hash: h(10),
            action_hash: h(11),
            merkle_path: Vec::new(),
        };

        let leaf = proof.leaf_hash();
        let tree = MerkleTree::build(vec![leaf]);

        let bad_proof = TraceStepProof {
            step_index: 1,
            merkle_path: tree.prove(0),
            ..proof
        };

        assert!(!verify_step_proof(tree.root(), &bad_proof));
    }

    #[test]
    fn wrong_merkle_path_fails() {
        let proof = TraceStepProof {
            step_index: 0,
            obs_hash: h(10),
            action_hash: h(11),
            merkle_path: Vec::new(),
        };

        let leaf = proof.leaf_hash();
        let tree = MerkleTree::build(vec![leaf, h(99)]);

        let mut bad_path = tree.prove(0);
        bad_path[0] = h(255);

        let bad_proof = TraceStepProof {
            merkle_path: bad_path,
            ..proof
        };

        assert!(!verify_step_proof(tree.root(), &bad_proof));
    }

    #[test]
    fn wrong_root_fails() {
        let proof = TraceStepProof {
            step_index: 0,
            obs_hash: h(10),
            action_hash: h(11),
            merkle_path: Vec::new(),
        };

        let leaf = proof.leaf_hash();
        let tree = MerkleTree::build(vec![leaf]);

        let proof = TraceStepProof {
            merkle_path: tree.prove(0),
            ..proof
        };

        assert!(!verify_step_proof(h(255), &proof));
    }
}
