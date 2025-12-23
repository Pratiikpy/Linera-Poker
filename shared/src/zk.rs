//! Zero-Knowledge Proof Types for Mental Poker
//!
//! This module defines the cryptographic proof structures used in the Linera Poker
//! mental poker protocol. It provides Pedersen commitments and Groth16 ZK-SNARKs
//! for privacy-preserving card dealing and revealing.
//!
//! # Architecture
//!
//! The ZK proof system consists of two main circuits:
//!
//! 1. **Dealing Circuit**: Proves that dealt cards were sampled from a properly
//!    shuffled 52-card deck without revealing which cards were dealt.
//!
//! 2. **Reveal Circuit**: Proves that revealed cards match previously committed
//!    cards without the dealer being able to forge different cards.
//!
//! # Phase 1 Implementation (Current)
//!
//! This is the Phase 1 mock implementation. All verification functions accept
//! valid-looking proofs to enable end-to-end testing of the poker protocol
//! before the actual BLS12-381 Groth16 circuits are implemented in Phase 2.
//!
//! **WARNING**: These mock functions DO NOT provide cryptographic security.
//! They perform only basic structural validation (non-empty proofs, correct
//! array lengths, etc.). Do not use in production until Phase 2 is complete.
//!
//! # Phase 2 Migration Path
//!
//! Phase 2 will replace the mock functions with:
//! - Real BLS12-381 Pedersen commitments
//! - Groth16 proof generation using arkworks-rs or bellman
//! - Cryptographic verification of dealing and reveal proofs
//! - Proper randomness generation and blinding factors
//!
//! The type signatures will remain unchanged, ensuring seamless migration.

use crate::Card;
use serde::{Deserialize, Serialize};

// ============================================================================
// CARD COMMITMENT (Pedersen Commitment)
// ============================================================================

/// Pedersen commitment to a playing card using BLS12-381 elliptic curve.
///
/// A Pedersen commitment allows a player to commit to a card value without
/// revealing it. The commitment is binding (cannot be changed later) and
/// hiding (reveals no information about the card).
///
/// # Cryptographic Construction
///
/// For a card with index `c` and randomness `r`:
/// ```text
/// Commitment = c * G + r * H
/// ```
/// where `G` and `H` are independent generators of the BLS12-381 G1 group.
///
/// # Phase 1 vs Phase 2
///
/// - **Phase 1 (current)**: `commitment` is a mock 48-byte blob
/// - **Phase 2**: `commitment` will be a compressed BLS12-381 G1 point (48 bytes)
///
/// # Security Properties
///
/// - **Binding**: Computationally infeasible to find two different card/randomness
///   pairs that produce the same commitment
/// - **Hiding**: The commitment reveals no information about the card value
///   (information-theoretic security)
/// - **Homomorphic**: Commitments can be added together (useful for batch verification)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardCommitment {
    /// Pedersen commitment value (48 bytes compressed BLS12-381 G1 point).
    ///
    /// In Phase 1, this is a placeholder byte vector.
    /// In Phase 2, this will be a valid compressed G1 point.
    pub commitment: Vec<u8>,

    /// Unique nonce to prevent commitment reuse across different games.
    ///
    /// This ensures that the same card dealt in different games will have
    /// different commitments, preventing replay attacks.
    pub nonce: [u8; 16],
}

impl CardCommitment {
    /// Expected size of a BLS12-381 G1 compressed point in bytes.
    pub const COMMITMENT_SIZE: usize = 48;

    /// Size of the nonce in bytes.
    pub const NONCE_SIZE: usize = 16;

    /// Create a new CardCommitment with the given commitment and nonce.
    ///
    /// # Arguments
    ///
    /// * `commitment` - The Pedersen commitment value (48 bytes expected)
    /// * `nonce` - A unique 16-byte nonce for this commitment
    ///
    /// # Example
    ///
    /// ```
    /// use linera_poker_shared::zk::CardCommitment;
    ///
    /// let commitment = vec![0u8; CardCommitment::COMMITMENT_SIZE];
    /// let nonce = [42u8; CardCommitment::NONCE_SIZE];
    /// let card_commitment = CardCommitment::new(commitment, nonce);
    /// ```
    pub fn new(commitment: Vec<u8>, nonce: [u8; 16]) -> Self {
        Self { commitment, nonce }
    }

    /// Validate the structural correctness of the commitment.
    ///
    /// Checks that the commitment has the expected size. In Phase 2, this will
    /// also validate that the bytes represent a valid BLS12-381 G1 point.
    ///
    /// # Returns
    ///
    /// `true` if the commitment is structurally valid, `false` otherwise.
    pub fn is_valid(&self) -> bool {
        self.commitment.len() == Self::COMMITMENT_SIZE
    }
}

// ============================================================================
// DEALING PROOF (Dealing Phase ZK-SNARK)
// ============================================================================

/// Zero-knowledge proof that dealt cards were honestly sampled from a shuffled deck.
///
/// This proof is generated by the dealer when dealing hole cards to players.
/// It proves in zero-knowledge that:
///
/// 1. The dealt cards exist in the committed 52-card deck
/// 2. No cards are dealt more than once (no duplicates)
/// 3. The cards were sampled according to the shuffle randomness
/// 4. The dealer cannot predict which cards were dealt
///
/// # Circuit Public Inputs
///
/// - Merkle root of the 52-card deck commitment
/// - Pedersen commitments to the dealt cards
///
/// # Circuit Private Inputs (Witness)
///
/// - The actual card indices
/// - Merkle proof paths showing cards are in the deck
/// - Pedersen randomness for each card commitment
///
/// # Phase 1 vs Phase 2
///
/// - **Phase 1 (current)**: `proof` is a mock 192-byte blob
/// - **Phase 2**: `proof` will be a valid Groth16 proof (192 bytes: G1 point (48) + G2 point (96) + G1 point (48))
///
/// # Security Guarantees
///
/// - **Soundness**: A malicious dealer cannot create a valid proof for cards
///   that don't exist in the shuffled deck
/// - **Zero-Knowledge**: The proof reveals nothing about which specific cards
///   were dealt (beyond what's revealed by the commitments)
/// - **Non-malleability**: The proof cannot be modified or reused for different cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealingProof {
    /// Groth16 zero-knowledge proof (192 bytes).
    ///
    /// Structure: π = (A, B, C) where:
    /// - A: G1 point (48 bytes compressed)
    /// - B: G2 point (96 bytes compressed)
    /// - C: G1 point (48 bytes compressed)
    ///
    /// In Phase 1, this is a placeholder byte vector.
    pub proof: Vec<u8>,

    /// Pedersen commitments to the dealt cards.
    ///
    /// For Texas Hold'em, this is always exactly 2 cards (hole cards).
    /// Each commitment binds the dealer to a specific card value.
    pub card_commitments: [CardCommitment; 2],

    /// Merkle root of the shuffled 52-card deck.
    ///
    /// This root commits the dealer to the entire deck ordering.
    /// The ZK proof verifies that dealt cards exist as leaves in this Merkle tree.
    pub deck_root: [u8; 32],
}

impl DealingProof {
    /// Expected size of a Groth16 proof in bytes.
    ///
    /// Breakdown:
    /// - A (G1): 48 bytes
    /// - B (G2): 96 bytes
    /// - C (G1): 48 bytes
    /// - Total: 192 bytes
    pub const PROOF_SIZE: usize = 192;

    /// Number of cards dealt in Texas Hold'em hole cards.
    pub const DEALT_CARDS_COUNT: usize = 2;

    /// Size of Merkle root in bytes (SHA-256 hash).
    pub const DECK_ROOT_SIZE: usize = 32;

    /// Create a new DealingProof.
    ///
    /// # Arguments
    ///
    /// * `proof` - The Groth16 proof bytes (192 bytes expected)
    /// * `card_commitments` - Array of exactly 2 card commitments
    /// * `deck_root` - 32-byte Merkle root of the shuffled deck
    ///
    /// # Example
    ///
    /// ```
    /// use linera_poker_shared::zk::{DealingProof, CardCommitment};
    ///
    /// let proof = vec![0u8; DealingProof::PROOF_SIZE];
    /// let commitment1 = CardCommitment::new(vec![0u8; 48], [1u8; 16]);
    /// let commitment2 = CardCommitment::new(vec![0u8; 48], [2u8; 16]);
    /// let deck_root = [0u8; 32];
    ///
    /// let dealing_proof = DealingProof::new(proof, [commitment1, commitment2], deck_root);
    /// ```
    pub fn new(
        proof: Vec<u8>,
        card_commitments: [CardCommitment; 2],
        deck_root: [u8; 32],
    ) -> Self {
        Self {
            proof,
            card_commitments,
            deck_root,
        }
    }

    /// Validate the structural correctness of the proof.
    ///
    /// Checks that the proof has the expected size and all commitments are valid.
    /// In Phase 2, this will also validate elliptic curve point encodings.
    ///
    /// # Returns
    ///
    /// `true` if the proof structure is valid, `false` otherwise.
    pub fn is_structurally_valid(&self) -> bool {
        self.proof.len() == Self::PROOF_SIZE
            && self.card_commitments.iter().all(|c| c.is_valid())
    }
}

// ============================================================================
// REVEAL PROOF (Showdown Phase ZK-SNARK)
// ============================================================================

/// Zero-knowledge proof that revealed cards match previously committed cards.
///
/// This proof is generated by a player during showdown when they reveal their
/// hole cards. It proves that:
///
/// 1. The revealed cards match the Pedersen commitments from the dealing phase
/// 2. The player knows the valid opening (card + randomness) for each commitment
/// 3. The cards were not tampered with or substituted
///
/// # Circuit Public Inputs
///
/// - The original Pedersen commitments (from dealing phase)
/// - The revealed card values
///
/// # Circuit Private Inputs (Witness)
///
/// - The randomness (blinding factors) used in the Pedersen commitments
///
/// # Phase 1 vs Phase 2
///
/// - **Phase 1 (current)**: `proof` is a mock 192-byte blob
/// - **Phase 2**: `proof` will be a valid Groth16 proof
///
/// # Security Guarantees
///
/// - **Soundness**: A player cannot produce a valid proof for cards different
///   from those they were originally dealt
/// - **Zero-Knowledge**: During dealing, commitments reveal no information.
///   At showdown, cards are revealed but the proof doesn't leak any additional info.
/// - **Binding**: The commitment scheme ensures cards cannot be changed after dealing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevealProof {
    /// Groth16 zero-knowledge proof (192 bytes).
    ///
    /// Proves knowledge of valid openings (randomness) for the card commitments
    /// without revealing the randomness before showdown.
    ///
    /// In Phase 1, this is a placeholder byte vector.
    pub proof: Vec<u8>,

    /// The actual card values being revealed.
    ///
    /// For Texas Hold'em, this is always exactly 2 cards.
    /// These are the player's hole cards that were previously committed.
    pub cards: Vec<Card>,

    /// Blinding factors (randomness) used in the Pedersen commitments.
    ///
    /// Each element corresponds to one card and contains the randomness `r`
    /// such that `Commitment = card_index * G + r * H`.
    ///
    /// In BLS12-381, each randomness value is a scalar (32 bytes).
    /// Revealing this allows anyone to verify the commitment opening.
    pub randomness: Vec<Vec<u8>>,
}

impl RevealProof {
    /// Expected size of a Groth16 proof in bytes.
    pub const PROOF_SIZE: usize = 192;

    /// Number of cards revealed in Texas Hold'em.
    pub const REVEALED_CARDS_COUNT: usize = 2;

    /// Size of each randomness scalar in bytes (BLS12-381 scalar field).
    pub const RANDOMNESS_SIZE: usize = 32;

    /// Create a new RevealProof.
    ///
    /// # Arguments
    ///
    /// * `proof` - The Groth16 proof bytes (192 bytes expected)
    /// * `cards` - The revealed cards (exactly 2 for Texas Hold'em)
    /// * `randomness` - The Pedersen randomness for each card (32 bytes each)
    ///
    /// # Example
    ///
    /// ```
    /// use linera_poker_shared::zk::RevealProof;
    /// use linera_poker_shared::{Card, Suit, Rank};
    ///
    /// let proof = vec![0u8; RevealProof::PROOF_SIZE];
    /// let cards = vec![
    ///     Card::new(Suit::Hearts, Rank::Ace),
    ///     Card::new(Suit::Spades, Rank::King),
    /// ];
    /// let randomness = vec![vec![0u8; 32], vec![1u8; 32]];
    ///
    /// let reveal_proof = RevealProof::new(proof, cards, randomness);
    /// ```
    pub fn new(proof: Vec<u8>, cards: Vec<Card>, randomness: Vec<Vec<u8>>) -> Self {
        Self {
            proof,
            cards,
            randomness,
        }
    }

    /// Validate the structural correctness of the proof.
    ///
    /// Checks that:
    /// - Proof has expected size
    /// - Correct number of cards
    /// - Randomness array matches card count
    /// - Each randomness has correct size
    ///
    /// # Returns
    ///
    /// `true` if the proof structure is valid, `false` otherwise.
    pub fn is_structurally_valid(&self) -> bool {
        self.proof.len() == Self::PROOF_SIZE
            && self.cards.len() == Self::REVEALED_CARDS_COUNT
            && self.randomness.len() == Self::REVEALED_CARDS_COUNT
            && self
                .randomness
                .iter()
                .all(|r| r.len() == Self::RANDOMNESS_SIZE)
    }
}

// ============================================================================
// POKER PROOF PARAMETERS (Verification Keys)
// ============================================================================

/// Cryptographic parameters for verifying ZK-SNARKs in the poker protocol.
///
/// This structure contains the verification keys for both the dealing circuit
/// and the reveal circuit. These keys are generated during a trusted setup
/// ceremony and must be available to all players for proof verification.
///
/// # Trusted Setup
///
/// Groth16 requires a trusted setup that generates:
/// - **Proving key** (pk): Used by the prover to generate proofs
/// - **Verification key** (vk): Used by verifiers to check proofs
///
/// The setup must be performed honestly, or the security of the entire
/// system is compromised. In production, use a multi-party computation (MPC)
/// ceremony to generate these keys.
///
/// # Phase 1 vs Phase 2
///
/// - **Phase 1 (current)**: Verification keys are mock byte blobs
/// - **Phase 2**: Keys will be actual BLS12-381 verification keys from trusted setup
///
/// # Key Structure
///
/// Each verification key contains:
/// - Alpha (G1 point)
/// - Beta (G2 point)
/// - Gamma (G2 point)
/// - Delta (G2 point)
/// - Gamma_ABC (vector of G1 points, one per public input)
///
/// Total size: ~300-500 bytes depending on number of public inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokerProofParams {
    /// Verification key for the dealing circuit.
    ///
    /// Used to verify DealingProof instances. Public inputs for this circuit:
    /// - Deck Merkle root (1 field element)
    /// - Card commitments (2 G1 points = 2 field elements each)
    ///
    /// In Phase 1, this is a placeholder byte vector.
    pub dealing_vk: Vec<u8>,

    /// Verification key for the reveal circuit.
    ///
    /// Used to verify RevealProof instances. Public inputs for this circuit:
    /// - Original commitments (2 G1 points)
    /// - Revealed card indices (2 field elements)
    ///
    /// In Phase 1, this is a placeholder byte vector.
    pub reveal_vk: Vec<u8>,
}

impl PokerProofParams {
    /// Expected size range for a verification key in bytes.
    ///
    /// Actual size depends on the number of public inputs in the circuit.
    /// This is an approximate range for validation purposes.
    pub const VK_MIN_SIZE: usize = 200;
    pub const VK_MAX_SIZE: usize = 1000;

    /// Create new PokerProofParams.
    ///
    /// # Arguments
    ///
    /// * `dealing_vk` - Verification key for the dealing circuit
    /// * `reveal_vk` - Verification key for the reveal circuit
    ///
    /// # Example
    ///
    /// ```
    /// use linera_poker_shared::zk::PokerProofParams;
    ///
    /// let dealing_vk = vec![0u8; 300];
    /// let reveal_vk = vec![0u8; 300];
    ///
    /// let params = PokerProofParams::new(dealing_vk, reveal_vk);
    /// ```
    pub fn new(dealing_vk: Vec<u8>, reveal_vk: Vec<u8>) -> Self {
        Self {
            dealing_vk,
            reveal_vk,
        }
    }

    /// Validate that the parameters are structurally sound.
    ///
    /// Checks that both verification keys are non-empty and within expected size range.
    /// In Phase 2, this will also validate the elliptic curve point encodings.
    ///
    /// # Returns
    ///
    /// `true` if parameters appear valid, `false` otherwise.
    pub fn is_valid(&self) -> bool {
        self.dealing_vk.len() >= Self::VK_MIN_SIZE
            && self.dealing_vk.len() <= Self::VK_MAX_SIZE
            && self.reveal_vk.len() >= Self::VK_MIN_SIZE
            && self.reveal_vk.len() <= Self::VK_MAX_SIZE
    }
}

// ============================================================================
// VERIFICATION FUNCTIONS (Phase 1: MOCK IMPLEMENTATION)
// ============================================================================

/// Verify a dealing proof.
///
/// # Phase 1 Implementation (MOCK VERSION)
///
/// **WARNING**: This is a MOCK implementation for Phase 1 testing.
/// It performs only basic structural validation and DOES NOT provide
/// cryptographic security. Any structurally valid proof will be accepted.
///
/// # Phase 2 Implementation
///
/// Phase 2 will implement real Groth16 verification:
///
/// 1. Parse the proof into (A, B, C) elliptic curve points
/// 2. Parse public inputs (deck root, commitments)
/// 3. Perform the Groth16 pairing check:
///    ```text
///    e(A, B) = e(alpha, beta) * e(pub_inputs * gamma_abc, gamma) * e(C, delta)
///    ```
/// 4. Return true iff the pairing equation holds
///
/// # Arguments
///
/// * `proof` - The dealing proof to verify
/// * `params` - The poker proof parameters containing verification keys
///
/// # Returns
///
/// `true` if the proof is valid (in Phase 1: structurally valid), `false` otherwise.
///
/// # Example
///
/// ```
/// use linera_poker_shared::zk::{DealingProof, PokerProofParams, CardCommitment, verify_dealing_proof};
///
/// let proof = vec![0u8; DealingProof::PROOF_SIZE];
/// let commitment1 = CardCommitment::new(vec![0u8; 48], [1u8; 16]);
/// let commitment2 = CardCommitment::new(vec![0u8; 48], [2u8; 16]);
/// let deck_root = [0u8; 32];
/// let dealing_proof = DealingProof::new(proof, [commitment1, commitment2], deck_root);
///
/// let params = PokerProofParams::new(vec![0u8; 300], vec![0u8; 300]);
///
/// let is_valid = verify_dealing_proof(&dealing_proof, &params);
/// assert!(is_valid); // In Phase 1, accepts all structurally valid proofs
/// ```
pub fn verify_dealing_proof(proof: &DealingProof, params: &PokerProofParams) -> bool {
    // Phase 1 MOCK: Only basic structural validation
    // Phase 2 TODO: Implement real Groth16 verification using arkworks-rs
    //
    // Real implementation will:
    // 1. Deserialize proof into (A, B, C) points
    // 2. Compute public input encoding
    // 3. Perform Groth16 pairing check
    // 4. Return pairing result

    // Basic structural checks that will also be in Phase 2
    if !proof.is_structurally_valid() {
        return false;
    }

    if !params.is_valid() {
        return false;
    }

    // MOCK: Accept all structurally valid proofs
    // In Phase 2, this will be replaced with actual cryptographic verification
    true
}

/// Verify a reveal proof against stored commitments.
///
/// # Phase 1 Implementation (MOCK VERSION)
///
/// **WARNING**: This is a MOCK implementation for Phase 1 testing.
/// It performs only basic structural validation and DOES NOT provide
/// cryptographic security. Any structurally valid proof will be accepted.
///
/// # Phase 2 Implementation
///
/// Phase 2 will implement real Groth16 verification and commitment checking:
///
/// 1. Verify the Groth16 proof using the reveal verification key
/// 2. Recompute commitments from revealed cards and randomness
/// 3. Compare recomputed commitments with stored commitments
/// 4. Return true iff proof is valid AND commitments match
///
/// This ensures that:
/// - The player knows valid openings for the commitments
/// - The revealed cards match exactly what was dealt
/// - No card substitution has occurred
///
/// # Arguments
///
/// * `proof` - The reveal proof to verify
/// * `stored_commitments` - The original commitments from the dealing phase
/// * `params` - The poker proof parameters containing verification keys
///
/// # Returns
///
/// `true` if the proof is valid and cards match commitments (in Phase 1: structurally valid), `false` otherwise.
///
/// # Example
///
/// ```
/// use linera_poker_shared::zk::{RevealProof, CardCommitment, PokerProofParams, verify_reveal_proof};
/// use linera_poker_shared::{Card, Suit, Rank};
///
/// let proof = vec![0u8; RevealProof::PROOF_SIZE];
/// let cards = vec![
///     Card::new(Suit::Hearts, Rank::Ace),
///     Card::new(Suit::Spades, Rank::King),
/// ];
/// let randomness = vec![vec![0u8; 32], vec![1u8; 32]];
/// let reveal_proof = RevealProof::new(proof, cards, randomness);
///
/// let commitment1 = CardCommitment::new(vec![0u8; 48], [1u8; 16]);
/// let commitment2 = CardCommitment::new(vec![0u8; 48], [2u8; 16]);
/// let stored_commitments = [commitment1, commitment2];
///
/// let params = PokerProofParams::new(vec![0u8; 300], vec![0u8; 300]);
///
/// let is_valid = verify_reveal_proof(&reveal_proof, &stored_commitments, &params);
/// assert!(is_valid); // In Phase 1, accepts all structurally valid proofs
/// ```
pub fn verify_reveal_proof(
    proof: &RevealProof,
    stored_commitments: &[CardCommitment; 2],
    params: &PokerProofParams,
) -> bool {
    // Phase 1 MOCK: Only basic structural validation
    // Phase 2 TODO: Implement real Groth16 verification and commitment opening check
    //
    // Real implementation will:
    // 1. Verify Groth16 proof
    // 2. Recompute commitments: C = card_index * G + randomness * H
    // 3. Compare with stored commitments
    // 4. Return true iff proof valid AND commitments match

    // Basic structural checks that will also be in Phase 2
    if !proof.is_structurally_valid() {
        return false;
    }

    if !stored_commitments.iter().all(|c| c.is_valid()) {
        return false;
    }

    if !params.is_valid() {
        return false;
    }

    // MOCK: Accept all structurally valid proofs
    // In Phase 2, this will be replaced with actual cryptographic verification
    true
}

// ============================================================================
// HELPER FUNCTIONS (For Testing)
// ============================================================================

/// Create a mock dealing proof for testing.
///
/// This generates a structurally valid dealing proof with placeholder
/// cryptographic values. Useful for integration testing the poker protocol
/// without requiring real ZK proof generation.
///
/// # Arguments
///
/// * `cards` - The cards being "dealt" (for testing purposes)
///
/// # Returns
///
/// A mock `DealingProof` that will pass structural validation.
///
/// # Example
///
/// ```
/// use linera_poker_shared::zk::create_mock_dealing_proof;
/// use linera_poker_shared::{Card, Suit, Rank};
///
/// let cards = [
///     Card::new(Suit::Hearts, Rank::Ace),
///     Card::new(Suit::Spades, Rank::King),
/// ];
///
/// let proof = create_mock_dealing_proof(&cards);
/// assert!(proof.is_structurally_valid());
/// ```
pub fn create_mock_dealing_proof(cards: &[Card; 2]) -> DealingProof {
    // Create mock commitments based on card indices
    let commitment1 = CardCommitment::new(
        vec![cards[0].to_index(); CardCommitment::COMMITMENT_SIZE],
        [1u8; CardCommitment::NONCE_SIZE],
    );
    let commitment2 = CardCommitment::new(
        vec![cards[1].to_index(); CardCommitment::COMMITMENT_SIZE],
        [2u8; CardCommitment::NONCE_SIZE],
    );

    // Create mock proof (all zeros)
    let proof = vec![0u8; DealingProof::PROOF_SIZE];

    // Create mock deck root (deterministic based on cards for testing)
    let mut deck_root = [0u8; DealingProof::DECK_ROOT_SIZE];
    deck_root[0] = cards[0].to_index();
    deck_root[1] = cards[1].to_index();

    DealingProof::new(proof, [commitment1, commitment2], deck_root)
}

/// Create a mock reveal proof for testing.
///
/// This generates a structurally valid reveal proof with placeholder
/// cryptographic values. Useful for integration testing showdown logic
/// without requiring real ZK proof generation.
///
/// # Arguments
///
/// * `cards` - The cards being revealed
/// * `commitments` - The original commitments (used to derive mock randomness)
///
/// # Returns
///
/// A mock `RevealProof` that will pass structural validation.
///
/// # Example
///
/// ```
/// use linera_poker_shared::zk::{create_mock_dealing_proof, create_mock_reveal_proof};
/// use linera_poker_shared::{Card, Suit, Rank};
///
/// let cards = [
///     Card::new(Suit::Hearts, Rank::Ace),
///     Card::new(Suit::Spades, Rank::King),
/// ];
///
/// let dealing_proof = create_mock_dealing_proof(&cards);
/// let reveal_proof = create_mock_reveal_proof(&cards, &dealing_proof.card_commitments);
///
/// assert!(reveal_proof.is_structurally_valid());
/// assert_eq!(reveal_proof.cards.len(), 2);
/// ```
pub fn create_mock_reveal_proof(
    cards: &[Card; 2],
    commitments: &[CardCommitment; 2],
) -> RevealProof {
    // Create mock proof (all zeros)
    let proof = vec![0u8; RevealProof::PROOF_SIZE];

    // Create mock randomness (derived from commitment nonces for consistency)
    let randomness = vec![
        commitments[0].nonce.repeat(2), // 16 * 2 = 32 bytes
        commitments[1].nonce.repeat(2),
    ];

    RevealProof::new(proof, cards.to_vec(), randomness)
}

/// Create mock poker proof parameters for testing.
///
/// Generates placeholder verification keys that will pass structural validation.
/// Useful for testing the poker protocol without a real trusted setup.
///
/// # Returns
///
/// Mock `PokerProofParams` with placeholder verification keys.
///
/// # Example
///
/// ```
/// use linera_poker_shared::zk::create_mock_params;
///
/// let params = create_mock_params();
/// assert!(params.is_valid());
/// ```
pub fn create_mock_params() -> PokerProofParams {
    // Create mock verification keys (300 bytes each)
    let dealing_vk = vec![0u8; 300];
    let reveal_vk = vec![1u8; 300];

    PokerProofParams::new(dealing_vk, reveal_vk)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Card, Rank, Suit};

    #[test]
    fn test_card_commitment_valid() {
        let commitment = vec![0u8; CardCommitment::COMMITMENT_SIZE];
        let nonce = [42u8; CardCommitment::NONCE_SIZE];
        let card_commitment = CardCommitment::new(commitment, nonce);

        assert!(card_commitment.is_valid());
    }

    #[test]
    fn test_card_commitment_invalid_size() {
        let commitment = vec![0u8; 32]; // Wrong size (should be 48)
        let nonce = [42u8; CardCommitment::NONCE_SIZE];
        let card_commitment = CardCommitment::new(commitment, nonce);

        assert!(!card_commitment.is_valid());
    }

    #[test]
    fn test_dealing_proof_structural_validation() {
        let proof = vec![0u8; DealingProof::PROOF_SIZE];
        let commitment1 = CardCommitment::new(vec![0u8; 48], [1u8; 16]);
        let commitment2 = CardCommitment::new(vec![0u8; 48], [2u8; 16]);
        let deck_root = [0u8; 32];

        let dealing_proof = DealingProof::new(proof, [commitment1, commitment2], deck_root);
        assert!(dealing_proof.is_structurally_valid());
    }

    #[test]
    fn test_dealing_proof_invalid_proof_size() {
        let proof = vec![0u8; 100]; // Wrong size
        let commitment1 = CardCommitment::new(vec![0u8; 48], [1u8; 16]);
        let commitment2 = CardCommitment::new(vec![0u8; 48], [2u8; 16]);
        let deck_root = [0u8; 32];

        let dealing_proof = DealingProof::new(proof, [commitment1, commitment2], deck_root);
        assert!(!dealing_proof.is_structurally_valid());
    }

    #[test]
    fn test_dealing_proof_invalid_commitment() {
        let proof = vec![0u8; DealingProof::PROOF_SIZE];
        let commitment1 = CardCommitment::new(vec![0u8; 32], [1u8; 16]); // Wrong size
        let commitment2 = CardCommitment::new(vec![0u8; 48], [2u8; 16]);
        let deck_root = [0u8; 32];

        let dealing_proof = DealingProof::new(proof, [commitment1, commitment2], deck_root);
        assert!(!dealing_proof.is_structurally_valid());
    }

    #[test]
    fn test_reveal_proof_structural_validation() {
        let proof = vec![0u8; RevealProof::PROOF_SIZE];
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let randomness = vec![vec![0u8; 32], vec![1u8; 32]];

        let reveal_proof = RevealProof::new(proof, cards, randomness);
        assert!(reveal_proof.is_structurally_valid());
    }

    #[test]
    fn test_reveal_proof_wrong_card_count() {
        let proof = vec![0u8; RevealProof::PROOF_SIZE];
        let cards = vec![Card::new(Suit::Hearts, Rank::Ace)]; // Only 1 card
        let randomness = vec![vec![0u8; 32]];

        let reveal_proof = RevealProof::new(proof, cards, randomness);
        assert!(!reveal_proof.is_structurally_valid());
    }

    #[test]
    fn test_reveal_proof_mismatched_randomness() {
        let proof = vec![0u8; RevealProof::PROOF_SIZE];
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let randomness = vec![vec![0u8; 32]]; // Only 1 randomness, should be 2

        let reveal_proof = RevealProof::new(proof, cards, randomness);
        assert!(!reveal_proof.is_structurally_valid());
    }

    #[test]
    fn test_reveal_proof_wrong_randomness_size() {
        let proof = vec![0u8; RevealProof::PROOF_SIZE];
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let randomness = vec![vec![0u8; 16], vec![1u8; 32]]; // First one wrong size

        let reveal_proof = RevealProof::new(proof, cards, randomness);
        assert!(!reveal_proof.is_structurally_valid());
    }

    #[test]
    fn test_poker_proof_params_valid() {
        let dealing_vk = vec![0u8; 300];
        let reveal_vk = vec![1u8; 300];
        let params = PokerProofParams::new(dealing_vk, reveal_vk);

        assert!(params.is_valid());
    }

    #[test]
    fn test_poker_proof_params_too_small() {
        let dealing_vk = vec![0u8; 100]; // Too small
        let reveal_vk = vec![1u8; 300];
        let params = PokerProofParams::new(dealing_vk, reveal_vk);

        assert!(!params.is_valid());
    }

    #[test]
    fn test_poker_proof_params_too_large() {
        let dealing_vk = vec![0u8; 300];
        let reveal_vk = vec![1u8; 2000]; // Too large
        let params = PokerProofParams::new(dealing_vk, reveal_vk);

        assert!(!params.is_valid());
    }

    #[test]
    fn test_verify_dealing_proof_mock_accepts_valid() {
        let cards = [
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let dealing_proof = create_mock_dealing_proof(&cards);
        let params = create_mock_params();

        assert!(verify_dealing_proof(&dealing_proof, &params));
    }

    #[test]
    fn test_verify_dealing_proof_rejects_invalid_structure() {
        let proof = vec![0u8; 100]; // Wrong size
        let commitment1 = CardCommitment::new(vec![0u8; 48], [1u8; 16]);
        let commitment2 = CardCommitment::new(vec![0u8; 48], [2u8; 16]);
        let deck_root = [0u8; 32];
        let dealing_proof = DealingProof::new(proof, [commitment1, commitment2], deck_root);
        let params = create_mock_params();

        assert!(!verify_dealing_proof(&dealing_proof, &params));
    }

    #[test]
    fn test_verify_reveal_proof_mock_accepts_valid() {
        let cards = [
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let dealing_proof = create_mock_dealing_proof(&cards);
        let reveal_proof = create_mock_reveal_proof(&cards, &dealing_proof.card_commitments);
        let params = create_mock_params();

        assert!(verify_reveal_proof(
            &reveal_proof,
            &dealing_proof.card_commitments,
            &params
        ));
    }

    #[test]
    fn test_verify_reveal_proof_rejects_invalid_structure() {
        let proof = vec![0u8; RevealProof::PROOF_SIZE];
        let cards = vec![Card::new(Suit::Hearts, Rank::Ace)]; // Wrong count
        let randomness = vec![vec![0u8; 32]];
        let reveal_proof = RevealProof::new(proof, cards, randomness);

        let commitment1 = CardCommitment::new(vec![0u8; 48], [1u8; 16]);
        let commitment2 = CardCommitment::new(vec![0u8; 48], [2u8; 16]);
        let commitments = [commitment1, commitment2];
        let params = create_mock_params();

        assert!(!verify_reveal_proof(&reveal_proof, &commitments, &params));
    }

    #[test]
    fn test_create_mock_dealing_proof_creates_valid_proof() {
        let cards = [
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let proof = create_mock_dealing_proof(&cards);

        assert!(proof.is_structurally_valid());
        assert_eq!(proof.card_commitments.len(), 2);
        assert_eq!(proof.proof.len(), DealingProof::PROOF_SIZE);
        assert_eq!(proof.deck_root.len(), DealingProof::DECK_ROOT_SIZE);
    }

    #[test]
    fn test_create_mock_reveal_proof_creates_valid_proof() {
        let cards = [
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let dealing_proof = create_mock_dealing_proof(&cards);
        let reveal_proof = create_mock_reveal_proof(&cards, &dealing_proof.card_commitments);

        assert!(reveal_proof.is_structurally_valid());
        assert_eq!(reveal_proof.cards.len(), 2);
        assert_eq!(reveal_proof.cards[0], cards[0]);
        assert_eq!(reveal_proof.cards[1], cards[1]);
        assert_eq!(reveal_proof.randomness.len(), 2);
    }

    #[test]
    fn test_card_commitment_equality() {
        let commitment1 = CardCommitment::new(vec![42u8; 48], [1u8; 16]);
        let commitment2 = CardCommitment::new(vec![42u8; 48], [1u8; 16]);
        let commitment3 = CardCommitment::new(vec![43u8; 48], [1u8; 16]);

        assert_eq!(commitment1, commitment2);
        assert_ne!(commitment1, commitment3);
    }

    #[test]
    fn test_dealing_proof_cloning() {
        let cards = [
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let proof = create_mock_dealing_proof(&cards);
        let cloned = proof.clone();

        assert_eq!(proof.proof, cloned.proof);
        assert_eq!(proof.card_commitments, cloned.card_commitments);
        assert_eq!(proof.deck_root, cloned.deck_root);
    }

    #[test]
    fn test_reveal_proof_cloning() {
        let cards = [
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let dealing_proof = create_mock_dealing_proof(&cards);
        let reveal_proof = create_mock_reveal_proof(&cards, &dealing_proof.card_commitments);
        let cloned = reveal_proof.clone();

        assert_eq!(reveal_proof.proof, cloned.proof);
        assert_eq!(reveal_proof.cards, cloned.cards);
        assert_eq!(reveal_proof.randomness, cloned.randomness);
    }
}

// ============================================================================
// KEY LOADING (Phase 2 - Trusted Setup)
// ============================================================================

/// Error type for key loading operations
#[derive(Debug)]
pub enum KeyLoadError {
    IoError(std::io::Error),
    DeserializationError(String),
    InvalidKeyFormat(String),
}

impl std::fmt::Display for KeyLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyLoadError::IoError(e) => write!(f, "I/O error: {}", e),
            KeyLoadError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            KeyLoadError::InvalidKeyFormat(msg) => write!(f, "Invalid key format: {}", msg),
        }
    }
}

impl std::error::Error for KeyLoadError {}

impl From<std::io::Error> for KeyLoadError {
    fn from(e: std::io::Error) -> Self {
        KeyLoadError::IoError(e)
    }
}

#[cfg(not(target_arch = "wasm32"))]
use ark_bls12_381::Bls12_381;
#[cfg(not(target_arch = "wasm32"))]
use ark_groth16::{ProvingKey, VerifyingKey};
#[cfg(not(target_arch = "wasm32"))]
use ark_serialize::CanonicalDeserialize;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

/// Load the dealing circuit proving key from disk.
///
/// This function reads the proving key generated during the trusted setup
/// ceremony and deserializes it for use in proof generation.
///
/// # Arguments
///
/// * `path` - Path to the proving key file (typically `keys/dealing.pk`)
///
/// # Returns
///
/// The deserialized proving key, or an error if loading fails.
///
/// # Example
///
/// ```no_run
/// use linera_poker_shared::zk::load_dealing_proving_key;
/// use std::path::Path;
///
/// let pk = load_dealing_proving_key(Path::new("keys/dealing.pk")).unwrap();
/// // Use pk to generate proofs...
/// ```
///
/// # Errors
///
/// - `KeyLoadError::IoError` if the file cannot be read
/// - `KeyLoadError::DeserializationError` if the key format is invalid
#[cfg(not(target_arch = "wasm32"))]
pub fn load_dealing_proving_key(path: &Path) -> Result<ProvingKey<Bls12_381>, KeyLoadError> {
    let bytes = std::fs::read(path)?;
    ProvingKey::deserialize_compressed(&bytes[..])
        .map_err(|e| KeyLoadError::DeserializationError(format!("{:?}", e)))
}

/// Load the dealing circuit verifying key from disk.
///
/// This function reads the verifying key generated during the trusted setup
/// ceremony and deserializes it for use in proof verification.
///
/// # Arguments
///
/// * `path` - Path to the verifying key file (typically `keys/dealing.vk`)
///
/// # Returns
///
/// The deserialized verifying key, or an error if loading fails.
///
/// # Example
///
/// ```no_run
/// use linera_poker_shared::zk::load_dealing_verifying_key;
/// use std::path::Path;
///
/// let vk = load_dealing_verifying_key(Path::new("keys/dealing.vk")).unwrap();
/// // Use vk to verify proofs...
/// ```
///
/// # Errors
///
/// - `KeyLoadError::IoError` if the file cannot be read
/// - `KeyLoadError::DeserializationError` if the key format is invalid
#[cfg(not(target_arch = "wasm32"))]
pub fn load_dealing_verifying_key(path: &Path) -> Result<VerifyingKey<Bls12_381>, KeyLoadError> {
    let bytes = std::fs::read(path)?;
    VerifyingKey::deserialize_compressed(&bytes[..])
        .map_err(|e| KeyLoadError::DeserializationError(format!("{:?}", e)))
}

/// Load the reveal circuit proving key from disk.
///
/// This function reads the proving key generated during the trusted setup
/// ceremony and deserializes it for use in proof generation.
///
/// # Arguments
///
/// * `path` - Path to the proving key file (typically `keys/reveal.pk`)
///
/// # Returns
///
/// The deserialized proving key, or an error if loading fails.
///
/// # Example
///
/// ```no_run
/// use linera_poker_shared::zk::load_reveal_proving_key;
/// use std::path::Path;
///
/// let pk = load_reveal_proving_key(Path::new("keys/reveal.pk")).unwrap();
/// // Use pk to generate proofs...
/// ```
///
/// # Errors
///
/// - `KeyLoadError::IoError` if the file cannot be read
/// - `KeyLoadError::DeserializationError` if the key format is invalid
#[cfg(not(target_arch = "wasm32"))]
pub fn load_reveal_proving_key(path: &Path) -> Result<ProvingKey<Bls12_381>, KeyLoadError> {
    let bytes = std::fs::read(path)?;
    ProvingKey::deserialize_compressed(&bytes[..])
        .map_err(|e| KeyLoadError::DeserializationError(format!("{:?}", e)))
}

/// Load the reveal circuit verifying key from disk.
///
/// This function reads the verifying key generated during the trusted setup
/// ceremony and deserializes it for use in proof verification.
///
/// # Arguments
///
/// * `path` - Path to the verifying key file (typically `keys/reveal.vk`)
///
/// # Returns
///
/// The deserialized verifying key, or an error if loading fails.
///
/// # Example
///
/// ```no_run
/// use linera_poker_shared::zk::load_reveal_verifying_key;
/// use std::path::Path;
///
/// let vk = load_reveal_verifying_key(Path::new("keys/reveal.vk")).unwrap();
/// // Use vk to verify proofs...
/// ```
///
/// # Errors
///
/// - `KeyLoadError::IoError` if the file cannot be read
/// - `KeyLoadError::DeserializationError` if the key format is invalid
#[cfg(not(target_arch = "wasm32"))]
pub fn load_reveal_verifying_key(path: &Path) -> Result<VerifyingKey<Bls12_381>, KeyLoadError> {
    let bytes = std::fs::read(path)?;
    VerifyingKey::deserialize_compressed(&bytes[..])
        .map_err(|e| KeyLoadError::DeserializationError(format!("{:?}", e)))
}

/// Load all keys required for the poker protocol.
///
/// Convenience function that loads both proving and verifying keys for
/// dealing and reveal circuits in a single call.
///
/// # Arguments
///
/// * `keys_dir` - Path to the directory containing key files
///
/// # Returns
///
/// A tuple of (dealing_pk, dealing_vk, reveal_pk, reveal_vk), or an error
/// if any key fails to load.
///
/// # Example
///
/// ```no_run
/// use linera_poker_shared::zk::load_all_keys;
/// use std::path::Path;
///
/// let (d_pk, d_vk, r_pk, r_vk) = load_all_keys(Path::new("keys")).unwrap();
/// // Keys are ready to use for proving and verifying...
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn load_all_keys(
    keys_dir: &Path,
) -> Result<
    (
        ProvingKey<Bls12_381>,
        VerifyingKey<Bls12_381>,
        ProvingKey<Bls12_381>,
        VerifyingKey<Bls12_381>,
    ),
    KeyLoadError,
> {
    let dealing_pk = load_dealing_proving_key(&keys_dir.join("dealing.pk"))?;
    let dealing_vk = load_dealing_verifying_key(&keys_dir.join("dealing.vk"))?;
    let reveal_pk = load_reveal_proving_key(&keys_dir.join("reveal.pk"))?;
    let reveal_vk = load_reveal_verifying_key(&keys_dir.join("reveal.vk"))?;

    Ok((dealing_pk, dealing_vk, reveal_pk, reveal_vk))
}

// ============================================================================
// PHASE 4: REAL PROOF GENERATION (Native Only)
// ============================================================================

/// Error type for proof generation operations
#[derive(Debug)]
pub enum ProofError {
    /// Circuit synthesis failed
    SynthesisError(String),
    /// Proof generation failed
    ProvingError(String),
    /// Verification failed
    VerificationError(String),
    /// Serialization error
    SerializationError(String),
    /// Invalid input
    InvalidInput(String),
}

impl std::fmt::Display for ProofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofError::SynthesisError(msg) => write!(f, "Circuit synthesis error: {}", msg),
            ProofError::ProvingError(msg) => write!(f, "Proof generation error: {}", msg),
            ProofError::VerificationError(msg) => write!(f, "Verification error: {}", msg),
            ProofError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ProofError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for ProofError {}

/// Generate a dealing proof (native only, not for WASM)
///
/// This function generates a Groth16 proof that the dealer has honestly
/// committed to 2 distinct cards from the shuffled deck.
///
/// # Arguments
///
/// * `cards` - The two cards being dealt
/// * `card_indices` - Positions of cards in the shuffled deck (0-51)
/// * `deck_root` - Merkle root of the 52-card deck
/// * `randomness` - Blinding factors for Pedersen commitments
/// * `merkle_proofs` - Proofs that cards exist in the deck
/// * `proving_key` - The Groth16 proving key for the dealing circuit
///
/// # Returns
///
/// A `DealingProof` containing the Groth16 proof and card commitments.
#[cfg(not(target_arch = "wasm32"))]
pub fn generate_dealing_proof(
    cards: &[crate::Card; 2],
    card_indices: &[u8; 2],
    deck_root: &[u8; 32],
    randomness: &[ark_bls12_381::Fr; 2],
    merkle_proofs: &[crate::circuits::MerkleProof; 2],
    proving_key: &ProvingKey<Bls12_381>,
) -> Result<DealingProof, ProofError> {
    use ark_groth16::Groth16;
    use ark_serialize::CanonicalSerialize;
    use ark_std::rand::SeedableRng;

    // Import the circuit
    use crate::circuits::DealingCircuit;

    // Create card commitments
    let mut commitments = Vec::new();
    for (i, card) in cards.iter().enumerate() {
        let commitment = create_pedersen_commitment(card.to_index(), &randomness[i])?;
        let nonce = generate_nonce(card.to_index(), i as u8);
        commitments.push(CardCommitment::new(commitment, nonce));
    }

    // Create the circuit with witness
    let circuit = DealingCircuit::new_with_witness(
        *deck_root,
        [commitments[0].commitment.clone(), commitments[1].commitment.clone()],
        *card_indices,
        [cards[0].to_index(), cards[1].to_index()],
        *randomness,
        merkle_proofs.clone(),
    );

    // Generate the proof
    let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
    let proof = Groth16::<Bls12_381>::prove(proving_key, circuit, &mut rng)
        .map_err(|e| ProofError::ProvingError(format!("{:?}", e)))?;

    // Serialize the proof
    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes)
        .map_err(|e| ProofError::SerializationError(format!("{:?}", e)))?;

    Ok(DealingProof::new(
        proof_bytes,
        [commitments[0].clone(), commitments[1].clone()],
        *deck_root,
    ))
}

/// Generate a reveal proof (native only, not for WASM)
///
/// This function generates a Groth16 proof that the revealed cards
/// match the commitments from the dealing phase.
///
/// # Arguments
///
/// * `cards` - The cards being revealed
/// * `commitments` - The original commitments from dealing
/// * `randomness` - The same blinding factors used during dealing
/// * `proving_key` - The Groth16 proving key for the reveal circuit
///
/// # Returns
///
/// A `RevealProof` containing the Groth16 proof and revealed cards.
#[cfg(not(target_arch = "wasm32"))]
pub fn generate_reveal_proof(
    cards: &[crate::Card; 2],
    commitments: &[CardCommitment; 2],
    randomness: &[ark_bls12_381::Fr; 2],
    proving_key: &ProvingKey<Bls12_381>,
) -> Result<RevealProof, ProofError> {
    use ark_groth16::Groth16;
    use ark_serialize::CanonicalSerialize;
    use ark_std::rand::SeedableRng;

    // Import the circuit
    use crate::circuits::RevealCircuit;

    // Create the circuit with witness
    let circuit = RevealCircuit::new_with_witness(
        [commitments[0].commitment.clone(), commitments[1].commitment.clone()],
        [cards[0].to_index(), cards[1].to_index()],
        *randomness,
    );

    // Generate the proof
    let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
    let proof = Groth16::<Bls12_381>::prove(proving_key, circuit, &mut rng)
        .map_err(|e| ProofError::ProvingError(format!("{:?}", e)))?;

    // Serialize the proof
    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes)
        .map_err(|e| ProofError::SerializationError(format!("{:?}", e)))?;

    // Serialize randomness
    let mut randomness_bytes = Vec::new();
    for r in randomness.iter() {
        let mut r_bytes = Vec::new();
        r.serialize_compressed(&mut r_bytes)
            .map_err(|e| ProofError::SerializationError(format!("{:?}", e)))?;
        randomness_bytes.push(r_bytes);
    }

    Ok(RevealProof::new(
        proof_bytes,
        cards.to_vec(),
        randomness_bytes,
    ))
}

/// Create a Pedersen commitment to a card value
///
/// C = value * G + randomness * H
/// where G and H are BLS12-381 generators
#[cfg(not(target_arch = "wasm32"))]
pub fn create_pedersen_commitment(
    card_index: u8,
    randomness: &ark_bls12_381::Fr,
) -> Result<Vec<u8>, ProofError> {
    use ark_bls12_381::{Fr, G1Projective};
    use ark_ec::Group;
    use ark_ff::PrimeField;
    use ark_serialize::CanonicalSerialize;

    // Get G1 generator
    let g = G1Projective::generator();

    // Create a second independent generator H = Hash-to-curve(G)
    // For simplicity, use scalar multiplication with a fixed large scalar
    let h_scalar = Fr::from(0xDEADBEEF_u64);
    let h = g * h_scalar;

    // Compute commitment: C = card_index * G + randomness * H
    let value_scalar = Fr::from(card_index as u64);
    let commitment = g * value_scalar + h * randomness;

    // Serialize to compressed form
    let affine = commitment.into();
    let mut bytes = Vec::new();
    <ark_bls12_381::G1Affine as CanonicalSerialize>::serialize_compressed(&affine, &mut bytes)
        .map_err(|e| ProofError::SerializationError(format!("{:?}", e)))?;

    Ok(bytes)
}

/// Generate a nonce for a card commitment
fn generate_nonce(card_index: u8, position: u8) -> [u8; 16] {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(&[card_index, position]);
    hasher.update(b"linera-poker-nonce-v1");

    // Add some entropy from current time if available
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(duration) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            hasher.update(&duration.as_nanos().to_le_bytes());
        }
    }

    let hash = hasher.finalize();
    let mut nonce = [0u8; 16];
    nonce.copy_from_slice(&hash[..16]);
    nonce
}

// ============================================================================
// PHASE 4: REAL GROTH16 VERIFICATION (WASM Compatible)
// ============================================================================

/// Verify a dealing proof using real Groth16 verification
///
/// This function performs cryptographic verification of the dealing proof
/// using the BLS12-381 pairing-based Groth16 verifier.
///
/// # Arguments
///
/// * `proof` - The dealing proof to verify
/// * `verifying_key_bytes` - Serialized Groth16 verifying key
///
/// # Returns
///
/// `true` if the proof is cryptographically valid, `false` otherwise.
pub fn verify_dealing_proof_real(
    proof: &DealingProof,
    verifying_key_bytes: &[u8],
) -> bool {
    use ark_bls12_381::{Bls12_381, Fr};
    use ark_groth16::{Groth16, Proof, VerifyingKey};
    use ark_serialize::CanonicalDeserialize;
    use ark_snark::SNARK;

    // Structural validation first
    if !proof.is_structurally_valid() {
        return false;
    }

    // Deserialize the verifying key
    let vk = match VerifyingKey::<Bls12_381>::deserialize_compressed(verifying_key_bytes) {
        Ok(vk) => vk,
        Err(_) => return false,
    };

    // Deserialize the proof
    let groth16_proof = match Proof::<Bls12_381>::deserialize_compressed(&proof.proof[..]) {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Construct public inputs
    // Public inputs for dealing circuit:
    // 1. Deck root (as field elements)
    // 2. Card commitments (as field elements)
    let mut public_inputs: Vec<Fr> = Vec::new();

    // Add deck root bytes as field elements
    for byte in proof.deck_root.iter() {
        public_inputs.push(Fr::from(*byte as u64));
    }

    // Add commitment bytes as field elements
    for commitment in proof.card_commitments.iter() {
        for byte in commitment.commitment.iter() {
            public_inputs.push(Fr::from(*byte as u64));
        }
    }

    // Verify the proof
    match Groth16::<Bls12_381>::verify(&vk, &public_inputs, &groth16_proof) {
        Ok(result) => result,
        Err(_) => false,
    }
}

/// Verify a reveal proof using real Groth16 verification
///
/// This function performs cryptographic verification of the reveal proof
/// and checks that the revealed cards match the original commitments.
///
/// # Arguments
///
/// * `proof` - The reveal proof to verify
/// * `stored_commitments` - The original commitments from dealing
/// * `verifying_key_bytes` - Serialized Groth16 verifying key
///
/// # Returns
///
/// `true` if the proof is cryptographically valid and cards match, `false` otherwise.
pub fn verify_reveal_proof_real(
    proof: &RevealProof,
    stored_commitments: &[CardCommitment; 2],
    verifying_key_bytes: &[u8],
) -> bool {
    use ark_bls12_381::{Bls12_381, Fr};
    use ark_groth16::{Groth16, Proof, VerifyingKey};
    use ark_serialize::CanonicalDeserialize;
    use ark_snark::SNARK;

    // Structural validation first
    if !proof.is_structurally_valid() {
        return false;
    }

    if !stored_commitments.iter().all(|c| c.is_valid()) {
        return false;
    }

    // Deserialize the verifying key
    let vk = match VerifyingKey::<Bls12_381>::deserialize_compressed(verifying_key_bytes) {
        Ok(vk) => vk,
        Err(_) => return false,
    };

    // Deserialize the proof
    let groth16_proof = match Proof::<Bls12_381>::deserialize_compressed(&proof.proof[..]) {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Construct public inputs
    // Public inputs for reveal circuit:
    // 1. Card commitments (as field elements)
    // 2. Revealed card values
    let mut public_inputs: Vec<Fr> = Vec::new();

    // Add commitment bytes as field elements
    for commitment in stored_commitments.iter() {
        for byte in commitment.commitment.iter() {
            public_inputs.push(Fr::from(*byte as u64));
        }
    }

    // Add revealed card values
    for card in proof.cards.iter() {
        public_inputs.push(Fr::from(card.to_index() as u64));
    }

    // Verify the proof
    match Groth16::<Bls12_381>::verify(&vk, &public_inputs, &groth16_proof) {
        Ok(result) => result,
        Err(_) => false,
    }
}

// ============================================================================
// EMBEDDED VERIFYING KEYS (For WASM Contracts)
// ============================================================================

/// Embedded dealing verifying key bytes
///
/// This is the serialized verifying key for the dealing circuit.
/// Generated by the trusted setup and embedded here for WASM contract use.
///
/// Size: ~1.2 KB
pub const DEALING_VK_BYTES: &[u8] = include_bytes!("../../keys/dealing.vk");

/// Embedded reveal verifying key bytes
///
/// This is the serialized verifying key for the reveal circuit.
/// Generated by the trusted setup and embedded here for WASM contract use.
///
/// Size: ~0.9 KB
pub const REVEAL_VK_BYTES: &[u8] = include_bytes!("../../keys/reveal.vk");

/// Verify a dealing proof using the embedded verifying key
///
/// Convenience function that uses the embedded verifying key.
/// This is the recommended function for WASM contracts.
pub fn verify_dealing_proof_embedded(proof: &DealingProof) -> bool {
    verify_dealing_proof_real(proof, DEALING_VK_BYTES)
}

/// Verify a reveal proof using the embedded verifying key
///
/// Convenience function that uses the embedded verifying key.
/// This is the recommended function for WASM contracts.
pub fn verify_reveal_proof_embedded(
    proof: &RevealProof,
    stored_commitments: &[CardCommitment; 2],
) -> bool {
    verify_reveal_proof_real(proof, stored_commitments, REVEAL_VK_BYTES)
}
