use sha2::{Sha256, Digest};

fn hash_pair(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(a);
    h.update(b);
    h.finalize().into()
}

#[derive(Clone)]
pub struct MerkleTree {
    pub leaves: Vec<[u8; 32]>,
    pub levels: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    pub fn build(leaves: Vec<[u8; 32]>) -> Self {
        let mut levels = Vec::new();
        levels.push(leaves.clone());

        let mut current = leaves.clone();

        while current.len() > 1 {
            let mut next = Vec::new();

            for i in (0..current.len()).step_by(2) {
                let left = current[i];
                let right = if i + 1 < current.len() {
                    current[i + 1]
                } else {
                    current[i]
                };
                next.push(hash_pair(&left, &right));
            }

            levels.push(next.clone());
            current = next;
        }

        Self { leaves, levels }
    }

    pub fn root(&self) -> [u8; 32] {
        self.levels.last().unwrap()[0]
    }

    /// Generate a Merkle inclusion proof for a leaf index
    pub fn prove(&self, mut index: usize) -> Vec<[u8; 32]> {
        let mut proof = Vec::new();

        for level in &self.levels {
            let sibling = if index % 2 == 0 {
                index + 1
            } else {
                index - 1
            };

            if sibling < level.len() {
                proof.push(level[sibling]);
            }

            index /= 2;
        }

        proof
    }

    /// Verify a Merkle inclusion proof for a leaf index
    pub fn verify(
        leaf: [u8; 32],
        proof: &[[u8; 32]],
        root: [u8; 32],
        mut index: usize,
    ) -> bool {
        let mut hash = leaf;

        for sibling in proof {
            hash = if index % 2 == 0 {
                hash_pair(&hash, sibling)
            } else {
                hash_pair(sibling, &hash)
            };
            index /= 2;
        }

        hash == root
    }
}
