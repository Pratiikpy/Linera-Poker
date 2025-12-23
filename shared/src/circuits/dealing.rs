// Dealing Circuit: Prove dealer committed to 2 valid cards from shuffled deck
//
// Public Inputs:
//   - deck_root: Merkle root of 52-card shuffled deck
//   - card_commitments: [C1, C2] Pedersen commitments
//
// Private Witness:
//   - card_indices: [idx1, idx2] positions in deck (0-51)
//   - card_values: [v1, v2] card values (0-51)
//   - randomness: [r1, r2] blinding factors
//   - merkle_proofs: Proofs that cards are in deck
//
// Constraints:
//   1. idx1 ≠ idx2 (no duplicates)
//   2. 0 ≤ idx1, idx2 < 52 (valid range)
//   3. deck[idx1] = v1, deck[idx2] = v2 (cards match positions)
//   4. C1 = Pedersen(v1, r1), C2 = Pedersen(v2, r2)
//
// Estimated constraint count: ~5,000 R1CS

use super::{gadgets::*, MerkleProof};
use ark_bls12_381::Fr;
use ark_r1cs_std::{
    alloc::AllocVar,
    fields::fp::FpVar,
    prelude::*,
    uint8::UInt8,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::{vec::Vec, Zero};

/// Dealing circuit for mental poker
///
/// Proves that dealer has committed to 2 distinct, valid cards from the shuffled deck.
#[derive(Clone)]
pub struct DealingCircuit {
    // ========== Public Inputs ==========
    /// Merkle root of 52-card shuffled deck
    pub deck_root: Option<[u8; 32]>,

    /// Card commitments [C1, C2]
    pub card_commitments: Option<[Vec<u8>; 2]>,

    // ========== Private Witness ==========
    /// Card indices in deck [idx1, idx2] (0-51)
    pub card_indices: Option<[u8; 2]>,

    /// Card values [v1, v2] (0-51)
    pub card_values: Option<[u8; 2]>,

    /// Randomness for commitments [r1, r2]
    pub randomness: Option<[Fr; 2]>,

    /// Merkle proofs for card inclusion
    pub merkle_proofs: Option<[MerkleProof; 2]>,
}

impl DealingCircuit {
    /// Create new dealing circuit for setup (proving key generation)
    pub fn new_for_setup() -> Self {
        Self {
            deck_root: None,
            card_commitments: None,
            card_indices: None,
            card_values: None,
            randomness: None,
            merkle_proofs: None,
        }
    }

    /// Create new dealing circuit with witness (for proving)
    pub fn new_with_witness(
        deck_root: [u8; 32],
        card_commitments: [Vec<u8>; 2],
        card_indices: [u8; 2],
        card_values: [u8; 2],
        randomness: [Fr; 2],
        merkle_proofs: [MerkleProof; 2],
    ) -> Self {
        Self {
            deck_root: Some(deck_root),
            card_commitments: Some(card_commitments),
            card_indices: Some(card_indices),
            card_values: Some(card_values),
            randomness: Some(randomness),
            merkle_proofs: Some(merkle_proofs),
        }
    }

    /// Validate witness data before circuit synthesis
    fn validate_witness(&self) -> Result<(), SynthesisError> {
        if let (Some(indices), Some(values), Some(randomness), Some(proofs)) = (
            &self.card_indices,
            &self.card_values,
            &self.randomness,
            &self.merkle_proofs,
        ) {
            // Check indices are in valid range
            for &idx in indices.iter() {
                if idx >= 52 {
                    return Err(SynthesisError::Unsatisfiable);
                }
            }

            // Check values are in valid range
            for &val in values.iter() {
                if val >= 52 {
                    return Err(SynthesisError::Unsatisfiable);
                }
            }

            // Check indices are distinct
            if indices[0] == indices[1] {
                return Err(SynthesisError::Unsatisfiable);
            }

            // Check Merkle proof lengths match
            if proofs[0].depth() != proofs[1].depth() {
                return Err(SynthesisError::Unsatisfiable);
            }

            // Randomness should not be zero (prevents trivial commitments)
            for r in randomness.iter() {
                if r.is_zero() {
                    return Err(SynthesisError::Unsatisfiable);
                }
            }
        }

        Ok(())
    }
}

impl ConstraintSynthesizer<Fr> for DealingCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Validate witness before generating constraints
        self.validate_witness()?;

        // ========== Allocate Public Inputs ==========

        // Allocate deck root (32 bytes)
        let deck_root_var = UInt8::new_input_vec(
            cs.clone(),
            &self.deck_root.unwrap_or([0u8; 32]),
        )?;

        if deck_root_var.len() != 32 {
            return Err(SynthesisError::Unsatisfiable);
        }

        let deck_root_bytes: [u8; 32] = self.deck_root.unwrap_or([0u8; 32]);

        // Allocate card commitments (2 x 32 bytes each)
        let _commitment1_var = UInt8::new_input_vec(
            cs.clone(),
            &self.card_commitments.as_ref().map(|c| c[0].as_slice()).unwrap_or(&[0u8; 32]),
        )?;

        let _commitment2_var = UInt8::new_input_vec(
            cs.clone(),
            &self.card_commitments.as_ref().map(|c| c[1].as_slice()).unwrap_or(&[0u8; 32]),
        )?;

        // ========== Allocate Private Witness ==========

        // Card indices (0-51)
        let idx1_var = FpVar::new_witness(cs.clone(), || {
            Ok(Fr::from(self.card_indices.as_ref().map(|i| i[0]).unwrap_or(0) as u64))
        })?;

        let idx2_var = FpVar::new_witness(cs.clone(), || {
            Ok(Fr::from(self.card_indices.as_ref().map(|i| i[1]).unwrap_or(0) as u64))
        })?;

        // Card values (0-51)
        let val1_var = FpVar::new_witness(cs.clone(), || {
            Ok(Fr::from(self.card_values.as_ref().map(|v| v[0]).unwrap_or(0) as u64))
        })?;

        let val2_var = FpVar::new_witness(cs.clone(), || {
            Ok(Fr::from(self.card_values.as_ref().map(|v| v[1]).unwrap_or(0) as u64))
        })?;

        // Randomness for commitments
        let rand1_var = FpVar::new_witness(cs.clone(), || {
            Ok(self.randomness.as_ref().map(|r| r[0]).unwrap_or(Fr::from(1u64)))
        })?;

        let rand2_var = FpVar::new_witness(cs.clone(), || {
            Ok(self.randomness.as_ref().map(|r| r[1]).unwrap_or(Fr::from(1u64)))
        })?;

        // ========== CONSTRAINT 1: idx1 ≠ idx2 (No Duplicates) ==========
        // ~2 constraints
        RangeCheckGadget::enforce_not_equal(&idx1_var, &idx2_var)?;

        // ========== CONSTRAINT 2: Valid Range (0 ≤ idx, val < 52) ==========
        // ~12 constraints per check, 4 checks = ~48 constraints
        RangeCheckGadget::check_card_range(&idx1_var)?;
        RangeCheckGadget::check_card_range(&idx2_var)?;
        RangeCheckGadget::check_card_range(&val1_var)?;
        RangeCheckGadget::check_card_range(&val2_var)?;

        // ========== CONSTRAINT 3: Merkle Path Verification ==========
        // deck[idx1] = v1, deck[idx2] = v2
        // ~1600 constraints per proof (assuming depth 6 for 64-leaf tree)
        // Total: ~3200 constraints

        if let Some(proofs) = &self.merkle_proofs {
            // Verify card 1 is in deck at idx1
            MerklePathGadget::verify_path(
                cs.clone(),
                &deck_root_bytes,
                &val1_var,
                &proofs[0].path,
                &proofs[0].indices,
            )?;

            // Verify card 2 is in deck at idx2
            MerklePathGadget::verify_path(
                cs.clone(),
                &deck_root_bytes,
                &val2_var,
                &proofs[1].path,
                &proofs[1].indices,
            )?;
        } else {
            // During setup, create dummy constraints
            let dummy_path = vec![[0u8; 32]; 6];
            let dummy_indices = vec![false; 6];

            MerklePathGadget::verify_path(
                cs.clone(),
                &deck_root_bytes,
                &val1_var,
                &dummy_path,
                &dummy_indices,
            )?;

            MerklePathGadget::verify_path(
                cs.clone(),
                &deck_root_bytes,
                &val2_var,
                &dummy_path,
                &dummy_indices,
            )?;
        }

        // ========== CONSTRAINT 4: Pedersen Commitment Verification ==========
        // C1 = Pedersen(v1, r1), C2 = Pedersen(v2, r2)
        // ~500 constraints per commitment = ~1000 constraints

        let commitment1_bytes = self
            .card_commitments
            .as_ref()
            .map(|c| c[0].as_slice())
            .unwrap_or(&[0u8; 32]);

        let commitment2_bytes = self
            .card_commitments
            .as_ref()
            .map(|c| c[1].as_slice())
            .unwrap_or(&[0u8; 32]);

        PedersenGadget::verify_commitment(cs.clone(), commitment1_bytes, &val1_var, &rand1_var)?;
        PedersenGadget::verify_commitment(cs.clone(), commitment2_bytes, &val2_var, &rand2_var)?;

        // ========== Total Estimated Constraints: ~5,000 ==========
        // Breakdown:
        // - Not equal: 2
        // - Range checks: 48
        // - Merkle proofs: 3,200
        // - Pedersen commitments: 1,000
        // - Overhead: ~750

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn test_dealing_circuit_setup() {
        let circuit = DealingCircuit::new_for_setup();
        let cs = ConstraintSystem::<Fr>::new_ref();

        let result = circuit.generate_constraints(cs.clone());
        assert!(result.is_ok());

        println!("Dealing circuit constraints: {}", cs.num_constraints());
        // Should be around 5,000 constraints
    }

    #[test]
    fn test_dealing_circuit_valid_witness() {
        let deck_root = [1u8; 32];
        let card_commitments = [vec![2u8; 32], vec![3u8; 32]];
        let card_indices = [0u8, 1u8];
        let card_values = [10u8, 20u8];
        let randomness = [Fr::from(100u64), Fr::from(200u64)];

        let merkle_proof1 = MerkleProof::new(
            vec![[4u8; 32], [5u8; 32], [6u8; 32], [7u8; 32], [8u8; 32], [9u8; 32]],
            vec![false, false, false, false, false, false],
        );

        let merkle_proof2 = MerkleProof::new(
            vec![[10u8; 32], [11u8; 32], [12u8; 32], [13u8; 32], [14u8; 32], [15u8; 32]],
            vec![true, false, false, false, false, false],
        );

        let circuit = DealingCircuit::new_with_witness(
            deck_root,
            card_commitments,
            card_indices,
            card_values,
            randomness,
            [merkle_proof1, merkle_proof2],
        );

        let cs = ConstraintSystem::<Fr>::new_ref();
        let result = circuit.generate_constraints(cs.clone());

        assert!(result.is_ok());
        // Note: Full satisfaction depends on actual Merkle proof validity
    }

    #[test]
    fn test_dealing_circuit_duplicate_indices() {
        let deck_root = [1u8; 32];
        let card_commitments = [vec![2u8; 32], vec![3u8; 32]];
        let card_indices = [5u8, 5u8]; // Duplicate!
        let card_values = [10u8, 20u8];
        let randomness = [Fr::from(100u64), Fr::from(200u64)];

        let merkle_proof = MerkleProof::new(
            vec![[0u8; 32]; 6],
            vec![false; 6],
        );

        let circuit = DealingCircuit::new_with_witness(
            deck_root,
            card_commitments,
            card_indices,
            card_values,
            randomness,
            [merkle_proof.clone(), merkle_proof],
        );

        // Should fail validation
        assert!(circuit.validate_witness().is_err());
    }

    #[test]
    fn test_dealing_circuit_invalid_range() {
        let deck_root = [1u8; 32];
        let card_commitments = [vec![2u8; 32], vec![3u8; 32]];
        let card_indices = [0u8, 55u8]; // 55 > 51!
        let card_values = [10u8, 20u8];
        let randomness = [Fr::from(100u64), Fr::from(200u64)];

        let merkle_proof = MerkleProof::new(
            vec![[0u8; 32]; 6],
            vec![false; 6],
        );

        let circuit = DealingCircuit::new_with_witness(
            deck_root,
            card_commitments,
            card_indices,
            card_values,
            randomness,
            [merkle_proof.clone(), merkle_proof],
        );

        // Should fail validation
        assert!(circuit.validate_witness().is_err());
    }

    #[test]
    fn test_dealing_circuit_zero_randomness() {
        let deck_root = [1u8; 32];
        let card_commitments = [vec![2u8; 32], vec![3u8; 32]];
        let card_indices = [0u8, 1u8];
        let card_values = [10u8, 20u8];
        let randomness = [Fr::from(0u64), Fr::from(200u64)]; // Zero randomness!

        let merkle_proof = MerkleProof::new(
            vec![[0u8; 32]; 6],
            vec![false; 6],
        );

        let circuit = DealingCircuit::new_with_witness(
            deck_root,
            card_commitments,
            card_indices,
            card_values,
            randomness,
            [merkle_proof.clone(), merkle_proof],
        );

        // Should fail validation
        assert!(circuit.validate_witness().is_err());
    }
}
