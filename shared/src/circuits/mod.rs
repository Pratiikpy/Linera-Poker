// Mental Poker Zero-Knowledge Circuits
//
// This module implements R1CS circuits for mental poker using arkworks.
// Circuits are compiled for native proving but only verification runs in WASM.

#![cfg(not(target_arch = "wasm32"))]

pub mod gadgets;
pub mod dealing;
pub mod reveal;

// Re-exports for convenience
pub use dealing::DealingCircuit;
pub use reveal::RevealCircuit;

use ark_bls12_381::Fr;
use serde::{Deserialize, Serialize};

/// Merkle proof for card inclusion in shuffled deck
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Path from leaf to root (bottom-up)
    pub path: Vec<[u8; 32]>,
    /// Indices indicating left (0) or right (1) sibling at each level
    pub indices: Vec<bool>,
}

impl MerkleProof {
    /// Create new Merkle proof
    pub fn new(path: Vec<[u8; 32]>, indices: Vec<bool>) -> Self {
        assert_eq!(
            path.len(),
            indices.len(),
            "Path and indices must have same length"
        );
        Self { path, indices }
    }

    /// Get depth of the tree
    pub fn depth(&self) -> usize {
        self.path.len()
    }
}

/// Card commitment using Pedersen commitment scheme
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardCommitment {
    /// Commitment value (compressed point)
    pub value: Vec<u8>,
}

impl CardCommitment {
    /// Create commitment from bytes
    pub fn from_bytes(value: Vec<u8>) -> Self {
        Self { value }
    }

    /// Get commitment bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_proof_creation() {
        let path = vec![[1u8; 32], [2u8; 32]];
        let indices = vec![false, true];
        let proof = MerkleProof::new(path.clone(), indices.clone());

        assert_eq!(proof.depth(), 2);
        assert_eq!(proof.path, path);
        assert_eq!(proof.indices, indices);
    }

    #[test]
    #[should_panic(expected = "Path and indices must have same length")]
    fn test_merkle_proof_length_mismatch() {
        let path = vec![[1u8; 32]];
        let indices = vec![false, true];
        MerkleProof::new(path, indices);
    }

    #[test]
    fn test_card_commitment_creation() {
        let bytes = vec![1, 2, 3, 4];
        let commitment = CardCommitment::from_bytes(bytes.clone());

        assert_eq!(commitment.as_bytes(), &bytes[..]);
    }
}
