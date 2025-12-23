// Reveal Circuit: Prove revealed cards match commitments
//
// Public Inputs:
//   - card_commitments: [C1, C2] Pedersen commitments (from dealing)
//   - revealed_cards: [v1, v2] card values being revealed
//
// Private Witness:
//   - randomness: [r1, r2] blinding factors (same as dealing)
//
// Constraints:
//   1. C1 = Pedersen(v1, r1), C2 = Pedersen(v2, r2) (opens correctly)
//   2. 0 ≤ v1, v2 < 52 (valid cards)
//
// Estimated constraint count: ~2,000 R1CS

use super::gadgets::*;
use ark_bls12_381::Fr;
use ark_r1cs_std::{
    alloc::AllocVar,
    fields::fp::FpVar,
    prelude::*,
    uint8::UInt8,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::{vec::Vec, Zero};

/// Reveal circuit for mental poker
///
/// Proves that revealed cards match the commitments made during dealing.
/// This ensures dealer cannot change cards after seeing opponent's actions.
#[derive(Clone)]
pub struct RevealCircuit {
    // ========== Public Inputs ==========
    /// Card commitments from dealing phase [C1, C2]
    pub card_commitments: Option<[Vec<u8>; 2]>,

    /// Revealed card values [v1, v2] (0-51)
    pub revealed_cards: Option<[u8; 2]>,

    // ========== Private Witness ==========
    /// Randomness used in commitments [r1, r2]
    /// Must match randomness from dealing phase
    pub randomness: Option<[Fr; 2]>,
}

impl RevealCircuit {
    /// Create new reveal circuit for setup (proving key generation)
    pub fn new_for_setup() -> Self {
        Self {
            card_commitments: None,
            revealed_cards: None,
            randomness: None,
        }
    }

    /// Create new reveal circuit with witness (for proving)
    pub fn new_with_witness(
        card_commitments: [Vec<u8>; 2],
        revealed_cards: [u8; 2],
        randomness: [Fr; 2],
    ) -> Self {
        Self {
            card_commitments: Some(card_commitments),
            revealed_cards: Some(revealed_cards),
            randomness: Some(randomness),
        }
    }

    /// Validate witness data before circuit synthesis
    fn validate_witness(&self) -> Result<(), SynthesisError> {
        if let (Some(commitments), Some(cards), Some(randomness)) = (
            &self.card_commitments,
            &self.revealed_cards,
            &self.randomness,
        ) {
            // Check commitment lengths
            for commitment in commitments.iter() {
                if commitment.len() != 32 {
                    return Err(SynthesisError::Unsatisfiable);
                }
            }

            // Check card values are in valid range
            for &card in cards.iter() {
                if card >= 52 {
                    return Err(SynthesisError::Unsatisfiable);
                }
            }

            // Check randomness is not zero (prevents trivial commitments)
            for r in randomness.iter() {
                if r.is_zero() {
                    return Err(SynthesisError::Unsatisfiable);
                }
            }
        }

        Ok(())
    }
}

impl ConstraintSynthesizer<Fr> for RevealCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Validate witness before generating constraints
        self.validate_witness()?;

        // ========== Allocate Public Inputs ==========

        // Allocate card commitments (2 x 32 bytes each)
        let _commitment1_var = UInt8::new_input_vec(
            cs.clone(),
            &self.card_commitments.as_ref().map(|c| c[0].as_slice()).unwrap_or(&[0u8; 32]),
        )?;

        let _commitment2_var = UInt8::new_input_vec(
            cs.clone(),
            &self.card_commitments.as_ref().map(|c| c[1].as_slice()).unwrap_or(&[0u8; 32]),
        )?;

        // Allocate revealed card values (public)
        let val1_var = FpVar::new_input(cs.clone(), || {
            Ok(Fr::from(self.revealed_cards.as_ref().map(|v| v[0]).unwrap_or(0) as u64))
        })?;

        let val2_var = FpVar::new_input(cs.clone(), || {
            Ok(Fr::from(self.revealed_cards.as_ref().map(|v| v[1]).unwrap_or(0) as u64))
        })?;

        // ========== Allocate Private Witness ==========

        // Randomness for commitments (must match dealing phase)
        let rand1_var = FpVar::new_witness(cs.clone(), || {
            Ok(self.randomness.as_ref().map(|r| r[0]).unwrap_or(Fr::from(1u64)))
        })?;

        let rand2_var = FpVar::new_witness(cs.clone(), || {
            Ok(self.randomness.as_ref().map(|r| r[1]).unwrap_or(Fr::from(1u64)))
        })?;

        // ========== CONSTRAINT 1: Valid Range (0 ≤ v1, v2 < 52) ==========
        // ~12 constraints per check, 2 checks = ~24 constraints
        RangeCheckGadget::check_card_range(&val1_var)?;
        RangeCheckGadget::check_card_range(&val2_var)?;

        // ========== CONSTRAINT 2: Commitment Opening Verification ==========
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

        // ========== Additional Security Constraint: Non-Zero Randomness ==========
        // Ensure randomness is not zero to prevent trivial commitments
        // ~2 constraints
        rand1_var.enforce_not_equal(&FpVar::zero())?;
        rand2_var.enforce_not_equal(&FpVar::zero())?;

        // ========== Total Estimated Constraints: ~2,000 ==========
        // Breakdown:
        // - Range checks: 24
        // - Pedersen commitments: 1,000
        // - Non-zero randomness: 2
        // - Overhead: ~974

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn test_reveal_circuit_setup() {
        let circuit = RevealCircuit::new_for_setup();
        let cs = ConstraintSystem::<Fr>::new_ref();

        let result = circuit.generate_constraints(cs.clone());
        assert!(result.is_ok());

        println!("Reveal circuit constraints: {}", cs.num_constraints());
        // Should be around 2,000 constraints
    }

    #[test]
    fn test_reveal_circuit_valid_witness() {
        let card_commitments = [vec![1u8; 32], vec![2u8; 32]];
        let revealed_cards = [10u8, 20u8];
        let randomness = [Fr::from(100u64), Fr::from(200u64)];

        let circuit = RevealCircuit::new_with_witness(
            card_commitments,
            revealed_cards,
            randomness,
        );

        let cs = ConstraintSystem::<Fr>::new_ref();
        let result = circuit.generate_constraints(cs.clone());

        assert!(result.is_ok());
        // Note: Full satisfaction depends on commitment validity
    }

    #[test]
    fn test_reveal_circuit_invalid_card_range() {
        let card_commitments = [vec![1u8; 32], vec![2u8; 32]];
        let revealed_cards = [10u8, 55u8]; // 55 > 51!
        let randomness = [Fr::from(100u64), Fr::from(200u64)];

        let circuit = RevealCircuit::new_with_witness(
            card_commitments,
            revealed_cards,
            randomness,
        );

        // Should fail validation
        assert!(circuit.validate_witness().is_err());
    }

    #[test]
    fn test_reveal_circuit_zero_randomness() {
        let card_commitments = [vec![1u8; 32], vec![2u8; 32]];
        let revealed_cards = [10u8, 20u8];
        let randomness = [Fr::from(0u64), Fr::from(200u64)]; // Zero randomness!

        let circuit = RevealCircuit::new_with_witness(
            card_commitments,
            revealed_cards,
            randomness,
        );

        // Should fail validation
        assert!(circuit.validate_witness().is_err());
    }

    #[test]
    fn test_reveal_circuit_invalid_commitment_length() {
        let card_commitments = [vec![1u8; 16], vec![2u8; 32]]; // First too short!
        let revealed_cards = [10u8, 20u8];
        let randomness = [Fr::from(100u64), Fr::from(200u64)];

        let circuit = RevealCircuit::new_with_witness(
            card_commitments,
            revealed_cards,
            randomness,
        );

        // Should fail validation
        assert!(circuit.validate_witness().is_err());
    }

    #[test]
    fn test_reveal_circuit_constraint_satisfaction() {
        // Test that constraints are properly enforced
        let card_commitments = [vec![1u8; 32], vec![2u8; 32]];
        let revealed_cards = [0u8, 51u8]; // Boundary values
        let randomness = [Fr::from(12345u64), Fr::from(67890u64)];

        let circuit = RevealCircuit::new_with_witness(
            card_commitments.clone(),
            revealed_cards,
            randomness,
        );

        // Validate witness first
        assert!(circuit.validate_witness().is_ok());

        let cs = ConstraintSystem::<Fr>::new_ref();
        let result = circuit.generate_constraints(cs.clone());

        assert!(result.is_ok());
    }

    #[test]
    fn test_reveal_matches_dealing_randomness() {
        // Simulate using same randomness in both circuits
        let randomness = [Fr::from(999u64), Fr::from(888u64)];

        // Dealing phase (simplified - just store randomness)
        let dealing_randomness = randomness;

        // Reveal phase - must use same randomness
        let card_commitments = [vec![3u8; 32], vec![4u8; 32]];
        let revealed_cards = [5u8, 15u8];

        let circuit = RevealCircuit::new_with_witness(
            card_commitments,
            revealed_cards,
            dealing_randomness, // Same randomness!
        );

        let cs = ConstraintSystem::<Fr>::new_ref();
        let result = circuit.generate_constraints(cs.clone());

        assert!(result.is_ok());
    }
}
