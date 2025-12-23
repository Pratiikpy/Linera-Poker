//! ZK-SNARK WASM Compatibility Test
//!
//! CRITICAL CHECKPOINT: Phase 1, Task 2
//!
//! This module validates that arkworks cryptographic libraries compile successfully
//! to WebAssembly (wasm32-unknown-unknown target), which is REQUIRED for Linera
//! blockchain execution.
//!
//! Kill Criterion: If this test fails to compile after 3 attempts, we ABORT the
//! ZK-SNARK integration plan and fall back to off-chain proving.
//!
//! Test Strategy:
//! 1. Import core arkworks types (field elements, curves, serialization)
//! 2. Perform basic field arithmetic (addition, multiplication)
//! 3. Create curve points and perform group operations
//! 4. Serialize and deserialize data structures
//! 5. Verify all operations work in no_std WASM environment

#![cfg(target_arch = "wasm32")]

use ark_bls12_381::{Fr, G1Affine, G1Projective};
use ark_ec::{CurveGroup, Group};
use ark_ff::{Field, PrimeField, Zero};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

/// Test 1: Basic field arithmetic in BLS12-381 scalar field
///
/// Validates that field operations compile and execute correctly in WASM.
/// The scalar field Fr is used for all private inputs in ZK-SNARKs.
#[test]
fn test_field_arithmetic_wasm() {
    // Create field elements from u64 constants
    let a = Fr::from(42u64);
    let b = Fr::from(17u64);

    // Addition
    let sum = a + b;
    assert_eq!(sum, Fr::from(59u64), "Field addition failed");

    // Multiplication
    let product = a * b;
    assert_eq!(product, Fr::from(714u64), "Field multiplication failed");

    // Subtraction
    let diff = a - b;
    assert_eq!(diff, Fr::from(25u64), "Field subtraction failed");

    // Multiplicative inverse (a * a^-1 = 1)
    let a_inv = a.inverse().expect("Field element should be invertible");
    let identity = a * a_inv;
    assert_eq!(identity, Fr::from(1u64), "Multiplicative inverse failed");
}

/// Test 2: Elliptic curve point operations
///
/// Validates that elliptic curve arithmetic compiles in WASM.
/// BLS12-381 G1 curve is used for commitments and proof elements.
#[test]
fn test_curve_operations_wasm() {
    // Get the generator point of the G1 curve
    let generator = G1Projective::generator();

    // Scalar multiplication: [5]G
    let scalar = Fr::from(5u64);
    let point = generator * scalar;

    // Verify it's not the identity (point at infinity)
    assert!(!point.is_zero(), "Scalar multiplication resulted in identity");

    // Point addition: [5]G + [3]G = [8]G
    let scalar2 = Fr::from(3u64);
    let point2 = generator * scalar2;
    let sum = point + point2;

    let expected = generator * Fr::from(8u64);
    assert_eq!(sum, expected, "Point addition failed");

    // Convert to affine representation (needed for serialization)
    let affine = point.into_affine();
    assert!(affine.is_on_curve(), "Point should be on curve");
}

/// Test 3: Serialization and deserialization
///
/// CRITICAL: Proofs and verification keys must be serialized for blockchain storage.
/// This validates that arkworks serialization works in WASM no_std environment.
#[test]
fn test_serialization_wasm() {
    // Create a field element
    let original = Fr::from(123456789u64);

    // Serialize to bytes
    let mut serialized = Vec::new();
    original
        .serialize_compressed(&mut serialized)
        .expect("Serialization should succeed");

    assert!(!serialized.is_empty(), "Serialized data should not be empty");

    // Deserialize back
    let deserialized = Fr::deserialize_compressed(&serialized[..])
        .expect("Deserialization should succeed");

    assert_eq!(
        original, deserialized,
        "Deserialized value should match original"
    );
}

/// Test 4: Combined operations (real-world usage pattern)
///
/// Simulates a typical ZK-SNARK workflow:
/// 1. Create field elements from game data (card indices, secrets)
/// 2. Perform cryptographic operations
/// 3. Serialize results for blockchain storage
#[test]
fn test_combined_workflow_wasm() {
    // Simulate card commitment: hash(card_index, secret, nonce)
    // In real implementation, this would be a Poseidon hash in R1CS
    let card_index = Fr::from(42u64); // Card #42 in deck
    let secret = Fr::from(0xDEADBEEFu64); // Player secret
    let nonce = Fr::from(0x12345678u64); // Random nonce

    // Simple commitment: c = card_index + secret * nonce
    let commitment = card_index + (secret * nonce);

    // Create a curve point commitment: C = [c]G
    let generator = G1Projective::generator();
    let commitment_point = generator * commitment;

    // Convert to affine for serialization
    let commitment_affine = commitment_point.into_affine();

    // Serialize the commitment
    let mut serialized = Vec::new();
    commitment_affine
        .serialize_compressed(&mut serialized)
        .expect("Commitment serialization should succeed");

    // Verify serialization produced data
    assert!(
        serialized.len() > 0,
        "Serialized commitment should have non-zero length"
    );

    // Deserialize and verify
    let deserialized: G1Affine = G1Affine::deserialize_compressed(&serialized[..])
        .expect("Commitment deserialization should succeed");

    assert_eq!(
        commitment_affine, deserialized,
        "Deserialized commitment should match original"
    );
}

/// Test 5: Zero and identity elements
///
/// Edge case testing for special values that often cause issues in cryptographic code.
#[test]
fn test_special_values_wasm() {
    // Field zero
    let zero = Fr::from(0u64);
    assert!(zero.is_zero(), "Zero should be recognized");

    // Field one
    let one = Fr::from(1u64);
    assert!(!one.is_zero(), "One should not be zero");
    assert_eq!(one * one, one, "One squared should equal one");

    // Curve identity (point at infinity)
    let identity = G1Projective::zero();
    assert!(identity.is_zero(), "Identity point should be zero");

    // Adding identity should not change point
    let generator = G1Projective::generator();
    let unchanged = generator + identity;
    assert_eq!(unchanged, generator, "Adding identity should not change point");
}

/// Test 6: Large scalar operations
///
/// Validates that large field operations work correctly in WASM.
/// ZK-SNARK proofs involve very large numbers (256-bit field elements).
#[test]
fn test_large_scalars_wasm() {
    // Create a large field element (close to field modulus)
    // BLS12-381 scalar field modulus is approximately 2^255
    let large = Fr::from(u64::MAX);

    // Operations with large numbers
    let large_squared = large * large;
    assert!(!large_squared.is_zero(), "Large number squared should not be zero");

    // Verify field modulus reduction works
    let sum = large + large;
    assert!(!sum.is_zero(), "Sum of large numbers should not be zero");

    // Inverse of large number
    let inverse = large.inverse().expect("Large number should have inverse");
    let product = large * inverse;
    assert_eq!(product, Fr::from(1u64), "Large number * inverse should equal 1");
}

#[cfg(test)]
mod compilation_verification {
    //! This module exists purely to verify compilation.
    //! If this compiles, arkworks is WASM-compatible.

    use super::*;

    /// Compile-time type verification
    #[allow(dead_code)]
    fn verify_types_compile() {
        // Field element type exists and is constructible
        let _field: Fr = Fr::from(1u64);

        // Curve point types exist
        let _projective: G1Projective = G1Projective::generator();
        let _affine: G1Affine = _projective.into_affine();

        // Serialization traits are implemented
        fn _check_serialize<T: CanonicalSerialize>(_: T) {}
        fn _check_deserialize<T: CanonicalDeserialize>() {}

        _check_serialize(_field);
        _check_deserialize::<Fr>();
        _check_serialize(_affine);
        _check_deserialize::<G1Affine>();
    }
}
