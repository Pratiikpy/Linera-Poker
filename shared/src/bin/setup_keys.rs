//! Groth16 Trusted Setup Ceremony for Linera Poker

use ark_bls12_381::Bls12_381;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::SeedableRng;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

// Include circuits module inline for this binary
#[path = "../circuits/mod.rs"]
mod circuits;
use circuits::{DealingCircuit, RevealCircuit};

const DEV_SEED: [u8; 32] = [
    0x42, 0x13, 0x37, 0x69, 0x88, 0xAA, 0xBB, 0xCC,
    0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44,
    0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC,
    0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44,
];

#[derive(Debug)]
enum SetupError {
    IoError(std::io::Error),
    SerializationError(ark_serialize::SerializationError),
    VerificationError(String),
}

impl From<std::io::Error> for SetupError {
    fn from(e: std::io::Error) -> Self {
        SetupError::IoError(e)
    }
}

impl From<ark_serialize::SerializationError> for SetupError {
    fn from(e: ark_serialize::SerializationError) -> Self {
        SetupError::SerializationError(e)
    }
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SetupError::IoError(e) => write!(f, "I/O error: {}", e),
            SetupError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            SetupError::VerificationError(msg) => write!(f, "Verification error: {}", msg),
        }
    }
}

impl std::error::Error for SetupError {}

fn save_proving_key(pk: &ProvingKey<Bls12_381>, path: &Path) -> Result<usize, SetupError> {
    let mut bytes = Vec::new();
    pk.serialize_compressed(&mut bytes)?;
    fs::write(path, &bytes)?;
    Ok(bytes.len())
}

fn save_verifying_key(vk: &VerifyingKey<Bls12_381>, path: &Path) -> Result<usize, SetupError> {
    let mut bytes = Vec::new();
    vk.serialize_compressed(&mut bytes)?;
    fs::write(path, &bytes)?;
    Ok(bytes.len())
}

fn load_proving_key(path: &Path) -> Result<ProvingKey<Bls12_381>, SetupError> {
    let bytes = fs::read(path)?;
    Ok(ProvingKey::deserialize_compressed(&bytes[..])?)
}

fn load_verifying_key(path: &Path) -> Result<VerifyingKey<Bls12_381>, SetupError> {
    let bytes = fs::read(path)?;
    Ok(VerifyingKey::deserialize_compressed(&bytes[..])?)
}

fn format_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn compute_checksum(path: &Path) -> Result<String, SetupError> {
    use sha2::{Digest, Sha256};
    let bytes = fs::read(path)?;
    let hash = Sha256::digest(&bytes);
    Ok(hex::encode(hash))
}

fn main() -> Result<(), SetupError> {
    println!("Groth16 Trusted Setup Ceremony - Linera Poker");
    println!("==============================================");
    println!();
    
    let keys_dir = PathBuf::from("keys");
    if !keys_dir.exists() {
        fs::create_dir(&keys_dir)?;
        println!("Created keys/ directory");
    }
    println!();
    
    let mut rng = rand_chacha::ChaCha20Rng::from_seed(DEV_SEED);
    println!("RNG initialized with deterministic seed");
    println!("Seed: {}", hex::encode(&DEV_SEED));
    println!();
    
    println!("1. DEALING CIRCUIT SETUP");
    println!("------------------------");
    let dealing_circuit = DealingCircuit::default();
    println!("Running Groth16 setup...");
    let start = Instant::now();
    let (dealing_pk, dealing_vk) = Groth16::<Bls12_381>::setup(dealing_circuit, &mut rng)
        .map_err(|e| SetupError::VerificationError(format!("Setup failed: {:?}", e)))?;
    println!("Setup completed in {:.2}s", start.elapsed().as_secs_f64());
    
    let dealing_pk_path = keys_dir.join("dealing.pk");
    let dealing_vk_path = keys_dir.join("dealing.vk");
    let pk_size = save_proving_key(&dealing_pk, &dealing_pk_path)?;
    let vk_size = save_verifying_key(&dealing_vk, &dealing_vk_path)?;
    println!("Proving key: {} ({})", dealing_pk_path.display(), format_bytes(pk_size));
    println!("Verifying key: {} ({})", dealing_vk_path.display(), format_bytes(vk_size));
    println!();
    
    println!("2. REVEAL CIRCUIT SETUP");
    println!("-----------------------");
    let reveal_circuit = RevealCircuit::default();
    println!("Running Groth16 setup...");
    let start = Instant::now();
    let (reveal_pk, reveal_vk) = Groth16::<Bls12_381>::setup(reveal_circuit, &mut rng)
        .map_err(|e| SetupError::VerificationError(format!("Setup failed: {:?}", e)))?;
    println!("Setup completed in {:.2}s", start.elapsed().as_secs_f64());
    
    let reveal_pk_path = keys_dir.join("reveal.pk");
    let reveal_vk_path = keys_dir.join("reveal.vk");
    let pk_size = save_proving_key(&reveal_pk, &reveal_pk_path)?;
    let vk_size = save_verifying_key(&reveal_vk, &reveal_vk_path)?;
    println!("Proving key: {} ({})", reveal_pk_path.display(), format_bytes(pk_size));
    println!("Verifying key: {} ({})", reveal_vk_path.display(), format_bytes(vk_size));
    println!();
    
    println!("3. KEY VERIFICATION");
    println!("-------------------");
    let _ = load_proving_key(&dealing_pk_path)?;
    let _ = load_verifying_key(&dealing_vk_path)?;
    let _ = load_proving_key(&reveal_pk_path)?;
    let _ = load_verifying_key(&reveal_vk_path)?;
    println!("All keys loaded successfully");
    println!();
    
    println!("4. CHECKSUMS");
    println!("------------");
    let dealing_pk_checksum = compute_checksum(&dealing_pk_path)?;
    let dealing_vk_checksum = compute_checksum(&dealing_vk_path)?;
    let reveal_pk_checksum = compute_checksum(&reveal_pk_path)?;
    let reveal_vk_checksum = compute_checksum(&reveal_vk_path)?;
    
    println!("Dealing PK: {}", dealing_pk_checksum);
    println!("Dealing VK: {}", dealing_vk_checksum);
    println!("Reveal PK: {}", reveal_pk_checksum);
    println!("Reveal VK: {}", reveal_vk_checksum);
    println!();
    
    let checksums_path = keys_dir.join("CHECKSUMS.txt");
    let mut checksums_file = fs::File::create(&checksums_path)?;
    writeln!(checksums_file, "SHA256 Checksums for Linera Poker Keys")?;
    writeln!(checksums_file, "Generated: {}", chrono::Utc::now())?;
    writeln!(checksums_file)?;
    writeln!(checksums_file, "dealing.pk: {}", dealing_pk_checksum)?;
    writeln!(checksums_file, "dealing.vk: {}", dealing_vk_checksum)?;
    writeln!(checksums_file, "reveal.pk: {}", reveal_pk_checksum)?;
    writeln!(checksums_file, "reveal.vk: {}", reveal_vk_checksum)?;
    
    println!("SETUP COMPLETE!");
    println!("Generated keys in keys/ directory");
    println!("See keys/README.md for documentation");
    println!();
    
    Ok(())
}
