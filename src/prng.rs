use crate::{GeneratorType, SolverType};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Create deterministic PRNG for a specific instance
pub fn create_instance_prng(
    master_seed: u64,
    generator: GeneratorType,
    solver: SolverType,
    instance_id: u64,
) -> Xoshiro256PlusPlus {
    let mut hasher = DefaultHasher::new();
    master_seed.hash(&mut hasher);
    generator.hash(&mut hasher);
    solver.hash(&mut hasher);
    instance_id.hash(&mut hasher);
    
    let hash1 = hasher.finish();
    
    // Generate more entropy for full 256-bit seed
    hasher.write_u64(hash1);
    let hash2 = hasher.finish();
    hasher.write_u64(hash2);
    let hash3 = hasher.finish();
    hasher.write_u64(hash3);
    let hash4 = hasher.finish();
    
    let mut bytes = [0u8; 32];
    bytes[0..8].copy_from_slice(&hash1.to_le_bytes());
    bytes[8..16].copy_from_slice(&hash2.to_le_bytes());
    bytes[16..24].copy_from_slice(&hash3.to_le_bytes());
    bytes[24..32].copy_from_slice(&hash4.to_le_bytes());
    Xoshiro256PlusPlus::from_seed(bytes)
}